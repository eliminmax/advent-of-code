// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 15 Part 2

use std::collections::HashMap;

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut spoken: HashMap<usize, usize> = HashMap::new();

    let mut next_number: usize = 0;
    for (n, i) in input
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .enumerate()
        .map(|(i, n)| (n, i))
    {
        if let Some(prev) = spoken.insert(n, i) {
            next_number = i - prev;
        } else {
            next_number = 0;
        }
    }

    let starting_len = spoken.len();

    for i in starting_len..29999999 {
        if let Some(prev) = spoken.insert(next_number, i) {
            next_number = i - prev;
        } else {
            next_number = 0;
        }
    }
    println!("{next_number}");
}
