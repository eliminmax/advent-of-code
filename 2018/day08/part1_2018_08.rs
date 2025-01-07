// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 8 Part 1

use std::collections::VecDeque;
use std::env::args;
use std::fs::read_to_string;

fn metadata_sum(data: &mut VecDeque<u8>) -> u32 {
    let mut total: u32 = 0;
    let children = data.pop_front().expect("Data too short");
    let metadata_nodes = data.pop_front().expect("Data too short");
    for _ in 0..children {
        total += metadata_sum(data);
    }
    for _ in 0..metadata_nodes {
        total += data.pop_front().expect("Data too short") as u32;
    }
    total
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut license_data: VecDeque<u8> = input
        .split_whitespace()
        .map(|n| n.parse::<u8>().expect("Failed to parse input number"))
        .collect();
    println!("{}", metadata_sum(&mut license_data));
}
