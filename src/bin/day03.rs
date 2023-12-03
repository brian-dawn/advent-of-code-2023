use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Schematic {
    data: Vec<Vec<char>>,
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

fn is_gear(c: char) -> bool {
    c == '*'
}

#[test]
fn test_is_symbol() {
    assert!(!is_symbol('9'));
    assert!(!is_symbol('.'));
    assert!(is_symbol('A'));
    assert!(is_symbol('h'));
}

impl Schematic {
    /// Returns the coordinates of any predicate fn.
    fn is_adjacent_to_symbol<F>(&self, row: usize, col: usize, f: F) -> Vec<(usize, usize)>
    where
        F: Fn(char) -> bool,
    {
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let x = row as i32;
        let y = col as i32;

        let mut out = vec![];

        for (row_offset, col_offset) in offsets.iter() {
            let adj_row = x + row_offset;
            let adj_col = y + col_offset;

            let Ok(adj_row) = usize::try_from(adj_row) else {
                continue;
            };

            let Ok(adj_col) = usize::try_from(adj_col) else {
                continue;
            };

            let Some(line) = self.data.get(adj_row) else {
                continue;
            };

            let Some(&c) = line.get(adj_col) else {
                continue;
            };

            if f(c) {
                out.push((adj_row, adj_col));
            }
        }

        out
    }
}

type GearRatios = HashMap<(u32, u32), Vec<u32>>;

fn walk(schematic: &Schematic) -> (u32, GearRatios) {
    let mut sum = 0;

    let mut gear_ratios: GearRatios = HashMap::new();

    for (i, line) in schematic.data.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            let mut num = 0;
            let mut next_to_symbol = false;

            let mut gear_coordinates_for_num = HashSet::new();

            while let Some(digit) = line.get(col).and_then(|c| c.to_digit(10)) {
                // Find all the adjacent gears for this digit.
                let gear_coordinates = schematic.is_adjacent_to_symbol(i, col, is_gear);
                gear_coordinates_for_num.extend(gear_coordinates);

                next_to_symbol = next_to_symbol
                    || !schematic
                        .is_adjacent_to_symbol(i, col, is_symbol)
                        .is_empty();

                num *= 10;
                num += digit;
                col += 1;
            }

            if next_to_symbol {
                sum += num;
            }

            // For each gear near this number insert it into the gear ratios hashmap.
            for (row, col) in gear_coordinates_for_num {
                let gear_ratios_for_gear = gear_ratios.entry((row as u32, col as u32)).or_default();
                gear_ratios_for_gear.push(num);
            }

            col += 1;
        }
    }
    (sum, gear_ratios)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = common::read_data("day03")?;

    let schematic = Schematic {
        data: input.iter().map(|line| line.chars().collect()).collect(),
    };

    let (part1, gear_ratios) = walk(&schematic);
    println!("Part 1: {}", part1);


    // For each gear, if there's only two numbers then the ratio is the multiplication of those two numbers.
    let part2 = gear_ratios
        .iter()
        .filter_map(|(gear, numbers)| {
            if numbers.len() == 2 {
                Some(numbers[0] * numbers[1])
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("Part 2: {}", part2);

    Ok(())
}
