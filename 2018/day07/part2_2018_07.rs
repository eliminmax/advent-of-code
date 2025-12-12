// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 7 Part 2

use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
enum OrderingError {
    InvalidFormat,
    InvalidInstructionId,
    Unresolvable,
}

// you + number of elves helping. It's 2 in the simplified sample, but 5 in the actual scenario.
const WORKER_COUNT: usize = 5;

#[derive(Debug, PartialEq, Copy, Clone)]
struct InstructionStep {
    id: u8,
    time_left: u8,
}

impl From<u8> for InstructionStep {
    fn from(value: u8) -> InstructionStep {
        assert!(value.is_ascii_uppercase());
        // A takes 60+1 seconds, and Z takes 60+26 seconds, so subtract 61 from b'A' to get the
        // offset from value to second count
        const OFFSET: u8 = b'A' - 61;
        InstructionStep {
            id: value,
            time_left: value - OFFSET,
        }
    }
}

trait Ordering {
    fn load_entry(&mut self, s: &str) -> Result<(), OrderingError>;
    fn time_needed(self) -> Result<u32, OrderingError>;
}

impl Ordering for HashMap<u8, Vec<u8>> {
    fn load_entry(&mut self, s: &str) -> Result<(), OrderingError> {
        let words: Vec<&str> = s.split_whitespace().collect();
        // I don't know how I feel about this if let construct
        if let [
            "Step",
            dep,
            "must",
            "be",
            "finished",
            "before",
            "step",
            step,
            "can",
            "begin.",
        ] = words.as_slice()
        {
            if (dep.len() != 1) || (step.len() != 1) {
                return Err(OrderingError::InvalidInstructionId);
            }

            let step = step.bytes().next().expect("step is 1 byte long as-is");
            let dep = dep.bytes().next().expect("step is 1 byte long as-is");

            if !(step.is_ascii_uppercase() && dep.is_ascii_uppercase()) {
                return Err(OrderingError::InvalidInstructionId);
            }

            self.entry(step)
                .and_modify(|v| v.push(dep))
                .or_insert(vec![dep]);
            self.entry(dep).or_default();
            Ok(())
        } else {
            Err(OrderingError::InvalidFormat)
        }
    }
    fn time_needed(mut self) -> Result<u32, OrderingError> {
        let mut unblocked: Vec<u8> = Vec::new();
        let mut workers: [Option<InstructionStep>; WORKER_COUNT] = [const { None }; WORKER_COUNT];
        let mut total_time = 0u32;
        while !self.is_empty() || workers != [const { None }; WORKER_COUNT] {
            let mut changed = false; // track whether any state change occurred
            for worker in workers.iter_mut() {
                if let Some(step) = worker {
                    changed = true; // both branches change the state
                    if step.time_left > 1 {
                        step.time_left -= 1;
                    } else {
                        self.values_mut().for_each(|v| v.retain(|c| *c != step.id));
                        *worker = None;
                    }
                }
            }
            let mut newly_unblocked: Vec<u8> = self
                .iter()
                .filter_map(|(k, v)| if v.is_empty() { Some(*k) } else { None })
                .collect();
            unblocked.append(&mut newly_unblocked);
            // the following is listed in the Rust Vec docs as a way to sort a vec backwards, and
            // the next unblocked ID alphabetically should be processed next, so should be at the
            // end of the Vec.
            unblocked.sort_by(|a, b| b.cmp(a));
            unblocked.dedup();
            for worker in workers.iter_mut() {
                if worker.is_none() {
                    if let Some(val) = unblocked.pop() {
                        changed = true;
                        let step = InstructionStep::from(val);
                        self.remove(&step.id);
                        *worker = Some(step);
                    }
                }
            }
            if !changed {
                return Err(OrderingError::Unresolvable);
            }
            total_time += 1;
        }
        Ok(total_time - 1)
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut order_resolver: HashMap<u8, Vec<u8>> = HashMap::new();
    input.lines().for_each(|line| {
        order_resolver
            .load_entry(line)
            .expect("Failed to parse line")
    });
    println!(
        "{}",
        order_resolver
            .time_needed()
            .expect("Failed to resolve order")
    );
}
