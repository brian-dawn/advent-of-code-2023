use std::error::Error;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq)]
struct Direction {
    value: String,

    left: String,
    right: String,
}

fn parse_direction(line: &str) -> Option<Direction> {
    let re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").ok()?;

    let caps = re.captures(line)?;

    let value = caps.get(1)?.as_str().to_string();
    let left = caps.get(2)?.as_str().to_string();
    let right = caps.get(3)?.as_str().to_string();

    Some(Direction { value, left, right })
}

fn walk(guide: &str, directions: &[Direction]) -> Option<u64> {
    // Convert directions to a hashmap.
    let directions = directions
        .iter()
        .map(|d| (&d.value, (&d.left, &d.right)))
        .collect::<std::collections::HashMap<_, _>>();

    let mut current = &"AAA".to_string();
    for (index, dir) in guide.chars().cycle().enumerate() {
        if current == "ZZZ" {
            return Some(index as u64);
        }

        let (left, right) = directions.get(&current)?;

        if dir == 'L' {
            current = left;
        } else if dir == 'R' {
            current = right;
        } else {
            return None;
        }
    }

    unreachable!()
}

fn walk2(guide: &str, directions: &[Direction]) -> Option<u64> {
    // Convert directions to a hashmap.
    let directions = directions
        .iter()
        .map(|d| (&d.value, (&d.left, &d.right)))
        .collect::<std::collections::HashMap<_, _>>();

    // Walkers are any that end in A.
    let mut walkers = directions
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| Some(k))
        .collect_vec();

    let mut path_lengths = vec![];

    for (index, dir) in guide.chars().cycle().enumerate() {
        // If all the walkers are done we are done.
        if walkers.iter().all(|w| w.is_none()) {
            break;
        }

        for walker in walkers.iter_mut() {
            // If the waler ends in Z we are done with it, delete it from the vector.
            if let Some(real_walker) = walker {
                if real_walker.ends_with('Z') {
                    *walker = None;
                    path_lengths.push(index as u64);
                    continue;
                }

                let (left, right) = directions.get(*real_walker)?;

                if dir == 'L' {
                    *real_walker = left;
                } else if dir == 'R' {
                    *real_walker = right;
                } else {
                    return None;
                }
            }
        }
    }

    // Now we can find the least common multiple of all the path lengths since they are loops.
    lcmx::lcmx(&path_lengths)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = common::read_data("day08")?;

    let mut lines = input.into_iter();

    let guide = lines.next().ok_or("No first line")?;
    let directions = lines
        .map(|line| parse_direction(&line))
        .collect::<Option<Vec<_>>>()
        .ok_or("Failed to parse directions")?;

    let part1 = walk(&guide, &directions).ok_or("Failed to walk")?;
    println!("part1: {}", part1);

    let part2 = walk2(&guide, &directions).ok_or("Failed to walk")?;
    println!("part2: {}", part2);

    Ok(())
}

#[test]
fn test_parse_diirection() {
    let example = "AAA = (BBB, CCC)";
    let expected = Direction {
        value: "AAA".to_string(),
        left: "BBB".to_string(),
        right: "CCC".to_string(),
    };
    let actual = parse_direction(example).unwrap();
    assert_eq!(expected, actual);
}
