// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 22 Part 1
use std::collections::VecDeque;

type Deck = VecDeque<u8>;

fn combat_round(deck_1: &mut Deck, deck_2: &mut Deck) {
    use std::cmp::Ordering;
    let Some(card_1) = deck_1.pop_front() else {
        return;
    };
    let Some(card_2) = deck_2.pop_front() else {
        deck_1.push_front(card_1);
        return;
    };
    match card_1.cmp(&card_2) {
        Ordering::Greater => {
            deck_1.push_back(card_1);
            deck_1.push_back(card_2);
        }
        Ordering::Less => {
            deck_2.push_back(card_2);
            deck_2.push_back(card_1);
        }
        Ordering::Equal => {
            panic!("someone cheated!");
        }
    }
}

fn score(deck: Deck) -> u32 {
    let mut multiplier = u32::try_from(deck.len()).unwrap();
    let mut score = 0;
    for card in deck {
        score += u32::from(card) * multiplier;
        multiplier -= 1;
    }
    score
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let (player_1, player_2) = input.split_once("\n\n").unwrap();

    let deck_1 = player_1.strip_prefix("Player 1:\n").unwrap().lines();
    let deck_2 = player_2.strip_prefix("Player 2:\n").unwrap().lines();

    let mut deck_1: Deck = deck_1.map(|l| l.parse().unwrap()).collect();
    let mut deck_2: Deck = deck_2.map(|l| l.parse().unwrap()).collect();

    while (!deck_1.is_empty()) && (!deck_2.is_empty()) {
        combat_round(&mut deck_1, &mut deck_2);
    }
    if deck_2.is_empty() {
        println!("{}", score(deck_1));
    } else {
        println!("{}", score(deck_2));
    }
}
