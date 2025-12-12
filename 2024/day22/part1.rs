// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 22 Part 1

use std::env::args;
use std::fs::read_to_string;

fn mix_and_prune(secret: u64, num: u64) -> u64 {
    (secret ^ num) % 16_777_216
}

fn next_secret(secret: u64) -> u64 {
    let secret = mix_and_prune(secret, secret * 64);
    let secret = mix_and_prune(secret, secret / 32);
    mix_and_prune(secret, secret * 2048)
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut states: Vec<u64> = input
        .lines()
        .map(|line| line.trim().parse().expect("Failed to parse line"))
        .collect();
    (0..2000).for_each(|_| {
        states
            .iter_mut()
            .for_each(|secret| *secret = next_secret(*secret));
    });
    println!("{}", states.into_iter().sum::<u64>());
}
