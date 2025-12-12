// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 10 Part 1

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut joltages: Vec<u8> = input.lines().map(|l| l.parse().unwrap()).collect();
    joltages.push(0);
    joltages.sort();
    joltages.push(joltages.last().copied().unwrap_or_default() + 3);
    let mut tally: [u16; 3] = [0; 3];
    for w in joltages.windows(2) {
        tally[usize::from(w[1] - w[0]) - 1] += 1;
    }
    println!("{}", tally[0] * tally[2]);
}
