fn parse_history(line: &str) -> Option<Vec<i64>> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().ok())
        .collect::<Option<Vec<i64>>>()
}

#[test]
fn test_parse_history() {
    let history = parse_history("1 2 3 4 5 6 7 8 9 10").unwrap();
    let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(history, expected);
}

fn derivative(history: &[i64]) -> Vec<i64> {
    let mut out = vec![];

    for i in 0..history.len() - 1 {
        let a = history[i];
        let b = history[i + 1];

        out.push(b - a);
    }

    out
}

#[test]
fn test_derivative() {
    let history = vec![1, 2, 3, 3];
    let expected = vec![1, 1, 0];
    assert_eq!(derivative(&history), expected);
}

fn extrapolate(history: &[i64]) -> Option<Vec<i64>> {
    if history.iter().all(|n| *n == 0) {
        return Some(vec![0]);
    }

    let mut rec = extrapolate(&derivative(history))?;

    let last = history.last()?;
    let last_rec = rec.last()?;

    rec.push(last + last_rec);

    Some(rec)
}

#[test]
fn test_extrapolate() {
    let history = vec![0, 3, 6, 9, 12, 15];

    let extrapolated = extrapolate(&history).unwrap();

    let expected = vec![0, 3, 18];
    assert_eq!(extrapolated, expected);
}

fn extrapolate_backwards(history: &[i64]) -> Option<Vec<i64>> {
    if history.iter().all(|n| *n == 0) {
        return Some(vec![0]);
    }

    let mut rec = extrapolate_backwards(&derivative(history))?;

    let first = history.first()?;
    let first_rec = rec.first()?;

    rec.insert(0, first - first_rec);

    Some(rec)
}

#[test]
fn test_extrapolate_backwards() {
    let history = vec![10, 13, 16, 21, 30, 45];

    let extrapolated = extrapolate_backwards(&history).unwrap();

    let expected = vec![5, 5, -2, 2, 0];
    assert_eq!(extrapolated, expected);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = common::read_data("day09")?;

    let part1 = input
        .iter()
        .filter_map(|line| parse_history(line))
        .filter_map(|history| {
            let extrapolated = extrapolate(&history);
            extrapolated?.last().cloned()
        })
        .sum::<i64>();

    println!("Part 1: {}", part1);

    let part2 = input
        .iter()
        .filter_map(|line| parse_history(line))
        .filter_map(|history| {
            let extrapolated = extrapolate_backwards(&history);
            extrapolated?.first().cloned()
        })
        .sum::<i64>();

    println!("Part 2: {}", part2);

    Ok(())
}
