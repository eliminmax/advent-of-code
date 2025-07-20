// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 13 Part 1

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut lines = input.lines();
    let ts: u32 = lines.next().unwrap().parse().unwrap();
    let buses: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|s| s.parse().ok())
        .collect();

    assert!(lines.next().is_none());

    let next_bus = buses.into_iter().min_by_key(|b| b - (ts % b)).unwrap();
    println!("{}", next_bus * (ts.next_multiple_of(next_bus) - ts));
}
