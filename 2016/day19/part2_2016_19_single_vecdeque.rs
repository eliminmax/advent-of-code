// SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 19 Part 2

// While this works, it is painfully slow.
const CIRCLE_SIZE: usize = include!("input");

fn main() {
    use std::collections::VecDeque;
    let mut gift_circle: VecDeque<_> = (1..=CIRCLE_SIZE).collect();
    while gift_circle.len() > 1 {
        gift_circle.remove(gift_circle.len() / 2);
        gift_circle.rotate_left(1);
    }
    println!("{}", gift_circle[0]);
}
