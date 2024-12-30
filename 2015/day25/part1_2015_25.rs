// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 25 Part 1

use std::env::args;
use std::fs::read_to_string;

fn next_code(prev: u64) -> u64 {
    (prev * 252533) % 33554393
}

fn location_number(row: u32, col: u32) -> u32 {
    let col_start = (1..=col).sum();
    let mut result = col_start;
    let mut next_offset = col;
    for _ in 1..row {
        result += next_offset;
        next_offset += 1;
    }
    result
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut words = input.split_whitespace().skip(15);
    let row: u32 = words
        .next()
        .expect("Missing row number in input")
        .strip_suffix(',')
        .expect("Row missing expected suffix.")
        .parse()
        .expect("Failed to parse row number");
    let _ = words.next();
    let col: u32 = words
        .next()
        .expect("Missing column number in input")
        .strip_suffix('.')
        .expect("Column missing expected suffix.")
        .parse()
        .expect("Failed to parse column number");
    let mut code = 20151125;
    for _ in 1..location_number(row, col) {
        code = next_code(code);
    }
    println!("{code}");
}
