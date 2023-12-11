use itertools::Itertools;
use std::error::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Space {
    Empty,
    Galaxy(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    data: Vec<Space>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from_lines(lines: Vec<String>) -> Grid {
        let mut grid = Grid {
            data: Vec::new(),
            width: 0,
            height: 0,
        };

        for line in lines {
            grid.width = line.len();
            grid.height += 1;

            for c in line.chars() {
                match c {
                    '.' => grid.data.push(Space::Empty),
                    '#' => grid.data.push(Space::Galaxy(0)),
                    _ => panic!("Invalid input"),
                }
            }
        }

        grid
    }

    fn get(&self, x: usize, y: usize) -> Option<&Space> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.data[y * self.width + x])
        }
    }

    fn insert_empty_row(&mut self, row: usize) {
        self.height += 1;

        for x in 0..self.width {
            self.data.insert(row * self.width + x, Space::Empty);
        }
    }

    fn insert_empty_column(&mut self, column: usize) {
        self.width += 1;

        for y in 0..self.height {
            self.data.insert(y * self.width + column, Space::Empty);
        }
    }

    fn pretty_print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y).unwrap() {
                    Space::Empty => print!("."),
                    Space::Galaxy(_) => print!("#"),
                }
            }
            println!();
        }
    }

    fn expand(&self) -> Grid {
        // Any rows or columns without galaxies are twice as big.

        let mut new_grid = self.clone();

        // Work backwards to avoid indexing issues.
        for y in (0..self.height).rev() {
            let mut has_galaxy = false;
            for x in 0..self.width {
                if let Space::Galaxy(_) = self.get(x, y).expect("Invalid index") {
                    has_galaxy = true;
                    break;
                }
            }

            if !has_galaxy {
                new_grid.insert_empty_row(y);
            }
        }

        // Now expand columns.
        for x in (0..self.width).rev() {
            let mut has_galaxy = false;
            for y in 0..self.height {
                if let Space::Galaxy(_) = self.get(x, y).expect("Invalid index") {
                    has_galaxy = true;
                    break;
                }
            }

            if !has_galaxy {
                new_grid.insert_empty_column(x);
            }
        }

        new_grid
    }
}

#[test]
fn test_expand() {
    let mut grid = Grid::from_lines(vec![
        "###".to_string(),
        "###".to_string(),
        "###".to_string(),
    ]);

    grid.insert_empty_row(1);

    let expected = Grid::from_lines(vec![
        "###".to_string(),
        "...".to_string(),
        "###".to_string(),
        "###".to_string(),
    ]);

    assert_eq!(grid, expected);
    assert_eq!(grid.width, 3);
    assert_eq!(grid.height, 4);

    grid.insert_empty_column(1);

    let expected = Grid::from_lines(vec![
        "#.##".to_string(),
        "....".to_string(),
        "#.##".to_string(),
        "#.##".to_string(),
    ]);

    assert_eq!(grid, expected);
    assert_eq!(grid.width, 4);
    assert_eq!(grid.height, 4);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = common::read_data("day11")?;

    let grid = Grid::from_lines(input);

    let expanded = grid.expand();
    expanded.pretty_print();

    // For each pair of galaxies find the manhattan distance between them.
    let galaxy_coordinates = expanded
        .data
        .iter()
        .enumerate()
        .filter_map(|(i, space)| match space {
            Space::Galaxy(_) => Some((i % expanded.width, i / expanded.width)),
            _ => None,
        })
        .collect::<Vec<_>>();

    let part1 = galaxy_coordinates
        .iter()
        .combinations(2)
        .map(|pair| {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];

            let dx = if x1 > x2 { x1 - x2 } else { x2 - x1 };
            let dy = if y1 > y2 { y1 - y2 } else { y2 - y1 };

            dx + dy
        })
        .sum::<usize>();

    println!("Part 1: {}", part1);

    Ok(())
}
