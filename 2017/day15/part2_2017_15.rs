// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 15 Part 2

#[derive(Debug)]
struct DuelingGenerator {
    value: u64,
    factor: u64,
    criteria_num: u64,
}

impl Iterator for DuelingGenerator {
    type Item = u16;
    fn next(&mut self) -> Option<u16> {
        loop {
            self.value *= self.factor;
            self.value %= 2147483647;
            if self.value % self.criteria_num == 0 {
                return Some(self.value as u16);
            }
        }
    }
}

fn judge(rounds: u32, mut a: DuelingGenerator, mut b: DuelingGenerator) -> u32 {
    let mut total = 0;
    for _ in 0..rounds {
        if let (Some(a_val), Some(b_val)) = (a.next(), b.next()) {
            if a_val == b_val {
                total += 1;
            }
        }
    }
    total
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut lines = input.lines();

    let mut next_val = || -> u64 {
        lines
            .next()
            .expect("Missing line in input")
            .split_whitespace()
            .collect::<Vec<_>>()
            .last()
            .expect("Empty line in input")
            .parse()
            .expect("End of input line could not be parsed as a u64")
    };

    let gen_a = DuelingGenerator {
        value: next_val(),
        factor: 16807,
        criteria_num: 4,
    };
    let gen_b = DuelingGenerator {
        value: next_val(),
        factor: 48271,
        criteria_num: 8,
    };
    println!("{}", judge(5_000_000, gen_a, gen_b));
}
