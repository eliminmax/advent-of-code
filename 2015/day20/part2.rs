// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 20 Part 2

// I hate this solution, but it works.

use std::env::args;
use std::fs::read_to_string;
use std::iter;

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let target_number = input
        .trim()
        .parse::<usize>()
        .expect("Failed to parse target number");

    let mut first_100_mil: Vec<usize> = iter::repeat(0usize).take(100_000_000).collect();

    let mut elf = 1usize;
    for _ in 0..1_000_000 {
        for house in (elf..=(elf * 50)).step_by(elf) {
            first_100_mil[house] += elf * 11;
        }
        elf += 1;
    }
    for (i, presents) in first_100_mil.into_iter().enumerate() {
        if presents >= target_number {
            println!("{}", i);
            break;
        }
    }
}
