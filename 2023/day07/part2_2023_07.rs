// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2023 Day 7 Part 2

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
#[repr(u8)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    T = 10,
    Num9 = 9,
    Num8 = 8,
    Num7 = 7,
    Num6 = 6,
    Num5 = 5,
    Num4 = 4,
    Num3 = 3,
    Num2 = 2,
    J = 1,
}

#[derive(Debug)]
struct InvalidCardError;

impl TryFrom<char> for Card {
    type Error = InvalidCardError;
    fn try_from(val: char) -> Result<Self, Self::Error> {
        match val {
            'A' => Ok(Card::A),
            'K' => Ok(Card::K),
            'Q' => Ok(Card::Q),
            'J' => Ok(Card::J),
            'T' => Ok(Card::T),
            '9' => Ok(Card::Num9),
            '8' => Ok(Card::Num8),
            '7' => Ok(Card::Num7),
            '6' => Ok(Card::Num6),
            '5' => Ok(Card::Num5),
            '4' => Ok(Card::Num4),
            '3' => Ok(Card::Num3),
            '2' => Ok(Card::Num2),
            _ => Err(InvalidCardError),
        }
    }
}

impl Card {
    fn as_char(&self) -> char {
        match self {
            Card::A => 'A',
            Card::K => 'K',
            Card::Q => 'Q',
            Card::J => 'J',
            Card::T => 'T',
            Card::Num9 => '9',
            Card::Num8 => '8',
            Card::Num7 => '7',
            Card::Num6 => '6',
            Card::Num5 => '5',
            Card::Num4 => '4',
            Card::Num3 => '3',
            Card::Num2 => '2',
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
#[repr(u8)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Hand(HandType, [Card; 5]);

#[derive(Debug)]
struct UncategorizableHandError;

#[derive(Debug)]
enum InvalidHandStrError {
    InvalidCard,
    BadHandSize,
    Uncategorizable,
}

impl From<UncategorizableHandError> for InvalidHandStrError {
    fn from(_e: UncategorizableHandError) -> Self {
        Self::Uncategorizable
    }
}

impl From<InvalidCardError> for InvalidHandStrError {
    fn from(_e: InvalidCardError) -> Self {
        Self::InvalidCard
    }
}

fn find_ideal(joker_count: u8, counts: &[u8]) -> HandType {
    if joker_count == 5 {
        HandType::FiveOfAKind
    } else if joker_count == 0 {
        match counts {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    } else {
        let mut possible_moves: Vec<Vec<u8>> = Vec::new();
        for (i, _) in counts.iter().enumerate() {
            let mut v = counts.to_owned();
            v[i] += 1;
            v.sort_by(|a, b| b.cmp(a));
            possible_moves.push(v);
        }
        // counting as a unique card will be less than adding to an existing set, so no need to
        // check that possibility, unless it's all jokers - a special case handled already.
        possible_moves
            .into_iter()
            .map(|c| find_ideal(joker_count - 1, &c))
            .max()
            .expect("No possible moves")
    }
}

impl Hand {
    fn categorize(cards: [Card; 5]) -> Self {
        let mut counts: HashMap<Card, u8> = HashMap::new();

        let (normal, jokers): (Vec<Card>, Vec<Card>) = cards.iter().partition(|c| *c != &Card::J);

        normal.into_iter().for_each(|c| {
            counts.entry(c).and_modify(|i| *i += 1).or_insert(1);
        });

        let mut counts: Vec<u8> = counts.into_values().collect();
        counts.sort_by(|a, b| b.cmp(a));
        Hand(find_ideal(jokers.len() as u8, &counts), cards)
    }
}

impl FromStr for Hand {
    type Err = InvalidHandStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = s
            .chars()
            .map(|c| Card::try_from(c))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| InvalidHandStrError::BadHandSize)?;
        Ok(Hand::categorize(cards))
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            &self.1.iter().map(|card| card.as_char()).collect::<String>()
        )
    }
}

fn main() {
    let mut bids: Vec<(Hand, u64)> = Vec::new();
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    for line in input.lines() {
        let (hand, bid) = line.split_once(' ').expect("Line is missing delimiter");
        let hand = Hand::from_str(hand).expect("Invalid hand");
        let bid = bid.parse::<u64>().expect("Failed to parse bid as number");
        bids.push((hand, bid));
    }
    bids.sort();

    println!(
        "{}",
        bids.iter()
            .enumerate()
            .map(|(i, (_hand, bid))| bid * (i + 1) as u64)
            .sum::<u64>()
    );
}
