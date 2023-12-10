use std::{collections::HashSet, error::Error};

#[derive(Debug, PartialEq, Eq)]
enum Pipe {
    Vertical,
    Horizontal,
    NE90,
    NW90,
    SE90,
    SW90,
    Ground,
    Start,
}

impl TryFrom<char> for Pipe {
    type Error = Box<dyn Error>;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NE90),
            'J' => Ok(Pipe::NW90),
            '7' => Ok(Pipe::SW90),
            'F' => Ok(Pipe::SE90),
            '.' => Ok(Pipe::Ground),
            'S' => Ok(Pipe::Start),
            _ => Err(format!("Unknown pipe: {}", c))?,
        }
    }
}

struct Map {
    pipes: Vec<Pipe>,
    width: usize,
    height: usize,
}

impl Map {
    fn get(&self, x: usize, y: usize) -> Option<&Pipe> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.pipes.get(y * self.width + x)
    }

    fn parse(input: &str) -> Result<Map, Box<dyn Error>> {
        let mut pipes = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for line in input.trim().lines() {
            height += 1;
            width = 0;

            for c in line.chars() {
                width += 1;
                pipes.push(Pipe::try_from(c)?);
            }
        }

        Ok(Map {
            pipes,
            width,
            height,
        })
    }

    fn find_start(&self) -> Option<(usize, usize)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Pipe::Start) = self.get(x, y) {
                    return Some((x, y));
                }
            }
        }

        None
    }


    // use tree
    fn walk_loop(&self, start: (usize, usize)) -> Option<Vec<(usize, usize)>> {
        let mut to_visit = vec![(start, vec![start])];

        while !to_visit.is_empty() {
            let ((x, y), path) = to_visit.pop()?;

            let Some(pipe) = self.get(x, y) else {
                continue;
            };

            let mut possibilities = HashSet::new();
            match pipe {
                Pipe::Start => {
                    if path.len() > 1 {
                        return Some(path);
                    }

                    // We have begun, fan out in all directions.
                    // TODO: But only if it's a valid symbol to come from.
                    // hard code for now lmao.
                    //possibilities.insert((x - 1, y));
                    //possibilities.insert((x + 1, y));
                    possibilities.insert((x, y - 1));
                    possibilities.insert((x, y + 1));
                }
                Pipe::Vertical => {
                    possibilities.insert((x, y - 1));
                    possibilities.insert((x, y + 1));
                }
                Pipe::Horizontal => {
                    possibilities.insert((x - 1, y));
                    possibilities.insert((x + 1, y));
                }
                Pipe::NE90 => {
                    possibilities.insert((x + 1, y));
                    possibilities.insert((x, y - 1));
                }
                Pipe::NW90 => {
                    possibilities.insert((x - 1, y));
                    possibilities.insert((x, y - 1));
                }
                Pipe::SW90 => {
                    possibilities.insert((x - 1, y));
                    possibilities.insert((x, y + 1));
                }
                Pipe::SE90 => {
                    possibilities.insert((x + 1, y));
                    possibilities.insert((x, y + 1));
                }
                Pipe::Ground => {
                    // Nothing
                }
            }

            for p in possibilities {
                // Don't immediately see start as a valid path.
                let initialized = p == start && path.len() > 2;
                if path.contains(&p) && !initialized {
                    continue;
                }

                let mut updated_path = path.clone();
                updated_path.push(p);

                to_visit.push((p, updated_path));
            }
        }

        None
    }
}

#[test]
fn test_map() {
    let input = "
.....
.....
.S-7.
.|.|.
.L-J.
.....";

    let map = Map::parse(input).unwrap();
    let start = map.find_start().unwrap();

    assert_eq!(start, (1, 2));

    assert_eq!(map.width, 5);
    assert_eq!(map.height, 6);

    assert_eq!(map.get(0, 0), Some(&Pipe::Ground));
    assert_eq!(map.get(1, 0), Some(&Pipe::Ground));
    assert_eq!(map.get(1, 2), Some(&Pipe::Start));
    assert_eq!(map.get(2, 2), Some(&Pipe::Horizontal));
    assert_eq!(map.get(3, 2), Some(&Pipe::SW90));

    let mut path = map.walk_loop(start).unwrap();
    let mut expected = vec![
        (1, 3),
        (1, 4),
        (2, 4),
        (3, 4),
        (3, 3),
        (3, 2),
        (2, 2),
        (1, 2),
    ];
    assert_eq!(path, expected);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input/day10.txt")?;

    let map = Map::parse(&input)?;
    let start = map.find_start().ok_or("No start found")?;
    let path = map.walk_loop(start).ok_or("No path found")?;

    let part1 = path.len() / 2;
    println!("Part 1: {}", part1);

    Ok(())
}
