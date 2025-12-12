// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 20 Part 2

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BlockRange {
    min: u32,
    max: u32,
}

impl BlockRange {
    fn range_size(&self) -> u32 {
        // add one because if they're equal, then they still block one address
        self.max - self.min + 1
    }
}

#[derive(Debug)]
enum BlockRangeParseError {
    MissingDelimiter,
    EmptyRange,
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
        let min = min.parse()?;
        let max = max.parse()?;
        if min > max {
            Err(BlockRangeParseError::EmptyRange)
        } else {
            Ok(BlockRange { min, max })
        }
    }
}

fn simplify_firewall(mut fw: Vec<BlockRange>) -> Vec<BlockRange> {
    use std::collections::VecDeque;
    if fw.is_empty() {
        return fw;
    }
    fw.sort();
    let mut fw: VecDeque<_> = fw.into();
    let mut simplified: Vec<BlockRange> = Vec::new();
    while let Some(rule) = fw.pop_front() {
        if rule.max < rule.min {
            panic!("rule starts before it ends");
        }
        if let Some(&mut ref mut prev_rule) = simplified.last_mut() {
            if rule.max <= prev_rule.max {
                // rule is covered by previous rule, so skip it
                continue;
            }
            if rule.min <= prev_rule.max {
                // previous rule and current rule overlap, so merge them and go to the next rule
                prev_rule.max = rule.max;
                continue;
            }
        }
        // only reach the end of the loop body if the rule doesn't overlap the previous rule.
        simplified.push(rule);
    }
    simplified
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let firewall: Vec<BlockRange> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse line as ip range"))
        .collect();
    let firewall = simplify_firewall(firewall);
    let blocked_size: u32 = firewall.into_iter().map(|range| range.range_size()).sum();
    println!("{}", u32::MAX - blocked_size + 1);
}
