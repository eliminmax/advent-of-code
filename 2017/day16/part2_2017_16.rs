// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 16 Part 2

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Clone)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

#[derive(Debug)]
enum DanceMoveParseError {
    IndexParseFail,
    BadFormat,
}

impl From<std::num::ParseIntError> for DanceMoveParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        DanceMoveParseError::IndexParseFail
    }
}

#[derive(Debug)]
struct MissingProgramError;
fn position_of<'a, T: PartialEq + 'a, I: Iterator<Item = &'a T>>(
    target: &T,
    iterable: I,
) -> Result<usize, MissingProgramError> {
    for (i, c) in iterable.enumerate() {
        if *c == *target {
            return Ok(i);
        }
    }
    Err(MissingProgramError)
}

impl std::str::FromStr for DanceMove {
    type Err = DanceMoveParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        match chars.next() {
            Some('s') => Ok(DanceMove::Spin(chars.collect::<String>().parse()?)),
            Some('x') => {
                let pos_a: usize = chars
                    .by_ref()
                    .take_while(|&c| c != '/')
                    .collect::<String>()
                    .parse()?;
                let pos_b: usize = chars.collect::<String>().parse()?;
                Ok(DanceMove::Exchange(pos_a, pos_b))
            }
            Some('p') => {
                let prog_a = chars.next().ok_or(DanceMoveParseError::BadFormat)?;
                if chars.next().is_none_or(|c| c != '/') {
                    return Err(DanceMoveParseError::BadFormat);
                }
                let prog_b = chars.next().ok_or(DanceMoveParseError::BadFormat)?;
                Ok(DanceMove::Partner(prog_a, prog_b))
            }
            _ => Err(DanceMoveParseError::BadFormat),
        }
    }
}

fn dance(moves: VecDeque<DanceMove>) -> Result<String, MissingProgramError> {
    use std::iter::FromIterator;
    let mut progs = VecDeque::from_iter('a'..='p');
    let mut sequences: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..1_000_000_000usize {
        if sequences.contains(&progs) {
            break;
        }
        sequences.push(progs.clone());

        let mut moves = moves.clone();
        while let Some(dance_move) = moves.pop_front() {
            match dance_move {
                DanceMove::Spin(i) => progs.rotate_right(i),
                DanceMove::Exchange(x, y) => progs.swap(x, y),
                DanceMove::Partner(a, b) => {
                    let x = position_of(&a, progs.iter())?;
                    let y = position_of(&b, progs.iter())?;
                    progs.swap(x, y);
                }
            }
        }
    }
    Ok(sequences[1_000_000_000 % sequences.len()].iter().collect())
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let moves: VecDeque<DanceMove> = input
        .split(',')
        .map(|s| s.parse().expect("Failed to parse dance move"))
        .collect();
    println!(
        "{}",
        dance(moves).expect("Tried to partner nonexistent program")
    );
}
