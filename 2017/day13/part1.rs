// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 13 Part 1

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut severity: u32 = 0;
    for line in input.lines() {
        let (depth, range) = line
            .split_once(": ")
            .expect("Failed to split line on \": \"");
        let depth: u32 = depth.parse().expect("Failed to parse depth");
        let range = range.parse::<u32>().expect("Failed to parse range");
        let span = (range - 1) * 2;
        if depth % span == 0 {
            severity += depth * range
        }
    }
    println!("{severity}");
}
