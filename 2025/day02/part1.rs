// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 02 Part 2

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let ranges = input.trim().split(',').map(|rstr| {
        rstr.split_once('-')
            .map(|(a, b)| a.parse::<u64>().unwrap()..=b.parse().unwrap())
            .unwrap()
    });
    let mut total = 0;
    for range in ranges {
        for (i, s) in range.map(|i| (i, i.to_string())) {
            let l = s.len();
            if l % 2 == 0 && s[..(l / 2)] == s[(l / 2)..] {
                total += i;
            }
        }
    }
    println!("{total}");
}
