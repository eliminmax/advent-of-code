// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 22 Part 2

use std::env::args;
use std::fs::read_to_string;
use std::collections::{HashMap, HashSet, VecDeque};

fn mix_and_prune(secret: u64, num: u64) -> u64 {
    (secret ^ num) % 16_777_216
}

fn next_secret(secret: u64) -> u64 {
    let secret = mix_and_prune(secret, secret * 64);
    let secret = mix_and_prune(secret, secret / 32);
    mix_and_prune(secret, secret * 2048)
}

fn cost(secret: u64) -> i8 {
    (secret % 10) as i8
}

fn find_vals_for(starting_secret: u64, vals: &mut HashMap<[i8; 4], u64>) {
    let mut seen_seqs: HashSet<[i8; 4]> = HashSet::new();
    let mut seq_tracker: VecDeque<i8> = VecDeque::with_capacity(5);
    let mut secret = starting_secret;

    for _ in 0..=2000 {
        let current_cost = cost(secret);
        seq_tracker.push_back(current_cost);
        if seq_tracker.len() == 5 {
            let seq: [i8; 4] = core::array::from_fn(|i| seq_tracker[i + 1] - seq_tracker[i]);
            if !seen_seqs.contains(&seq) {
                seen_seqs.insert(seq);
                vals.entry(seq).and_modify(|e| *e += current_cost as u64).or_insert(current_cost as u64);
            }
            seq_tracker.pop_front();
        }
        secret = next_secret(secret);
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let starting_secrets: Vec<u64> = input
        .lines()
        .map(|line| line.trim().parse().expect("Failed to parse line"))
        .collect();

    let mut sequences: HashMap<[i8; 4], u64> = HashMap::new();

    for secret in starting_secrets.into_iter() {
        find_vals_for(secret, &mut sequences);
    }

    println!("{}", sequences.into_values().max().expect("failed to obtain max value"));
}
