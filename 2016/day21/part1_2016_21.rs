// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 21 Part 1

#[derive(Debug, PartialEq)]
enum ScrambleOp {
    SwapPos(usize, usize),
    SwapLetters(char, char),
    RotLeft(usize),
    RotRight(usize),
    RotFromLet(char),
    Reverse { start: usize, end: usize },
    Move { from: usize, to: usize },
}

#[derive(Debug)]
enum ScrambleOpParseError {
    IndexParse,
    LetterSize,
    BadRange,
    UnrecognizedOp,
}
impl From<std::num::ParseIntError> for ScrambleOpParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        ScrambleOpParseError::IndexParse
    }
}

fn letter_from(s: &str) -> Result<char, ScrambleOpParseError> {
    let s: Vec<_> = s.chars().collect();
    if s.len() == 1 {
        Ok(s[0])
    } else {
        Err(ScrambleOpParseError::LetterSize)
    }
}

impl std::str::FromStr for ScrambleOp {
    type Err = ScrambleOpParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_whitespace().collect();
        match &words[..] {
            ["swap", "position", x, "with", "position", y] => {
                Ok(Self::SwapPos(x.parse()?, y.parse()?))
            }
            ["swap", "letter", x, "with", "letter", y] => {
                Ok(Self::SwapLetters(letter_from(x)?, letter_from(y)?))
            }
            ["rotate", "right", x, "steps"] => Ok(Self::RotRight(x.parse()?)),
            ["rotate", "left", x, "steps"] => Ok(Self::RotLeft(x.parse()?)),
            ["rotate", "right", "1", "step"] => Ok(Self::RotRight(1)),
            ["rotate", "left", "1", "step"] => Ok(Self::RotLeft(1)),
            ["rotate", "based", "on", _, _, _, x] => Ok(Self::RotFromLet(letter_from(x)?)),
            ["reverse", "positions", x, "through", y] => {
                let start: usize = x.parse()?;
                let end: usize = y.parse()?;
                if end < start {
                    Err(ScrambleOpParseError::BadRange)
                } else {
                    Ok(Self::Reverse { start, end })
                }
            }
            ["move", "position", from, "to", "position", to] => Ok(Self::Move {
                from: from.parse()?,
                to: to.parse()?,
            }),
            _ => Err(ScrambleOpParseError::UnrecognizedOp),
        }
    }
}

#[derive(Debug)]
enum ScrambleError {
    MissingLetter,
    OutOfBounds,
}

fn scramble<I>(rules: I, s: &str) -> Result<String, ScrambleError>
where
    I: IntoIterator<Item = ScrambleOp>,
{
    use std::collections::VecDeque;
    use ScrambleOp as Op;
    let mut letters: VecDeque<char> = s.chars().collect();
    let position_of = |target: char, v: &VecDeque<char>| -> Result<usize, ScrambleError> {
        for (i, c) in v.iter().enumerate() {
            if *c == target {
                return Ok(i);
            }
        }
        Err(ScrambleError::MissingLetter)
    };
    for op in rules.into_iter() {
        match op {
            Op::SwapPos(x, y) => letters.swap(x, y),
            Op::SwapLetters(a, b) => {
                let x = position_of(a, &letters)?;
                let y = position_of(b, &letters)?;
                letters.swap(x, y);
            }
            Op::RotLeft(n) => letters.rotate_left(n),
            Op::RotRight(n) => letters.rotate_right(n),
            Op::RotFromLet(c) => {
                let mut i = position_of(c, &letters)? + 1;
                // already added one, so if index was 4, i is 5, so use > instead of >=
                if i > 4 {
                    i += 1;
                }
                letters.rotate_right(i);
            }
            Op::Reverse { start, end } => {
                let mut reverser: Vec<char> = letters.range(start..=end).cloned().collect();
                reverser.reverse();
                let mut reverser = reverser.into_iter();
                letters
                    .range_mut(start..=end)
                    .for_each(|c| *c = reverser.next().unwrap_or_else(|| unreachable!()));
            }
            Op::Move { from, to } => {
                let l = letters.remove(from).ok_or(ScrambleError::OutOfBounds)?;
                letters.insert(to, l);
            }
        }
    }
    Ok(letters.into_iter().collect())
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let scrambler: Vec<ScrambleOp> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse scramble op"))
        .collect();
    println!(
        "{}",
        scramble(scrambler, "abcdefgh").expect("Failed to scramble string")
    );
}
