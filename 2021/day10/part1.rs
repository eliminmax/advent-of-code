// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 10 Part 1

#[derive(Debug, PartialEq, Clone, Copy)]
enum BracketType {
    Parentheses,
    Square,
    Curly,
    Angle,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum LineErrorKind {
    IllegalClose(BracketType),
    Incomplete,
}

fn classify_line(line: &str) -> LineErrorKind {
    let mut expected: Vec<BracketType> = Vec::with_capacity(line.len());

    macro_rules! check_open_bracket {
        ($expected: ident) => {{
            let expected_bracket = expected.pop().expect("No opening brace to check against");
            if expected_bracket != BracketType::$expected {
                return LineErrorKind::IllegalClose(BracketType::$expected);
            }
        }};
    }
    for bracket in line.trim().chars() {
        match bracket {
            '(' => expected.push(BracketType::Parentheses),
            '[' => expected.push(BracketType::Square),
            '{' => expected.push(BracketType::Curly),
            '<' => expected.push(BracketType::Angle),
            ')' => check_open_bracket!(Parentheses),
            ']' => check_open_bracket!(Square),
            '}' => check_open_bracket!(Curly),
            '>' => check_open_bracket!(Angle),
            c => panic!("Unexpected character {c:?} in line {line:?}"),
        }
    }

    debug_assert!(
        !expected.is_empty(),
        "All input lines have mismatches or are incomplete"
    );

    LineErrorKind::Incomplete
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let score: u32 = input
        .lines()
        .map(|line| match classify_line(line) {
            LineErrorKind::Incomplete => 0,
            LineErrorKind::IllegalClose(BracketType::Parentheses) => 3,
            LineErrorKind::IllegalClose(BracketType::Square) => 57,
            LineErrorKind::IllegalClose(BracketType::Curly) => 1197,
            LineErrorKind::IllegalClose(BracketType::Angle) => 25137,
        })
        .sum();
    println!("{score}");
}
