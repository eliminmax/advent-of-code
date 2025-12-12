// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 13 Part 2
#[derive(Debug)]
struct Scanner {
    depth: u32,
    span: u32,
}

impl Scanner {
    fn is_safe_with_delay(&self, picoseconds: u32) -> bool {
        (self.depth + picoseconds) % self.span != 0
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let mut scanners: Vec<Scanner> = Vec::new();
    for line in input.lines() {
        let (depth, range) = line
            .split_once(": ")
            .expect("Failed to split line on \": \"");
        let depth: u32 = depth.parse().expect("Failed to parse depth");
        let span = (range.parse::<u32>().expect("Failed to parse range") - 1) * 2;
        scanners.push(Scanner { depth, span });
    }

    for i in 0u32.. {
        if scanners.iter().all(|scanner| scanner.is_safe_with_delay(i)) {
            println!("{i}");
            break;
        }
    }
}
