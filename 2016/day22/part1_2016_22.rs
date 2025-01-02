// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 22 Part 1

#[derive(Debug, PartialEq)]
struct Node {
    used: u32,
    avail: u32,
}

#[derive(Debug)]
enum NodeParseError {
    ParseInt,
    UnrecognizedFormat,
}

impl<T> From<Vec<T>> for NodeParseError {
    fn from(_e: Vec<T>) -> Self {
        NodeParseError::UnrecognizedFormat
    }
}
impl From<std::num::ParseIntError> for NodeParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        NodeParseError::ParseInt
    }
}

impl std::str::FromStr for Node {
    type Err = NodeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::convert::TryInto;
        let [used, avail]: [&str; 2] = s
            .split_whitespace()
            .skip(2) // skip dev ID and size
            .filter_map(|w| w.strip_suffix('T')) // only keep item if suffix was there
            .collect::<Vec<_>>()
            .try_into()?;
        Ok(Node {
            used: used.parse()?,
            avail: avail.parse()?,
        })
    }
}

fn main() {
    use std::collections::VecDeque;
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut nodes: VecDeque<Node> = input
        .lines()
        .skip(2)
        .map(|line| line.parse().expect("Failed to parse node"))
        .collect();
    let mut total = 0usize;
    for _ in 0..nodes.len() {
        let used = nodes[0].used;
        if used != 0 {
            total += nodes.range(1..).filter(|node| node.avail >= used).count();
        }
        nodes.rotate_left(1);
    }
    println!("{total}");
}
