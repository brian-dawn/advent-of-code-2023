use std::{error::Error, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Races {
    time: Vec<u64>,
    distance: Vec<u64>,
}

impl FromStr for Races {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Races, Self::Err> {
        let mut lines = input.trim().lines();

        let time = lines
            .next()
            .ok_or("No time line")?
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse().ok())
            .collect();

        let distance = lines
            .next()
            .ok_or("No distance line")?
            .split_whitespace()
            .skip(1)
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Races { time, distance })
    }
}

#[test]
fn test_from_str() {
    let input = r#"
        time: 1 2 3 4 5
        distance: 1 2 3 4 5
    "#;

    let expected = Races {
        time: vec![1, 2, 3, 4, 5],
        distance: vec![1, 2, 3, 4, 5],
    };

    let actual = input.parse::<Races>().unwrap();
    assert_eq!(expected, actual);
}

fn wins_against(race_time: u64, distance_to_beat: u64, hold_time: u64) -> bool {
    // For each time unit of hold time we increase our velocity by 1 distance unit / 1 time unit

    let available_time = race_time - hold_time;
    let distance_covered = available_time * hold_time;

    distance_covered > distance_to_beat
}

#[test]
fn test_wins_against() {
    assert!(!wins_against(7, 9, 0));
    assert!(!wins_against(7, 9, 1));
    assert!(wins_against(7, 9, 2));
    assert!(wins_against(7, 9, 3));
    assert!(wins_against(7, 9, 4));
    assert!(wins_against(7, 9, 5));
    assert!(!wins_against(7, 9, 6));
}

fn possible_ways_to_win(race_time: u64, distance_to_beat: u64) -> usize {
    (0..race_time)
        .skip_while(|hold_time| !wins_against(race_time, distance_to_beat, *hold_time))
        .take_while(|hold_time| wins_against(race_time, distance_to_beat, *hold_time))
        .count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("input/day06.txt")?;

    let races = input.parse::<Races>()?;

    let part1 = races
        .time
        .iter()
        .zip(races.distance.iter())
        .map(|(t, d)| possible_ways_to_win(*t, *d))
        .product::<usize>();

    println!("Part 1: {}", part1);

    let updated_input = input.replace(' ', "").replace(':', " ");
    let race = updated_input.parse::<Races>()?;
    let part2 = possible_ways_to_win(race.time[0], race.distance[0]);

    println!("Part 2: {}", part2);

    Ok(())
}
