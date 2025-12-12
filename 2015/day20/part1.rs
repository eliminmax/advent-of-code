// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 20 Part 1

use std::env::args;
use std::fs::read_to_string;

/// number of presents is the sum of the factors of the number, all multiplied by 10.
fn present_count(house: u32) -> u32 {
    let mut sum = house + 1;
    for i in 2..=(house / 2) {
        if house % i == 0 {
            sum += i;
        }
    }
    sum * 10
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let target_number = input
        .trim()
        .parse::<u32>()
        .expect("Failed to parse target number");
    let mut house = 1u32;
    while present_count(house) < target_number {
        house += 1;
    }
    println!("{house}");
}
