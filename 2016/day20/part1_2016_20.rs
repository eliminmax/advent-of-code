// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 20 Part 1

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BlockRange {
    min: u32,
    max: u32,
}

#[derive(Debug)]
enum BlockRangeParseError {
    MissingDelimiter,
    ParseIntError,
}

impl From<std::num::ParseIntError> for BlockRangeParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        BlockRangeParseError::ParseIntError
    }
}

impl std::str::FromStr for BlockRange {
    type Err = BlockRangeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (min, max) = s
            .split_once('-')
            .ok_or(BlockRangeParseError::MissingDelimiter)?;
        Ok(BlockRange {
            min: min.parse()?,
            max: max.parse()?,
        })
    }
}

trait FireWall {
    fn ip_allowed(&self, ip_addr: u32) -> bool;
}

impl FireWall for Vec<BlockRange> {
    fn ip_allowed(&self, ip_addr: u32) -> bool {
        self.iter()
            .all(|range| ip_addr < range.min || ip_addr > range.max)
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let mut firewall: Vec<BlockRange> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse line as ip range"))
        .collect();
    // sort the firewall so that ranges that start earlier are checked earlier
    firewall.sort();

    for i in 0u32.. {
        if firewall.ip_allowed(i) {
            println!("{i}");
            break;
        }
    }
}
