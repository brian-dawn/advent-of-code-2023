use std::{cmp::max, error::Error, str::FromStr};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl FromStr for Card {
    type Err = Box<dyn Error>;

    fn from_str(input: &str) -> Result<Card, Self::Err> {
        match input {
            "A" => Ok(Card::A),
            "K" => Ok(Card::K),
            "Q" => Ok(Card::Q),
            "J" => Ok(Card::J),
            "T" => Ok(Card::T),
            "9" => Ok(Card::Nine),
            "8" => Ok(Card::Eight),
            "7" => Ok(Card::Seven),
            "6" => Ok(Card::Six),
            "5" => Ok(Card::Five),
            "4" => Ok(Card::Four),
            "3" => Ok(Card::Three),
            "2" => Ok(Card::Two),
            _ => Err(format!("Invalid card: {}", input).into()),
        }
    }
}

fn parse_hand(input: &str) -> Result<Vec<Card>, Box<dyn Error>> {
    input
        .trim()
        .chars()
        .map(|s| s.to_string().parse::<Card>())
        .collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_to_type(hand: &[Card]) -> HandType {
    let mut hand = hand.to_vec();
    hand.sort();

    let groups = hand
        .iter()
        .group_by(|&element| element)
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    match groups.len() {
        1 => HandType::FiveOfAKind,
        2 => {
            if groups[0].len() == 2 || groups[0].len() == 3 {
                HandType::FullHouse
            } else {
                HandType::FourOfAKind
            }
        }
        3 => {
            if groups[0].len() == 3 || groups[1].len() == 3 || groups[2].len() == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        4 => HandType::OnePair,
        5 => HandType::HighCard,
        _ => unreachable!(),
    }
}

fn hand_to_type_joker(hand: &[Card]) -> HandType {
    let cards = [
        Card::A,
        Card::K,
        Card::Q,
        Card::T,
        Card::Nine,
        Card::Eight,
        Card::Seven,
        Card::Six,
        Card::Five,
        Card::Four,
        Card::Three,
        Card::Two,
    ];

    let mut highest = HandType::HighCard;

    for card in cards {
        // Replace J with card.
        let updated_hand = hand
            .iter()
            .map(|&c| if c == Card::J { card } else { c })
            .collect::<Vec<_>>();

        highest = max(hand_to_type(&updated_hand), highest);
    }

    highest
}

#[test]
fn test_hand_to_type_joker() {
    let hand = vec![Card::J, Card::J, Card::J, Card::J, Card::J];
    assert_eq!(hand_to_type_joker(&hand), HandType::FiveOfAKind);

    let hand = vec![Card::J, Card::J, Card::J, Card::J, Card::A];
    assert_eq!(hand_to_type_joker(&hand), HandType::FiveOfAKind);

    let hand = vec![Card::J, Card::J, Card::K, Card::A, Card::A];
    assert_eq!(hand_to_type_joker(&hand), HandType::FourOfAKind);

    let hand = vec![Card::A, Card::A, Card::A, Card::A, Card::A];
    assert_eq!(hand_to_type_joker(&hand), HandType::FiveOfAKind);

    let hand = vec![Card::J, Card::J, Card::J, Card::J, Card::K];
    assert_eq!(hand_to_type_joker(&hand), HandType::FiveOfAKind);

    let hand = vec![Card::J, Card::J, Card::J, Card::K, Card::K];
    assert_eq!(hand_to_type_joker(&hand), HandType::FiveOfAKind);
}

fn compare_hands(joker_mode: bool, hand1: &[Card], hand2: &[Card]) -> std::cmp::Ordering {
    let hand1_result = if joker_mode {
        hand_to_type_joker(hand1)
    } else {
        hand_to_type(hand1)
    };

    let hand2_result = if joker_mode {
        hand_to_type_joker(hand2)
    } else {
        hand_to_type(hand2)
    };

    if hand1_result > hand2_result {
        return std::cmp::Ordering::Greater;
    } else if hand1_result < hand2_result {
        return std::cmp::Ordering::Less;
    }

    (0..5)
        .find_map(|i| {
            let card1 = hand1[i];
            let card2 = hand2[i];

            if card1 == card2 {
                return None;
            }

            // Joker mode makes jokers always less than anything.
            if joker_mode {
                if card1 == Card::J {
                    return Some(std::cmp::Ordering::Less);
                }

                if card2 == Card::J {
                    return Some(std::cmp::Ordering::Greater);
                }
            }

            if card1 < card2 {
                Some(std::cmp::Ordering::Greater)
            } else if card1 > card2 {
                Some(std::cmp::Ordering::Less)
            } else {
                None
            }
        })
        .unwrap_or(std::cmp::Ordering::Equal)
}

#[test]
fn test_compare_hands() {
    let hand1 = vec![Card::T, Card::Five, Card::Five, Card::J, Card::Five];
    let hand2 = vec![Card::Q, Card::Q, Card::Q, Card::J, Card::A];

    assert!(compare_hands(false, &hand1, &hand2) == std::cmp::Ordering::Less);

    let hand1 = vec![Card::J, Card::J, Card::J, Card::J, Card::Five];
    let hand2 = vec![Card::J, Card::J, Card::J, Card::J, Card::Four];

    assert!(compare_hands(false, &hand1, &hand2) == std::cmp::Ordering::Greater);

    let hand1 = vec![Card::A, Card::A, Card::A, Card::A, Card::A];
    let hand2 = vec![Card::J, Card::J, Card::J, Card::J, Card::J];

    assert!(compare_hands(false, &hand1, &hand2) == std::cmp::Ordering::Greater);

    let hand1 = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    let hand2 = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    assert_eq!(
        compare_hands(false, &hand1, &hand2),
        std::cmp::Ordering::Equal
    );

    let hand1 = vec![Card::A, Card::K, Card::Q, Card::J, Card::Nine];
    let hand2 = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    assert_eq!(
        compare_hands(false, &hand1, &hand2),
        std::cmp::Ordering::Less
    );

    let hand1 = vec![Card::A, Card::J, Card::T, Card::Nine, Card::Eight];
    let hand2 = vec![Card::A, Card::Q, Card::T, Card::Nine, Card::Eight];
    assert_eq!(
        compare_hands(false, &hand1, &hand2),
        std::cmp::Ordering::Less
    );

    let hand1 = vec![Card::A, Card::A, Card::Q, Card::J, Card::T];
    let hand2 = vec![Card::Q, Card::Q, Card::J, Card::T, Card::Nine];
    assert_eq!(
        compare_hands(false, &hand1, &hand2),
        std::cmp::Ordering::Greater
    );

    let hand1 = vec![Card::Five, Card::Four, Card::Three, Card::Two, Card::A];
    let hand2 = vec![Card::Two, Card::Three, Card::Four, Card::Five, Card::A];
    assert_eq!(
        compare_hands(false, &hand1, &hand2),
        std::cmp::Ordering::Greater
    );

    let hand1 = vec![Card::Two, Card::Three, Card::Four, Card::Five, Card::Six];
    let hand2 = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    assert_eq!(
        compare_hands(false, &hand1, &hand2),
        std::cmp::Ordering::Less
    );

    let hand1 = vec![Card::A, Card::A, Card::A, Card::A, Card::K];
    let hand2 = vec![Card::K, Card::K, Card::K, Card::Q, Card::Q];
    assert_eq!(
        compare_hands(false, &hand1, &hand2),
        std::cmp::Ordering::Greater
    );
}

fn parse_hand_with_bid(line: &str) -> Result<(u64, Vec<Card>), Box<dyn Error>> {
    let mut parts = line.split_whitespace();

    let hand_str = parts.next().ok_or("failed to parse hand")?;
    let bid_str = parts.next().ok_or("failed to parse bid")?;

    let hand = parse_hand(hand_str)?;
    let bid = bid_str.parse::<u64>()?;

    Ok((bid, hand))
}

fn main() -> Result<(), Box<dyn Error>> {
    let lines = common::read_data("day07")?;

    let hands_with_bids = lines
        .iter()
        .map(|line| parse_hand_with_bid(line))
        .collect::<Result<Vec<_>, _>>()?;
    // Sort the hands by strength
    let mut hands_with_bids = hands_with_bids;
    hands_with_bids.sort_by(|(_, hand1), (_, hand2)| compare_hands(false, hand1, hand2));

    let part1 = hands_with_bids
        .iter()
        .enumerate()
        .map(|(i, (bid, _))| {
            let rank = i + 1;
            bid * rank as u64
        })
        .sum::<u64>();

    println!("Part 1: {}", part1);

    hands_with_bids.sort_by(|(_, hand1), (_, hand2)| compare_hands(true, hand1, hand2));

    let part1 = hands_with_bids
        .iter()
        .enumerate()
        .map(|(i, (bid, _))| {
            let rank = i + 1;
            bid * rank as u64
        })
        .sum::<u64>();

    println!("Part 2: {}", part1);

    Ok(())
}
