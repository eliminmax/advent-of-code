// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 10 Part 2

use std::num::NonZeroU64;
#[derive(Debug, PartialEq, Clone, Copy)]
enum BracketType {
    Parentheses,
    Square,
    Curly,
    Angle,
}

fn score_line(line: &str) -> Option<NonZeroU64> {
    let mut need_closing: Vec<BracketType> = Vec::with_capacity(line.len());

    macro_rules! check_open_bracket {
        ($expected: ident) => {{
            let expected_bracket = need_closing
                .pop()
                .expect("No opening brace to check against");
            if expected_bracket != BracketType::$expected {
                return None;
            }
        }};
    }
    for bracket in line.trim().chars() {
        match bracket {
            '(' => need_closing.push(BracketType::Parentheses),
            '[' => need_closing.push(BracketType::Square),
            '{' => need_closing.push(BracketType::Curly),
            '<' => need_closing.push(BracketType::Angle),
            ')' => check_open_bracket!(Parentheses),
            ']' => check_open_bracket!(Square),
            '}' => check_open_bracket!(Curly),
            '>' => check_open_bracket!(Angle),
            c => panic!("Unexpected character {c:?} in line {line:?}"),
        }
    }

    let mut score = 0;
    while let Some(bracket) = need_closing.pop() {
        score *= 5;
        score += match bracket {
            BracketType::Parentheses => 1,
            BracketType::Square => 2,
            BracketType::Curly => 3,
            BracketType::Angle => 4,
        };
    }

    Some(NonZeroU64::new(score).expect("At least one extra bracket"))
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let mut scores: Vec<u64> = input
        .lines()
        .filter_map(score_line)
        .map(NonZeroU64::get)
        .collect();

    debug_assert!(
        scores.len() % 2 == 1,
        "problem states that there will be an odd number of items"
    );

    scores.sort();

    let score = scores[scores.len() / 2];
    println!("{score}");
}
