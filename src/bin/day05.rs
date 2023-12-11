use itertools::{self, Itertools};
use std::error::Error;

fn parse_paragraph(paragraph: &str) -> Vec<(u64, u64, u64)> {
    let mut lines = paragraph.trim().lines();

    // Skip the first line.
    lines.next();

    lines
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let seed = parts.next()?.parse::<u64>().ok()?;
            let to_soil = parts.next()?.parse::<u64>().ok()?;
            let soil = parts.next()?.parse::<u64>().ok()?;
            Some((seed, to_soil, soil))
        })
        .collect()
}

fn parse_seeds(seeds: &str) -> Vec<u64> {
    seeds
        .split_whitespace()
        .filter_map(|s| s.parse::<u64>().ok())
        .collect()
}

#[test]
fn test_parse_seeds() {
    let seeds = "seeds: 1 2 3 4 5";

    let expected = vec![1, 2, 3, 4, 5];
    let actual = parse_seeds(seeds);
    assert_eq!(expected, actual);
}

#[test]
fn test_parse_paragraph() {
    let paragraph = r#"
        foo
        1 2 3
        2 3 4
        3 4 5
    "#;

    let expected = vec![(1, 2, 3), (2, 3, 4), (3, 4, 5)];
    let actual = parse_paragraph(paragraph);
    assert_eq!(expected, actual);
}

fn mapping(num: u64, to: &[(u64, u64, u64)]) -> u64 {
    // to is dest range start, source range start, range length
    for (dest, source, length) in to {
        if num >= *source && num < source + length {
            return dest + (num - source);
        }
    }

    num
}

#[test]
fn test_mapping() {
    let to = vec![(50, 98, 2), (52, 50, 48)];

    assert_eq!(mapping(98, &to), 50);
    assert_eq!(mapping(99, &to), 51);
}

fn part2(input: &str) -> Option<()> {
    // Split by paragraph.
    let mut paragraphs = input.split("\n\n");

    let seeds = parse_seeds(paragraphs.next()?);
    let seed_to_soil = parse_paragraph(paragraphs.next()?);
    let soil_to_fertilizer = parse_paragraph(paragraphs.next()?);
    let fertilizer_to_water = parse_paragraph(paragraphs.next()?);
    let water_to_light = parse_paragraph(paragraphs.next()?);
    let light_to_temperature = parse_paragraph(paragraphs.next()?);
    let temperature_to_humidity = parse_paragraph(paragraphs.next()?);
    let humidity_to_location = parse_paragraph(paragraphs.next()?);

    let mut lowest = std::u64::MAX;

    let mut seeds_by_pairs = vec![];
    for (seed, range) in seeds.into_iter().tuples() {
        for i in 0..range {
            seeds_by_pairs.push(seed + i);
        }
    }

    for seed in seeds_by_pairs {
        // Walk everything till we get to location.
        let mut num = seed;
        num = mapping(num, &seed_to_soil);
        num = mapping(num, &soil_to_fertilizer);
        num = mapping(num, &fertilizer_to_water);
        num = mapping(num, &water_to_light);
        num = mapping(num, &light_to_temperature);
        num = mapping(num, &temperature_to_humidity);
        num = mapping(num, &humidity_to_location);

        lowest = std::cmp::min(lowest, num);
    }

    println!("Part 2: {}", lowest);

    Some(())
}

fn part1(input: &str) -> Option<()> {
    // Split by paragraph.
    let mut paragraphs = input.split("\n\n");

    let seeds = parse_seeds(paragraphs.next()?);
    let seed_to_soil = parse_paragraph(paragraphs.next()?);
    let soil_to_fertilizer = parse_paragraph(paragraphs.next()?);
    let fertilizer_to_water = parse_paragraph(paragraphs.next()?);
    let water_to_light = parse_paragraph(paragraphs.next()?);
    let light_to_temperature = parse_paragraph(paragraphs.next()?);
    let temperature_to_humidity = parse_paragraph(paragraphs.next()?);
    let humidity_to_location = parse_paragraph(paragraphs.next()?);

    let mut lowest = std::u64::MAX;
    for seed in seeds {
        // Walk everything till we get to location.
        let mut num = seed;
        num = mapping(num, &seed_to_soil);
        num = mapping(num, &soil_to_fertilizer);
        num = mapping(num, &fertilizer_to_water);
        num = mapping(num, &water_to_light);
        num = mapping(num, &light_to_temperature);
        num = mapping(num, &temperature_to_humidity);
        num = mapping(num, &humidity_to_location);

        lowest = std::cmp::min(lowest, num);
    }

    println!("Part 1: {}", lowest);

    Some(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input/day05.txt")?;

    part1(&input);
    part2(&input);
    Ok(())
}
