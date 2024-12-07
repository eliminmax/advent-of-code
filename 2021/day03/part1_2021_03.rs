// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 3 Part 1

use std::env::args;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let vals: Vec<u16> = input
        .lines()
        .map(|s| u16::from_str_radix(s, 2).expect("Failed to parse as binary integer string"))
        .collect();
    let bits: u32 = vals
        .iter()
        .map(|i| 16 - i.leading_zeros())
        .max()
        .expect("vals empty");
    let mut epsilon: u16 = 0;
    let mut gamma: u16 = 0;
    let cutoff: usize = vals.len() / 2;

    for bit in 0..bits {
        let mask = 1 << bit;
        if vals.iter().filter(|i| (*i & mask) != 0).count() > cutoff {
            gamma += mask;
        } else {
            epsilon += mask;
        }
    }
    // overflows on my input if not cast here
    println!("{}", gamma as u32 * epsilon as u32);
}
