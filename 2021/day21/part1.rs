// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 21 Part 1

#[derive(Debug, Clone)]
struct DeterministicDie {
    next_up: u8,
}

impl DeterministicDie {
    const fn roll(&mut self) -> u32 {
        let rolled = self.next_up;
        self.next_up += 1;
        self.next_up %= 100;
        rolled as _
    }
    const fn new() -> Self {
        Self { next_up: 1 }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct PlayerInfo {
    score: u32,
    space: u8,
}

impl PlayerInfo {
    /// take a turn, returning the score at the end of the turn
    const fn take_turn(&mut self, die: &mut DeterministicDie) {
        let roll = die.roll() + die.roll() + die.roll();
        // subtract 1 before modulus, then add it back afterwards, to keep it in range 1..=10
        self.space = (((self.space as u32) + roll - 1) % 10 + 1) as u8;
        self.score += self.space as u32;
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let (p1_start, p2_start) = input.trim().split_once('\n').unwrap();
    let p1_start = p1_start
        .strip_prefix("Player 1 starting position: ")
        .unwrap()
        .parse()
        .unwrap();
    let p2_start = p2_start
        .strip_prefix("Player 2 starting position: ")
        .unwrap()
        .parse()
        .unwrap();
    let mut p1 = PlayerInfo {
        score: 0,
        space: p1_start,
    };
    let mut p2 = PlayerInfo {
        score: 0,
        space: p2_start,
    };
    let mut die = DeterministicDie::new();
    let mut roll_count = 0;
    while p2.score < 1000 {
        p1.take_turn(&mut die);
        roll_count += 3;
        if p1.score >= 1000 {
            break;
        }
        p2.take_turn(&mut die);
        roll_count += 3;
    }
    println!("{}", roll_count * p1.score.min(p2.score));
}
