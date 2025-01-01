// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 15 Part 1

use std::num::NonZero;

#[derive(Debug)]
struct Disc {
    id: u32,
    start_pos: u32,
    cycle_length: NonZero<u32>,
}

#[derive(Debug)]
struct DiscParseError;
impl From<std::num::ParseIntError> for DiscParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        DiscParseError
    }
}

impl std::str::FromStr for Disc {
    type Err = DiscParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_suffix('.').ok_or(DiscParseError)?;
        let words: Vec<_> = s.split_whitespace().collect();
        let id_str = words
            .get(1)
            .and_then(|w| w.strip_prefix('#'))
            .ok_or(DiscParseError)?;
        Ok(Disc {
            id: id_str.parse()?,
            start_pos: words.get(11).ok_or(DiscParseError)?.parse()?,
            cycle_length: words.get(3).ok_or(DiscParseError)?.parse()?,
        })
    }
}

fn test_time(time: u32, group: &[Disc]) -> bool {
    0u32 == group
        .iter()
        .map(|disc| (disc.id + disc.start_pos + time) % disc.cycle_length)
        .sum()
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let discs: Vec<Disc> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse disc"))
        .collect();
    let mut i = 0;
    loop {
        if test_time(i, discs.as_slice()) {
            println!("{i}");
            break;
        }
        i += 1;
    }
}
