// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 05 Part 2

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct IdRange {
    start: u64,
    end: u64,
}

/// sort `ranges`, and merge overlapping ranges
fn tally_fresh(mut ranges: VecDeque<IdRange>) -> u64 {
    ranges.make_contiguous().sort();
    let mut merged_ranges = Vec::with_capacity(ranges.len());
    while let Some(mut range) = ranges.pop_front() {
        while ranges.front().is_some_and(|next| range.end >= next.start) {
            let next = ranges.pop_front().unwrap();
            range.end = range.end.max(next.end);
        }
        merged_ranges.push(range);
    }
    merged_ranges
        .into_iter()
        .map(|IdRange { start, end }| end - start + 1)
        .sum()
}

fn main() -> Result<(), IdRangeParseError> {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let (ranges, _) = input
        .split_once("\n\n")
        .expect("blank line separating sections");
    let fresh_ids: u64 = tally_fresh(ranges.lines().map(str::parse).collect::<Result<_, _>>()?);
    println!("{fresh_ids}");
    Ok(())
}

use std::num::ParseIntError;

impl std::str::FromStr for IdRange {
    type Err = IdRangeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').ok_or(Self::Err::MissingDelimiter)?;
        let start = start.parse()?;
        let end = end.parse()?;

        if end < start {
            Err(Self::Err::NegativeSpan { start, end })
        } else {
            Ok(Self { start, end })
        }
    }
}

#[derive(Debug)]
enum IdRangeParseError {
    MissingDelimiter,
    NegativeSpan {
        #[allow(dead_code)]
        start: u64,
        #[allow(dead_code)]
        end: u64,
    },
    IntParse(#[allow(dead_code)] ParseIntError),
}

impl From<ParseIntError> for IdRangeParseError {
    fn from(e: ParseIntError) -> Self {
        IdRangeParseError::IntParse(e)
    }
}
