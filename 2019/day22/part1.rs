// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 22 Part 1
use std::collections::VecDeque;
use std::num::ParseIntError;

struct Deck {
    cards: VecDeque<u16>,
}

impl Deck {
    fn deal_into_new(&mut self) {
        self.cards.make_contiguous().reverse();
    }
    fn cut(&mut self, mut count: isize) {
        count %= isize::try_from(self.cards.len()).expect("length of less than isize::MAX");
        if count.is_negative() {
            self.cards.rotate_right(count.unsigned_abs());
        } else {
            self.cards.rotate_left(count.unsigned_abs());
        }
    }

    fn deal_with_increment(&mut self, count: usize) {
        let mut new_cards: Vec<Option<u16>> = [None].repeat(self.cards.len());
        let mut i = 0;
        while let Some(card) = self.cards.pop_front() {
            assert!(new_cards[i].is_none(), "space already occupied");
            new_cards[i] = Some(card);
            i += count;
            i %= new_cards.len();
        }
        debug_assert!(new_cards.iter().all(|c| c.is_some()), "Empty space left");
        self.cards.extend(new_cards.into_iter().flatten());
    }

    fn apply(&mut self, step: ShuffleStep) {
        match step {
            ShuffleStep::DealIntoNew => self.deal_into_new(),
            ShuffleStep::Cut(count) => self.cut(count),
            ShuffleStep::DealWithIncrement(count) => self.deal_with_increment(count),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShuffleStep {
    Cut(isize),
    DealIntoNew,
    DealWithIncrement(usize),
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let cards = VecDeque::from(core::array::from_fn::<u16, 10007, _>(|i| i as u16));
    let mut deck = Deck { cards };
    for step in input.lines().map(|l| l.parse().unwrap()) {
        deck.apply(step);
    }
    println!(
        "{}",
        deck.cards
            .into_iter()
            .enumerate()
            .find(|(_, card)| *card == 2019)
            .unwrap()
            .0
    );
}

#[derive(Debug)]
enum StepParseError {
    IntParse(#[allow(dead_code)] ParseIntError),
    FormatError(#[allow(dead_code)] String),
}

impl From<ParseIntError> for StepParseError {
    fn from(e: ParseIntError) -> Self {
        Self::IntParse(e)
    }
}

impl std::str::FromStr for ShuffleStep {
    type Err = StepParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();

        match &words[..] {
            ["cut", count] => Ok(Self::Cut(count.parse()?)),
            ["deal", "into", "new", "stack"] => Ok(Self::DealIntoNew),
            ["deal", "with", "increment", count] => Ok(Self::DealWithIncrement(count.parse()?)),
            _ => Err(StepParseError::FormatError(s.into())),
        }
    }
}

#[cfg(test)]
mod aoc_examples {
    use super::Deck;
    use std::collections::VecDeque;
    fn mini_deck() -> Deck {
        Deck {
            cards: (0..10).collect(),
        }
    }

    #[test]
    fn new_stack_example() {
        let mut deck = mini_deck();
        deck.deal_into_new();
        assert_eq!(deck.cards, VecDeque::from([9, 8, 7, 6, 5, 4, 3, 2, 1, 0]));
    }

    #[test]
    fn cut_example_positive() {
        let mut deck = mini_deck();
        deck.cut(3);
        assert_eq!(deck.cards, VecDeque::from([3, 4, 5, 6, 7, 8, 9, 0, 1, 2]));
    }

    #[test]
    fn cut_example_negative() {
        let mut deck = mini_deck();
        deck.cut(-4);
        assert_eq!(deck.cards, VecDeque::from([6, 7, 8, 9, 0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn deal_increment_example() {
        let mut deck = mini_deck();
        deck.deal_with_increment(3);
        assert_eq!(deck.cards, VecDeque::from([0, 7, 4, 1, 8, 5, 2, 9, 6, 3]));
    }

    #[test]
    fn complete_examples() {
        let mut deck = mini_deck();
        deck.apply("deal with increment 7".parse().unwrap());
        deck.apply("deal into new stack".parse().unwrap());
        deck.apply("deal into new stack".parse().unwrap());
        assert_eq!(deck.cards, VecDeque::from([0, 3, 6, 9, 2, 5, 8, 1, 4, 7]));

        let mut deck = mini_deck();
        deck.apply("cut 6".parse().unwrap());
        deck.apply("deal with increment 7".parse().unwrap());
        deck.apply("deal into new stack".parse().unwrap());
        assert_eq!(deck.cards, VecDeque::from([3, 0, 7, 4, 1, 8, 5, 2, 9, 6]));

        let mut deck = mini_deck();
        deck.apply("cut 6".parse().unwrap());
        deck.apply("deal with increment 7".parse().unwrap());
        deck.apply("deal into new stack".parse().unwrap());
        assert_eq!(deck.cards, VecDeque::from([3, 0, 7, 4, 1, 8, 5, 2, 9, 6]));

        let mut deck = mini_deck();
        deck.apply("deal into new stack".parse().unwrap());
        deck.apply("cut -2".parse().unwrap());
        deck.apply("deal with increment 7".parse().unwrap());
        deck.apply("cut 8".parse().unwrap());
        deck.apply("cut -4".parse().unwrap());
        deck.apply("deal with increment 7".parse().unwrap());
        deck.apply("cut 3".parse().unwrap());
        deck.apply("deal with increment 9".parse().unwrap());
        deck.apply("deal with increment 3".parse().unwrap());
        deck.apply("cut -1".parse().unwrap());
        assert_eq!(deck.cards, VecDeque::from([9, 2, 5, 8, 1, 4, 7, 0, 3, 6]));
    }
}
