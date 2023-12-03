#[derive(Debug)]
struct Schematic {
    data: Vec<Vec<char>>,
}

fn is_symbol(c: char) -> bool {
    !c.is_numeric() && c != '.'
}

#[test]
fn test_is_symbol() {
    assert!(!is_symbol('9'));
    assert!(!is_symbol('.'));
    assert!(is_symbol('A'));
    assert!(is_symbol('h'));
}

impl Schematic {
    fn is_adjacent_to_symbol(&self, row: usize, col: usize) -> bool {
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

            if is_symbol(c) {
                return true;
            }
        }

        false
    }
}

fn part1(schematic: &Schematic) -> u32 {
    let mut sum = 0;

    for (i, line) in schematic.data.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            let mut num = 0;
            let mut next_to_symbol = false;

            while let Some(digit) = line.get(col).and_then(|c| c.to_digit(10)) {
                next_to_symbol = next_to_symbol || schematic.is_adjacent_to_symbol(i, col);

                num *= 10;
                num += digit;
                col += 1;
            }

            if next_to_symbol {
                sum += num;
            }
            col += 1;
        }
    }
    sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = common::read_data("day03")?;

    let schematic = Schematic {
        data: input.iter().map(|line| line.chars().collect()).collect(),
    };

    let part1 = part1(&schematic);
    println!("Part 1: {}", part1);

    Ok(())
}
