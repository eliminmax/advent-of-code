// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 9 Part 2

use std::env::args;
use std::fs::read_to_string;

/// Remove all garbage, and all characters other than `{` and `}`.
fn count_garbage(stream: String) -> u64 {
    let mut total: u64 = 0;
    let mut stream_chars = stream.chars();
    while let Some(c) = stream_chars.next() {
        if c == '<' {
            'garb: while let Some(garbage) = stream_chars.next() {
                match garbage {
                    '!' => _ = stream_chars.next(),
                    '>' => break 'garb,
                    _ => total += 1,
                }
            }
        }
    }
    total
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    println!("{}", count_garbage(input));
}
