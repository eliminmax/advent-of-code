// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 22 Part 2

// Observations:
// - There are 6 contiguous nodes which each have over 500T available
//   and over 490T used, but no way to get the data out from them.
// - Of the remaining nodes, the highest used is lower than the lowest size
// - One of those remaining nodes is empty
//
// With those observations, it appears that, at least for my input, it can be reduced into a
// block-sliding puzzle where the goal is to get the block starting in one corner into the opposite
// corner.

use std::collections::{self, HashSet};
use std::iter::FromIterator;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct NodeStatus {
    empty: bool,
}

#[derive(Debug)]
enum NodeParseError {
    ParseInt,
    UnrecognizedFormat,
}

impl<T> From<Vec<T>> for NodeParseError {
    // TryInto<[T; N]> for Vec<T, A> uses Vec<T, A> as its error type
    fn from(_e: Vec<T>) -> Self {
        NodeParseError::UnrecognizedFormat
    }
}

impl From<std::num::ParseIntError> for NodeParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        NodeParseError::ParseInt
    }
}

type Location = (i8, i8);
fn parse_node(s: &str) -> Result<Option<(Location, NodeStatus)>, NodeParseError> {
    use std::convert::TryInto;
    let mut words = s.split_whitespace();
    let [x, y]: [_; 2] = words
        .next()
        .ok_or(NodeParseError::UnrecognizedFormat)?
        .split('/')
        .last()
        .ok_or(NodeParseError::UnrecognizedFormat)?
        .split('-')
        .skip(1)
        .collect::<Vec<_>>()
        .try_into()?;

    let x: i8 = x
        .strip_prefix('x')
        .ok_or(NodeParseError::UnrecognizedFormat)?
        .parse()?;
    let y: i8 = y
        .strip_prefix('y')
        .ok_or(NodeParseError::UnrecognizedFormat)?
        .parse()?;

    let used: u32 = words
        .skip(1) // skip the avail
        .filter_map(|w| w.strip_suffix('T')) // only keep item if suffix was there
        .next()
        .ok_or(NodeParseError::UnrecognizedFormat)?
        .parse()?;

    match used {
        0 => Ok(Some(((x, y), NodeStatus { empty: true }))),
        1..400 => Ok(Some(((x, y), NodeStatus { empty: false }))),
        _ => Ok(None),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct NodeGrid {
    locations: HashSet<Location>,
    empty: Location,
    target: Location,
}

impl NodeGrid {
    /// panics unless there's exactly one empty space and one target space.
    fn new_from<I: IntoIterator<Item = (Location, NodeStatus)>>(iter: I) -> Self {
        use collections::HashMap;
        let raw_data: HashMap<Location, NodeStatus> = HashMap::from_iter(iter);
        let target = raw_data
            .keys()
            .filter(|(_x, y)| *y == 0)
            .cloned()
            .max()
            .expect("Failed to determine target node");
        let empties: Vec<_> = raw_data
            .keys()
            .filter(|k| raw_data[k].empty)
            .cloned()
            .collect();
        assert_eq!(empties.len(), 1);

        NodeGrid {
            locations: HashSet::from_iter(raw_data.into_keys()),
            empty: empties[0],
            target,
        }
    }
}

/// uses a BFS to find the fewest steps needed to get target data to 0, 0.
/// panics if all paths are exhausted without success
fn fewest_steps(grid: NodeGrid) -> u32 {
    use collections::VecDeque;
    let NodeGrid {
        locations,
        empty,
        target,
    } = grid;
    let mut seen_states: HashSet<(Location, Location)> = HashSet::from([(empty, target)]);
    let mut queue: VecDeque<(u32, Location, Location)> = VecDeque::from([(0, empty, target)]);
    macro_rules! neighbors {
        ($loc: ident) => {{
            vec![
                ($loc.0 - 1, $loc.1),
                ($loc.0 + 1, $loc.1),
                ($loc.0, $loc.1 - 1),
                ($loc.0, $loc.1 + 1),
            ]
            .into_iter()
            .filter(|i| locations.contains(i))
        }};
    }
    while let Some((steps, empty, target)) = queue.pop_front() {
        macro_rules! push_if_new {
            ($empty: ident, $target: ident) => {
                if seen_states.insert(($empty, $target)) {
                    queue.push_back((steps + 1, $empty, $target));
                }
            };
        }
        for neighbor in neighbors!(empty) {
            if neighbor == target {
                if empty == (0, 0) {
                    return steps + 1;
                }
                // flip target and empty
                push_if_new!(target, empty);
            } else {
                push_if_new!(neighbor, target);
            }
        }
    }
    panic!("failed to find any move sequence that gets target data to (0, 0)");
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let nodes = NodeGrid::new_from(
        input
            .lines()
            .skip(2)
            .filter_map(|line| parse_node(line).expect("Failed to parse node device")),
    );
    println!("{}", fewest_steps(nodes));
}
