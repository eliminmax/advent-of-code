// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 19 Part 2

const CONSTANT: u64 = include!("constant");

fn main() {
    println!(
        "{}",
        (1..=CONSTANT).filter(|i| CONSTANT % *i == 0).sum::<u64>()
    );
}
