// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 05 Part 1

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    /// Check if `id` is in range
    fn contains(&self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }
}

/// sort `ranges`, and merge overlapping ranges
fn normalize_ranges(mut ranges: VecDeque<IdRange>) -> Box<[IdRange]> {
    ranges.make_contiguous().sort();
    let mut merged_ranges = Vec::with_capacity(ranges.len());
    while let Some(mut range) = ranges.pop_front() {
        while ranges
            .front()
            .is_some_and(|next| range.contains(next.start))
        {
            let next = ranges.pop_front().unwrap();
            range.end = range.end.max(next.end);
        }
        merged_ranges.push(range);
    }
    merged_ranges.into_boxed_slice()
}

fn main() -> Result<(), ProgramError> {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let (ranges, ids) = input
        .split_once("\n\n")
        .expect("blank line separating sections");
    let ranges = normalize_ranges(ranges.lines().map(str::parse).collect::<Result<_, _>>()?);
    let mut total: u32 = 0;
    for id in ids.lines() {
        let id: u64 = id.parse()?;
        let idx = ranges.partition_point(|r| r.end < id);
        if ranges.get(idx).is_some_and(|r| r.contains(id)) {
            total += 1;
        }
    }
    println!("{total}");
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
            Err(Self::Err::NegativeSpan(i128::from(end) - i128::from(start)))
        } else {
            Ok(Self { start, end })
        }
    }
}

#[derive(Debug)]
enum IdRangeParseError {
    MissingDelimiter,
    NegativeSpan(#[allow(dead_code)] i128),
    IntParse(#[allow(dead_code)] ParseIntError),
}

#[derive(Debug)]
enum ProgramError {
    IdRange(#[allow(dead_code)] IdRangeParseError),
    IntParse(#[allow(dead_code)] ParseIntError),
}

macro_rules! impl_into_variant {
    {$from: ty, $to: ty, $variant: ident} => {
        impl From<$from> for $to {
            fn from(e: $from) -> Self {
                <$to>::$variant(e)
            }
        }
    }
}

impl_into_variant! {ParseIntError, IdRangeParseError, IntParse }
impl_into_variant! {ParseIntError, ProgramError, IntParse }
impl_into_variant! {IdRangeParseError, ProgramError, IdRange }
