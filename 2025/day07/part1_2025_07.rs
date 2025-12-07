// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 07 Part 1
use std::collections::HashSet;

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut lines = input.lines();
    let mut positions = HashSet::from([lines
        .next()
        .unwrap()
        .bytes()
        .position(|b| b == b'S')
        .unwrap()]);
    let mut total_splits = 0;
    for line in lines.map(str::as_bytes).filter(|l| l.contains(&b'^')) {
        let mut new_positions = HashSet::with_capacity(positions.len() * 2);
        for p in positions.iter().copied() {
            if line[p] == b'^' {
                new_positions.extend([p - 1, p + 1]);
                total_splits += 1;
            } else {
                new_positions.insert(p);
            }
        }
        positions = new_positions;
    }
    println!("{total_splits}");
}
