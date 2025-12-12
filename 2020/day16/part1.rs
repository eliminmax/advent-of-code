// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 16 Part 1

use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
struct FieldInfo {
    _name: String,
    ranges: [RangeInclusive<u16>; 2],
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut lines = input.lines();
    let mut fields: Vec<FieldInfo> = Vec::new();

    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        fields.push(line.parse().unwrap());
    }

    let valid_ranges: Vec<RangeInclusive<u16>> =
        fields.into_iter().flat_map(|fi| fi.ranges).collect();

    assert_eq!(lines.next(), Some("your ticket:"));

    let _my_ticket: Vec<u16> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();

    assert_eq!(lines.next(), Some(""));
    assert_eq!(lines.next(), Some("nearby tickets:"));
    let bad_tickets: u16 = lines
        .flat_map(|line| line.split(",").map(|i| i.parse::<u16>().unwrap()))
        .filter(|field| !valid_ranges.iter().any(|range| range.contains(field)))
        .sum();
    println!("{bad_tickets}");
}

#[derive(Debug)]
struct FieldInfoParseFailure;

impl From<std::num::ParseIntError> for FieldInfoParseFailure {
    fn from(_: std::num::ParseIntError) -> Self {
        Self
    }
}

impl std::str::FromStr for FieldInfo {
    type Err = FieldInfoParseFailure;
    fn from_str(s: &str) -> Result<Self, FieldInfoParseFailure> {
        let parse_range = |rstr: &str| -> Result<RangeInclusive<u16>, FieldInfoParseFailure> {
            if let Some((a, b)) = rstr.split_once("-")
                && let Ok(start) = a.parse()
                && let Ok(stop) = b.parse()
            {
                Ok(start..=stop)
            } else {
                Err(FieldInfoParseFailure)
            }
        };
        let (name, ranges) = s.trim().split_once(": ").ok_or(FieldInfoParseFailure)?;
        let (r0, r1) = ranges.split_once(" or ").ok_or(FieldInfoParseFailure)?;
        let r0 = parse_range(r0)?;
        let r1 = parse_range(r1)?;
        if r0.end() <= r1.start() {
            Ok(Self {
                _name: name.into(),
                ranges: [r0, r1],
            })
        } else {
            Err(FieldInfoParseFailure)
        }
    }
}
