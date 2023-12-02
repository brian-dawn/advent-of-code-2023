#[derive(Debug, Eq, PartialEq)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    hands: Vec<Hand>,
}

// Given "2 red, 2 green" or "1 red, 1 green, 2 blue"
// Return a struct representing the hand.
fn parse_hand(hand_line: &str) -> Option<Hand> {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for part in hand_line.split(", ") {
        let mut parts = part.split_whitespace();
        let number = parts.next()?.parse::<u32>().ok()?;
        let color = parts.next()?;

        match color {
            "red" => red += number,
            "green" => green += number,
            "blue" => blue += number,
            _ => return None,
        }
    }

    Some(Hand { red, green, blue })
}

#[test]
fn test_parse_hand() {
    let hand = parse_hand("2 red, 2 green").unwrap();
    let expected = Hand {
        red: 2,
        green: 2,
        blue: 0,
    };
    assert_eq!(hand, expected);

    let hand = parse_hand("1 red, 1 green, 2 blue").unwrap();
    let expected = Hand {
        red: 1,
        green: 1,
        blue: 2,
    };
    assert_eq!(hand, expected);
}

/// Given
/// Game 1: 2 red, 2 green; 1 red, 1 green, 2 blue; 3 blue, 3 red, 3 green; 1 blue, 3 green, 7 red; 5 red, 3 green, 1 blue
/// Return a struct representing the game.
fn parse_game(line: &str) -> Option<Game> {
    let id = line
        .split(":")
        .next()?
        .split_whitespace()
        .nth(1)?
        .parse::<u32>()
        .ok()?;

    let hands = line
        .split(":")
        .nth(1)?
        .split(";")
        .map(|hand| {
            let hand = hand.trim();
            parse_hand(hand)
        })
        .flatten()
        .collect::<Vec<_>>();

    Some(Game { id, hands })
}

#[test]
fn test_parse_game() {
    let game = parse_game("Game 1: 2 red, 2 green; 1 red, 1 green, 2 blue; 3 blue, 3 red, 3 green")
        .unwrap();

    let expected = Game {
        id: 1,
        hands: vec![
            Hand {
                red: 2,
                green: 2,
                blue: 0,
            },
            Hand {
                red: 1,
                green: 1,
                blue: 2,
            },
            Hand {
                red: 3,
                green: 3,
                blue: 3,
            },
        ],
    };

    assert_eq!(game, expected);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = common::read_data("day02")?;

    let part1 = input
        .iter()
        .filter_map(|line| parse_game(line))
        .filter(|game| {
            let all_red_less_than = game.hands.iter().all(|hand| hand.red <= 12);
            let all_green_less_than = game.hands.iter().all(|hand| hand.green <= 13);
            let all_blue_less_than = game.hands.iter().all(|hand| hand.blue <= 14);
            all_red_less_than && all_green_less_than && all_blue_less_than
        })
        .map(|game| game.id)
        .sum::<u32>();

    println!("Part 1: {}", part1);


    let part2 = input.iter().filter_map(|line| parse_game(line)).map(|game| {
        // Find the max of each red, green, blue
        let max_red = game.hands.iter().map(|hand| hand.red).max().unwrap_or(0);
        let max_green = game.hands.iter().map(|hand| hand.green).max().unwrap_or(0);
        let max_blue = game.hands.iter().map(|hand| hand.blue).max().unwrap_or(0);

        max_red * max_green * max_blue
    }).sum::<u32>();

    println!("Part 2: {}", part2);

    Ok(())
}
