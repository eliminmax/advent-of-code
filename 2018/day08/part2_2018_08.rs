// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 8 Part 2

use std::collections::VecDeque;
use std::env::args;
use std::fs::read_to_string;

fn node_value(data: &mut VecDeque<u8>) -> u32 {
    let children = data.pop_front().expect("Data too short");
    let metadata_nodes = data.pop_front().expect("Data too short");
    if children == 0 {
        (0..metadata_nodes)
            .map(|_| data.pop_front().expect("Data too short") as u32)
            .sum()
    } else {
        let child_values: Vec<u32> = (0..children).map(|_| node_value(data)).collect();
        (0..metadata_nodes)
            .map(|_| data.pop_front().expect("Data too short") as usize - 1)
            .filter_map(|i| child_values.get(i))
            .sum()
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut license_data: VecDeque<u8> = input
        .split_whitespace()
        .map(|n| n.parse::<u8>().expect("Failed to parse input number"))
        .collect();
    println!("{}", node_value(&mut license_data));
}
