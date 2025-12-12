// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 25 Part 1
use std::collections::{BTreeMap, HashMap, VecDeque};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Left = -1,
    Right = 1,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Rule {
    write: bool,
    move_dir: Direction,
    next_state: char,
}

#[derive(Debug)]
struct TuringMachine {
    state: char,
    tape: HashMap<i32, bool>,
    rules: BTreeMap<char, [Rule; 2]>,
    location: i32,
}

impl TuringMachine {
    fn checksum(&self) -> usize {
        self.tape.values().filter(|&v| *v).count()
    }
    fn update(&mut self) {
        let current_state: usize = self
            .tape
            .get(&self.location)
            .cloned()
            .unwrap_or(false)
            .into();
        let Rule {
            write,
            move_dir,
            next_state,
        } = self.rules[&self.state][current_state];
        self.tape
            .entry(self.location)
            .and_modify(|e| *e = write)
            .or_insert(write);
        self.location += move_dir as i32;
        self.state = next_state;
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let checksum_after: u32 = input
        .lines()
        .nth(1)
        .and_then(|l| l.split_whitespace().nth(5))
        .expect("extracting 6th word of line 2")
        .parse()
        .expect("parsing checksum_after from 6th word of line 2");
    let mut tm: TuringMachine = input
        .parse()
        .expect("Turing machine should be in format from Advent of Code");
    for _ in 0..checksum_after {
        tm.update();
    }
    println!("{}", tm.checksum());
}

#[derive(Debug)]
enum TMParseError {
    RuleParseFailure,
    UnknownStartState,
    UnknownStateId,
    DuplicateEntry(#[expect(unused)] char),
}

fn parse_rule(lines: &mut VecDeque<&str>) -> Result<Rule, TMParseError> {
    Ok(Rule {
        write: lines
            .pop_front()
            .and_then(|l| l.chars().nth(22)) // next state is the character at index 22
            .and_then(|c| match c {
                '0' => Some(false),
                '1' => Some(true),
                _ => None,
            })
            .ok_or(TMParseError::RuleParseFailure)?,
        move_dir: lines
            .pop_front()
            .and_then(|l| l.split_whitespace().last())
            .and_then(|d| match d {
                "right." => Some(Direction::Right),
                "left." => Some(Direction::Left),
                _ => None,
            })
            .ok_or(TMParseError::RuleParseFailure)?,
        next_state: lines
            .pop_front()
            .and_then(|l| l.chars().nth(26)) // index of next state character
            .ok_or(TMParseError::RuleParseFailure)?,
    })
}

impl std::str::FromStr for TuringMachine {
    type Err = TMParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_state = s.chars().nth(15).ok_or(TMParseError::UnknownStartState)?;

        // using a VecDeque as `while let Some(...) = vec_deque.pop_front()` does not require
        // maintaining ownership during the loop body, but `while let Some(...) = iter.next()`
        // does.
        let mut lines: VecDeque<&str> = s.lines().skip(3).collect();
        let mut rules: BTreeMap<char, [Rule; 2]> = BTreeMap::new();

        while let Some(state) = lines.pop_front() {
            let state_id: char = state.chars().nth(9).ok_or(TMParseError::UnknownStateId)?;
            // skip the "If the current value is 0" line
            let _ = lines.pop_front();
            let rule_for_0 = parse_rule(&mut lines)?;
            // skip the "If the current value is 1" line
            let _ = lines.pop_front();
            let rule_for_1 = parse_rule(&mut lines)?;
            if rules.insert(state_id, [rule_for_0, rule_for_1]).is_some() {
                return Err(TMParseError::DuplicateEntry(state_id));
            }
            // skip the blank line
            let _ = lines.pop_front();
        }
        Ok(TuringMachine {
            state: start_state,
            tape: HashMap::new(),
            rules,
            location: 0,
        })
    }
}
