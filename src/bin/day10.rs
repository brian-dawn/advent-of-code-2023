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

    fn flood_fill(&self, outer_path: &[(usize, usize)]) -> u64 {
        // Find the centeroid by averaging all the coordinates
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum Flood {
            Empty,
            Flooded,
            Blocked,
        }

        let width = self.width * 2;
        let height = self.height * 2;

        let mut flood = vec![Flood::Empty; width * height];

        // Perform flood fill, note that "squeezing between pipes" is allowed.
        // Therefor we will 2x the resolution.

        // Fill in the gaps in the outer path.

        for window in outer_path.windows(2) {
            let (x1, y1) = window[0];
            let (x2, y2) = window[1];

            // fill in x1, y1 as well as the space between x1, y1 and x2, y2
            let avg_x = (x1 * 2 + x2 * 2) / 2;
            let avg_y = (y1 * 2 + y2 * 2) / 2;
            flood[y1 * 2 * width + x1 * 2] = Flood::Blocked;
            flood[avg_y * width + avg_x] = Flood::Blocked;
        }

        // Now we flood from the outside
        let mut to_visit = vec![(0, 0)];
        while !to_visit.is_empty() {
            let Some((x, y)) = to_visit.pop() else {
                continue;
            };

            let mut possibilities = HashSet::new();
            possibilities.insert((x - 1, y));
            possibilities.insert((x + 1, y));
            possibilities.insert((x, y - 1));
            possibilities.insert((x, y + 1));

            for p in possibilities {
                let (x, y) = p;

                if x >= width || y >= height {
                    continue;
                }

                if flood[y * width + x] != Flood::Empty {
                    continue;
                }

                flood[y * width + x] = Flood::Flooded;
                to_visit.push(p);
            }
        }

        // Pretty print..
        for y in 0..height {
            for x in 0..width {
                match flood[y * width + x] {
                    Flood::Empty => print!("."),
                    Flood::Flooded => print!("~"),
                    Flood::Blocked => print!("#"),
                }
            }
            println!();
        }

        let mut total = 0;
        let original_path_set = outer_path.iter().cloned().collect::<HashSet<_>>();

        for y in 0..height {
            for x in 0..width {
                if original_path_set.contains(&(x / 2, y / 2)) {
                    continue;
                }
                if flood[y * width + x] == Flood::Empty {
                    total += 1;
                }
            }
        }

        total / 4
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

    let part2 = map.flood_fill(&path);

    println!("Part 2: {}", part2);

    Ok(())
}
