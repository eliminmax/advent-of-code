// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 12 Part 2
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::FromIterator;
use std::num::ParseIntError;

type PipeLinks = HashMap<u16, HashSet<u16>>;

trait PipeLinker {
    type Error;
    fn link_from_str(&mut self, s: &str) -> Result<(), Self::Error>;
    fn get_group_with(&self, start_key: u16) -> HashSet<u16>;
    fn count_groups(&self) -> u16;
}

impl PipeLinker for PipeLinks {
    type Error = PipeLinkParseError;

    fn link_from_str(&mut self, s: &str) -> Result<(), Self::Error> {
        let (k, vals) = s
            .split_once(" <-> ")
            .ok_or(PipeLinkParseError::UnknownFormat)?;
        let k: u16 = k.parse()?;
        let vals: Vec<u16> = vals
            .split(", ")
            .map(|v| v.parse())
            .collect::<Result<_, _>>()?;
        for v in vals.iter() {
            if self.get(v).is_some_and(|pipe| !pipe.contains(&k)) {
                return Err(PipeLinkParseError::PipeMismatch);
            }
        }
        if self.insert(k, HashSet::from_iter(vals)).is_none() {
            Ok(())
        } else {
            Err(PipeLinkParseError::DuplicateKey)
        }
    }

    fn get_group_with(&self, start_key: u16) -> HashSet<u16> {
        let mut full_group: HashSet<u16> = HashSet::new();
        if !self.contains_key(&start_key) {
            return HashSet::new();
        }
        let mut queue: VecDeque<u16> = VecDeque::from([start_key]);
        while let Some(key) = queue.pop_front() {
            for neighbor in self[&key].iter().cloned() {
                if full_group.insert(neighbor) {
                    queue.push_back(neighbor);
                }
            }
        }

        full_group
    }

    fn count_groups(&self) -> u16 {
        let mut seen: HashSet<u16> = HashSet::new();
        let mut groups: u16 = 0;
        for key in self.keys().cloned() {
            if seen.insert(key) {
                groups += 1;
                seen.extend(self.get_group_with(key));
            }
        }
        groups
    }
}

#[derive(Debug)]
enum PipeLinkParseError {
    ParseInt,
    UnknownFormat,
    DuplicateKey,
    PipeMismatch,
}

impl From<ParseIntError> for PipeLinkParseError {
    fn from(_e: ParseIntError) -> Self {
        PipeLinkParseError::ParseInt
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut links = PipeLinks::new();
    for line in input.lines() {
        links
            .link_from_str(line)
            .expect("Failed to load link from line");
    }
    println!("{}", links.count_groups());
}
