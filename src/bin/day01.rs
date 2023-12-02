fn parse_spelled_numbers(input: &str) -> Vec<u32> {
    let number_words = [
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];

    let mut out = vec![];

    for i in 0..input.len() {
        for (word, number) in number_words {
            if input[i..].starts_with(word) {
                out.push(number);
                break;
            }
        }
    }

    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = common::read_data("day01")?;

    let answer_1: u32 = input
        .iter()
        .map(|line| {
            let digits = line.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>();

            let first_digit = digits.first()?;
            let last_digit = digits.last()?;

            Some(first_digit.to_digit(10)? * 10 + last_digit.to_digit(10)?)
        })
        .flatten()
        .sum();

    println!("Answer 1: {}", answer_1);

    let answer_2: u32 = input
        .iter()
        .map(|line| {
            let digits = parse_spelled_numbers(line);

            let first_digit = digits.first()?;
            let last_digit = digits.last()?;

            Some(first_digit * 10 + last_digit)
        })
        .flatten()
        .sum();

    println!("Answer 2: {}", answer_2);
    Ok(())
}
