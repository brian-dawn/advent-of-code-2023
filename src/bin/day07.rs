use std::{error::Error, str::FromStr};

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

fn five_of_a_kind(hand: &[Card]) -> bool {
    hand.iter().all(|c| c == &hand[0])
}

#[test]
fn test_five_of_a_kind() {
    let cards = vec![Card::A, Card::A, Card::A, Card::A, Card::A];
    assert!(five_of_a_kind(&cards));

    let cards = vec![Card::A, Card::A, Card::A, Card::Q, Card::T];
    assert!(!five_of_a_kind(&cards));
}

fn four_of_a_kind(hand: &[Card]) -> bool {
    hand.windows(4).any(|w| w.iter().all(|c| c == &w[0]))
}

#[test]
fn test_four_of_a_kind() {
    let cards = vec![Card::A, Card::A, Card::A, Card::A, Card::T];
    assert!(four_of_a_kind(&cards));

    let cards = vec![Card::A, Card::A, Card::A, Card::Q, Card::T];
    assert!(!four_of_a_kind(&cards));

    let cards = vec![Card::A, Card::A, Card::A, Card::A, Card::A];
    assert!(four_of_a_kind(&cards));
}

fn full_house(hand: &[Card]) -> bool {
    let mut hand = hand.to_vec();
    hand.sort();

    let groups = hand
        .iter()
        .group_by(|&element| element)
        .into_iter()
        .map(|(_, group)| group.collect::<Vec<_>>())
        .collect::<Vec<_>>();

    groups.len() == 2 && (groups[0].len() == 2 || groups[0].len() == 3)
}

#[test]
fn test_full_house() {
    let cards = vec![Card::A, Card::A, Card::A, Card::Q, Card::Q];
    assert!(full_house(&cards));

    let cards = vec![Card::A, Card::A, Card::Q, Card::Q, Card::T];
    assert!(!full_house(&cards));

    let cards = vec![Card::A, Card::A, Card::A, Card::A, Card::T];
    assert!(!full_house(&cards));

    let cards = vec![Card::A, Card::A, Card::T, Card::T, Card::K];
    assert!(!full_house(&cards));

    let cards = vec![Card::A, Card::A, Card::A, Card::T, Card::T];
    assert!(full_house(&cards));
}

fn three_of_a_kind(hand: &[Card]) -> bool {
    let mut hand = hand.to_vec();
    hand.sort();

    let has_three = hand
        .iter()
        .group_by(|&element| element)
        .into_iter()
        .map(|(key, group)| (key, group.collect::<Vec<_>>()))
        .filter(|(_, group)| group.len() == 3)
        .count()
        == 1;

    let has_two = hand
        .iter()
        .group_by(|&element| element)
        .into_iter()
        .map(|(key, group)| (key, group.collect::<Vec<_>>()))
        .filter(|(_, group)| group.len() == 2)
        .count()
        == 1;

    has_three && !has_two
}

#[test]
fn test_three_of_a_kind() {
    let cards = vec![Card::A, Card::A, Card::A, Card::Q, Card::T];
    assert!(three_of_a_kind(&cards));

    let cards = vec![Card::A, Card::A, Card::Q, Card::J, Card::T];
    assert!(!three_of_a_kind(&cards));

    let cards = vec![Card::A, Card::A, Card::A, Card::A, Card::T];
    assert!(!three_of_a_kind(&cards));

    let cards = vec![Card::A, Card::A, Card::T, Card::T, Card::K];
    assert!(!three_of_a_kind(&cards));

    let cards = vec![Card::A, Card::A, Card::A, Card::T, Card::T];
    assert!(!three_of_a_kind(&cards));
}

fn two_pair(hand: &[Card]) -> bool {
    let mut hand = hand.to_vec();
    hand.sort();

    hand.iter()
        .group_by(|&element| element)
        .into_iter()
        .map(|(key, group)| (key, group.collect::<Vec<_>>()))
        .filter(|(_, group)| group.len() == 2)
        .count()
        == 2
}

#[test]
fn test_two_pair() {
    let cards = vec![Card::A, Card::A, Card::Q, Card::Q, Card::T];
    assert!(two_pair(&cards));

    let cards = vec![Card::A, Card::A, Card::Q, Card::J, Card::T];
    assert!(!two_pair(&cards));

    let cards = vec![Card::A, Card::A, Card::A, Card::Q, Card::T];
    assert!(!two_pair(&cards));

    let cards = vec![Card::A, Card::A, Card::A, Card::Q, Card::Q];
    assert!(!two_pair(&cards));
}

fn one_pair(hand: &[Card]) -> bool {
    let card_set = hand.iter().collect::<std::collections::HashSet<_>>();
    card_set.len() == 4
}

#[test]
fn test_one_pair() {
    let cards = vec![Card::A, Card::A, Card::Q, Card::J, Card::T];
    assert!(one_pair(&cards));

    let cards = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    assert!(!one_pair(&cards));

    let cards = vec![Card::A, Card::A, Card::Q, Card::Q, Card::T];
    assert!(!one_pair(&cards));
}

fn high_card(hand: &[Card]) -> bool {
    let card_set = hand.iter().collect::<std::collections::HashSet<_>>();
    card_set.len() == 5
}

#[test]
fn test_high_card() {
    let cards = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    assert!(high_card(&cards));

    let cards = vec![Card::A, Card::A, Card::Q, Card::J, Card::T];
    assert!(!high_card(&cards));
}

fn compare_hands(hand1: &[Card], hand2: &[Card]) -> std::cmp::Ordering {
    let fns = vec![
        five_of_a_kind,
        four_of_a_kind,
        full_house,
        three_of_a_kind,
        two_pair,
        one_pair,
        high_card,
    ];

    for f in fns {
        let hand1_result = f(&hand1);
        let hand2_result = f(&hand2);

        if hand1_result && !hand2_result {
            return std::cmp::Ordering::Greater;
        }

        if !hand1_result && hand2_result {
            return std::cmp::Ordering::Less;
        }
    }

    (0..5)
        .find_map(|i| {
            let card1 = hand1[i];
            let card2 = hand2[i];

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

    assert!(compare_hands(&hand1, &hand2) == std::cmp::Ordering::Less);

    let hand1 = vec![Card::J, Card::J, Card::J, Card::J, Card::Five];
    let hand2 = vec![Card::J, Card::J, Card::J, Card::J, Card::Four];

    assert!(compare_hands(&hand1, &hand2) == std::cmp::Ordering::Greater);

    let hand1 = vec![Card::A, Card::A, Card::A, Card::A, Card::A];
    let hand2 = vec![Card::J, Card::J, Card::J, Card::J, Card::J];

    assert!(compare_hands(&hand1, &hand2) == std::cmp::Ordering::Greater);

    let hand1 = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    let hand2 = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    assert_eq!(compare_hands(&hand1, &hand2), std::cmp::Ordering::Equal);

    let hand1 = vec![Card::A, Card::K, Card::Q, Card::J, Card::Nine];
    let hand2 = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    assert_eq!(compare_hands(&hand1, &hand2), std::cmp::Ordering::Less);

    let hand1 = vec![Card::A, Card::J, Card::T, Card::Nine, Card::Eight];
    let hand2 = vec![Card::A, Card::Q, Card::T, Card::Nine, Card::Eight];
    assert_eq!(compare_hands(&hand1, &hand2), std::cmp::Ordering::Less);

    let hand1 = vec![Card::A, Card::A, Card::Q, Card::J, Card::T];
    let hand2 = vec![Card::Q, Card::Q, Card::J, Card::T, Card::Nine];
    assert_eq!(compare_hands(&hand1, &hand2), std::cmp::Ordering::Greater);

    let hand1 = vec![Card::Five, Card::Four, Card::Three, Card::Two, Card::A];
    let hand2 = vec![Card::Two, Card::Three, Card::Four, Card::Five, Card::A];
    assert_eq!(compare_hands(&hand1, &hand2), std::cmp::Ordering::Greater);

    let hand1 = vec![Card::Two, Card::Three, Card::Four, Card::Five, Card::Six];
    let hand2 = vec![Card::A, Card::K, Card::Q, Card::J, Card::T];
    assert_eq!(compare_hands(&hand1, &hand2), std::cmp::Ordering::Less);

    let hand1 = vec![Card::A, Card::A, Card::A, Card::A, Card::K];
    let hand2 = vec![Card::K, Card::K, Card::K, Card::Q, Card::Q];
    assert_eq!(compare_hands(&hand1, &hand2), std::cmp::Ordering::Greater);
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
    hands_with_bids.sort_by(|(_, hand1), (_, hand2)| compare_hands(hand1, hand2));

    println!("{:?}", hands_with_bids);
    let part1 = hands_with_bids
        .iter()
        .enumerate()
        .map(|(i, (bid, _))| {
            let rank = i + 1;
            bid * rank as u64
        })
        .sum::<u64>();

    println!("Part 1: {}", part1);

    Ok(())
}
