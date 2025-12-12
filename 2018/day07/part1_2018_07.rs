// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 7 Part 1

use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
enum OrderingError {
    InvalidFormat,
    Unresolvable,
}

trait Ordering {
    fn load_entry(&mut self, s: &str) -> Result<(), OrderingError>;
    fn to_order(self) -> Result<String, OrderingError>;
}

impl Ordering for HashMap<char, Vec<char>> {
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
            if (dep.chars().count() != 1) || (step.chars().count() != 1) {
                panic!("Step ids must be 1 character long");
            }
            let step = step.chars().next().expect("step is 1 character long as-is");
            let dep = dep.chars().next().expect("step is 1 character long as-is");
            self.entry(step)
                .and_modify(|v| v.push(dep))
                .or_insert(vec![dep]);
            self.entry(dep).or_default();
            Ok(())
        } else {
            Err(OrderingError::InvalidFormat)
        }
    }
    fn to_order(mut self) -> Result<String, OrderingError> {
        let mut order: String = String::new();
        let mut unblocked: Vec<char> = Vec::new();
        while !self.is_empty() {
            unblocked.extend(
                self.iter()
                    .filter_map(|(k, v)| if v.is_empty() { Some(*k) } else { None }),
            );
            unblocked.sort_by(|a, b| b.cmp(a));
            unblocked.dedup();
            if let Some(step) = unblocked.pop() {
                self.values_mut().for_each(|v| v.retain(|c| *c != step));
                self.remove(&step);
                order.push(step);
            } else {
                return Err(OrderingError::Unresolvable);
            }
        }
        Ok(order)
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut order_resolver: HashMap<char, Vec<char>> = HashMap::new();
    input.lines().for_each(|line| {
        order_resolver
            .load_entry(line)
            .expect("Failed to parse line")
    });
    println!(
        "{}",
        order_resolver.to_order().expect("Failed to resolve order")
    );
}
