// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 01 Part 1

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut pass = 0u16;
    let mut position: i16 = 50;
    for line in input.lines() {
        let direction_byte = line.as_bytes()[0];
        let shift_amount: i16 = line[1..].parse().expect("valid number within range");
        match direction_byte {
            b'R' => position += shift_amount,
            b'L' => position -= shift_amount,
            c => panic!("invalid direction: {}", c as char),
        }
        position = position.rem_euclid(100);
        if position == 0 {
            pass += 1;
        }
    }
    println!("{pass}");
}
