// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 23 Part 2

// See part2_2017_23.md for explanation of how I whittled down the input into this

fn is_composite(n: u32) -> bool {
    for i in 2..(n / 2) {
        if n % i == 0 {
            return true;
        }
    }
    false
}

fn main() {
    const START: u32 = include!("start");
    const END: u32 = include!("end");
    const INCREMENT: usize = include!("increment");
    let mut total = 0;
    for n in (START..=END).step_by(INCREMENT) {
        if is_composite(n) {
            total += 1;
        }
    }
    println!("{total}");
}
