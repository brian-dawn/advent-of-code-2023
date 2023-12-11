use itertools::Itertools;
use std::{collections::HashSet, error::Error};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Galaxy {
    id: usize,
    x: usize,
    y: usize,
}

fn from_lines(input: Vec<String>) -> Vec<Galaxy> {
    let mut galaxies = Vec::new();

    let mut id = 0;
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                id += 1;
                galaxies.push(Galaxy { x, y, id });
            }
        }
    }
    galaxies
}

fn manhattan_distance(
    g1: &Galaxy,
    g2: &Galaxy,
    expansion_rows: &HashSet<&usize>,
    expansion_cols: &HashSet<&usize>,
    expansion_amount: usize,
) -> usize {

    let mut distance = 0;

    // If we cross an empty row or column we expand the distance by 2 instead of 1 as we walk.

    // First walk horizontally
    let mut x = g1.x;

    while x != g2.x {
        if expansion_cols.contains(&x) {
            distance += expansion_amount;
        } else {
            distance += 1;
        }
        if x < g2.x {
            x += 1;
        } else {
            x -= 1;
        }
    }

    // Now walk vertically
    let mut y = g1.y;
    while y != g2.y {
        if expansion_rows.contains(&y) {
            distance += expansion_amount;
        } else {
            distance += 1;
        }
        if y < g2.y {
            y += 1;
        } else {
            y -= 1;
        }
    }

    distance
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = common::read_data("day11")?;

    let galaxies = from_lines(input);


    let max_y = galaxies.iter().map(|g| g.y).max().ok_or("No galaxies")?;
    let max_x = galaxies.iter().map(|g| g.x).max().ok_or("No galaxies")?;

    let y_set: HashSet<usize> = (0..=max_y).collect();
    let x_set: HashSet<usize> = (0..=max_x).collect();

    let galaxy_ys = galaxies.iter().map(|g| g.y).collect::<HashSet<_>>();
    let galaxy_xs = galaxies.iter().map(|g| g.x).collect::<HashSet<_>>();

    let empty_rows = y_set.difference(&galaxy_ys).collect::<HashSet<_>>();
    let empty_cols = x_set.difference(&galaxy_xs).collect::<HashSet<_>>();


    // Now for each pair of galaxies find the manhattan distance, if we cross an empty row or column we expand the distance by 1.
    let part1 = galaxies
        .iter()
        .combinations(2)
        .map(|g| manhattan_distance(&g[0], &g[1], &empty_rows, &empty_cols, 2))
        .sum::<usize>();

    println!("Part 1: {}", part1);

    // Now part 2 we expand by 1 million.
    let part2 = galaxies
        .iter()
        .combinations(2)
        .map(|g| manhattan_distance(&g[0], &g[1], &empty_rows, &empty_cols, 1_000_000))
        .sum::<usize>();

    println!("Part 2: {}", part2);

    Ok(())
}
