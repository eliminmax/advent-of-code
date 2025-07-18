// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 09 Part 1

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let lines: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

    'main_loop: for window in lines.windows(26) {
        let candidates: Vec<u64> = window[..25].iter().cloned().filter(|i| *i < window[25]).collect();
        for i in 0..candidates.len() {
            for j in (i + 1)..candidates.len() {
                if candidates[i] + candidates[j] == window[25] {
                    continue 'main_loop;
                }
            }
        }
        println!("{}", window[25]);
        break 'main_loop;
    }
}
