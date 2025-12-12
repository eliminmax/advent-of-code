// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 16 Part 2

use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct FieldInfo {
    name: String,
    ranges: [RangeInclusive<u16>; 2],
}

/// go through a queue of columns, trying to see if only one unresolved field fits it. If so, it
/// removes that field from `unresolved_fields` and maps it to the column number, otherwise, it
/// returns it to the queue
///
/// Panics if a column can't match any unresolved fields.
/// Does not guard against the possibility of infinite loops.
fn resolve_fields(
    mut unresolved_fields: HashSet<FieldInfo>,
    nearby_tickets: &[Vec<u16>],
) -> HashMap<String, usize> {
    let mut unmatched_columns: VecDeque<usize> = (0..unresolved_fields.len()).collect();
    let mut resolved = HashMap::with_capacity(unresolved_fields.len());
    while let Some(col) = unmatched_columns.pop_front() {
        let mut possibilities = unresolved_fields.clone();
        for ticket in nearby_tickets {
            possibilities.retain(|f| {
                f.ranges[0].contains(&ticket[col]) || f.ranges[1].contains(&ticket[col])
            });
        }
        assert!(!possibilities.is_empty());
        if possibilities.len() == 1 {
            let field = possibilities.into_iter().next().unwrap();
            unresolved_fields.remove(&field);
            resolved.insert(field.name.to_owned(), col);
        } else {
            unmatched_columns.push_back(col);
        }
    }

    resolved
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut lines = input.lines();
    let mut fields: HashSet<FieldInfo> = HashSet::new();

    while let Some(line) = lines.next()
        && !line.is_empty()
    {
        fields.insert(line.parse().unwrap());
    }

    let valid_ranges: HashSet<RangeInclusive<u16>> =
        fields.iter().flat_map(|fi| fi.ranges.clone()).collect();

    assert_eq!(lines.next(), Some("your ticket:"));

    let my_ticket: Vec<u16> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|item| item.parse().unwrap())
        .collect();
    assert_eq!(my_ticket.len(), fields.len());

    assert_eq!(lines.next(), Some(""));
    assert_eq!(lines.next(), Some("nearby tickets:"));
    let mut nearby_tickets: Vec<Vec<u16>> = lines
        .map(|line| line.split(",").map(|i| i.parse().unwrap()).collect())
        .collect();
    assert!(nearby_tickets
        .iter()
        .all(|ticket| ticket.len() == my_ticket.len()));

    // discard nearby tickets with invalid items
    nearby_tickets.retain(|ticket| {
        ticket
            .iter()
            .all(|field| valid_ranges.iter().any(|r| r.contains(field)))
    });

    let mappings = resolve_fields(fields, &nearby_tickets);
    let answer: u64 = mappings
        .into_iter()
        .filter(|(field, _)| field.starts_with("departure"))
        .map(|(_, col)| u64::from(my_ticket[col]))
        .product();
    println!("{answer}");
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
                name: name.into(),
                ranges: [r0, r1],
            })
        } else {
            Err(FieldInfoParseFailure)
        }
    }
}
