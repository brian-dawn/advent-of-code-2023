use std::error::Error;

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    scratch_numbers: Vec<u32>,
}

impl Card {
    fn from_str(s: &str) -> Option<Card> {
        // Split the input string into two parts: the ID and the numbers part.
        let mut parts = s.split(": ");
        let id_part = parts.next()?;
        let numbers_part = parts.next()?;

        // Extract the ID from the ID part.
        let id = id_part.split_whitespace().nth(1)?.parse::<u32>().ok()?;

        // Split the numbers part into winning and scratch numbers.
        let mut number_parts = numbers_part.split(" | ");
        let winning_numbers_str = number_parts.next()?;
        let scratch_numbers_str = number_parts.next()?;

        // Parse winning numbers.
        let winning_numbers = winning_numbers_str
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        // Parse scratch numbers.
        let scratch_numbers = scratch_numbers_str
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect::<Vec<u32>>();

        Some(Card {
            id,
            winning_numbers,
            scratch_numbers,
        })
    }

    fn matches(&self) -> usize {
        let winning_set = self
            .winning_numbers
            .iter()
            .collect::<std::collections::HashSet<_>>();
        let scratch_set = self
            .scratch_numbers
            .iter()
            .collect::<std::collections::HashSet<_>>();

        winning_set.intersection(&scratch_set).count()
    }

    fn score(&self) -> u32 {
        let matches = self.matches();
        if matches == 0 {
            return 0;
        }

        2u32.pow(matches as u32 - 1)
    }
}

#[test]
fn test_card_from_str() {
    let card = Card::from_str("Card 1: 1 2 3 | 4 5 6").unwrap();
    assert_eq!(card.id, 1);
    assert_eq!(card.winning_numbers, vec![1, 2, 3]);
    assert_eq!(card.scratch_numbers, vec![4, 5, 6]);
}

#[test]
fn test_score() {
    let card = Card {
        id: 1,
        winning_numbers: vec![1, 2, 3],
        scratch_numbers: vec![1, 2, 3],
    };
    assert_eq!(card.score(), 4);

    // Zero matches.
    let card = Card {
        id: 1,
        winning_numbers: vec![1, 2, 3],
        scratch_numbers: vec![4, 5, 6],
    };
    assert_eq!(card.score(), 0);
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = common::read_data("day04")?;

    let part1 = input
        .iter()
        .filter_map(|l| Card::from_str(l))
        .map(|c| c.score())
        .sum::<u32>();

    println!("Part 1: {}", part1);

    // Now part 2.
    let card_vec = input
        .iter()
        .filter_map(|l| Card::from_str(l))
        .collect::<Vec<Card>>();

    // We start with 1 copy of each card.
    let mut id_to_copies = card_vec
        .iter()
        .map(|c| (c.id, 1))
        .collect::<std::collections::HashMap<u32, u32>>();

    for card in card_vec {
        let matches = card.matches();

        let Some(copies) = id_to_copies.get(&card.id) else {
            break;
        };

        for _ in 0..*copies {
            for i in 0..matches {
                let id = card.id + i as u32 + 1;
                // Add a copy of the card to the deck.
                let Some(copies) = id_to_copies.get(&id) else {
                    break;
                };
                id_to_copies.insert(id, copies + 1);
            }
        }
    }

    let part2 = id_to_copies.values().sum::<u32>();

    println!("Part 2: {}", part2);

    Ok(())
}
