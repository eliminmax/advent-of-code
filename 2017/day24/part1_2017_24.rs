// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 24 Part 1
use std::collections::VecDeque;

// impls for std::fmt::Debug and std::str::FromStr below main
#[derive(PartialEq, Clone)]
struct BridgePin(u8, u8);

#[derive(Debug, PartialEq, Clone, Default)]
struct Bridge {
    exposed_port: u8,
    score: u16,
}

impl BridgePin {
    fn score(&self) -> u16 {
        (self.0 + self.1).into()
    }

    /// try to link with the bridge_link, returning the new bridge that creates
    fn link_to(&self, bridge: &Bridge) -> Option<Bridge> {
        if self.0 == bridge.exposed_port {
            Some(Bridge {
                exposed_port: self.1,
                score: bridge.score + self.score(),
            })
        } else if self.1 == bridge.exposed_port {
            Some(Bridge {
                exposed_port: self.0,
                score: bridge.score + self.score(),
            })
        } else {
            None
        }
    }
}

fn strongest_bridge_score(pins: VecDeque<BridgePin>) -> u16 {
    let mut queue: VecDeque<(Bridge, VecDeque<BridgePin>)> =
        VecDeque::from([(Bridge::default(), pins)]);
    let mut strongest: u16 = 0;
    while let Some((bridge, mut pins)) = queue.pop_front() {
        for _ in 0..pins.len() {
            let current_pin = pins.pop_front().expect("non-empty");
            if let Some(next_bridge) = current_pin.link_to(&bridge) {
                strongest = strongest.max(next_bridge.score);
                queue.push_back((next_bridge, pins.clone()));
            }
            pins.push_back(current_pin);
        }
    }
    strongest
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let pins: VecDeque<BridgePin> = input
        .lines()
        .map(|line| {
            line.parse()
                .expect("input must be in proper format for Advent of Code problem")
        })
        .collect();
    println!("{}", strongest_bridge_score(pins));
}

#[derive(Debug)]
enum PinParseError {
    BadStructure(#[expect(unused)] String),
    NumParseFailure(#[expect(unused)] std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for PinParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        PinParseError::NumParseFailure(e)
    }
}

impl std::str::FromStr for BridgePin {
    type Err = PinParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s
            .split_once('/')
            .ok_or(PinParseError::BadStructure(s.to_string()))?;
        Ok(BridgePin(a.parse()?, b.parse()?))
    }
}

impl std::fmt::Debug for BridgePin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.0, self.1)
    }
}
