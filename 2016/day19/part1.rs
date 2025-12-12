// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 19 Part 1

const CIRCLE_SIZE: usize = include!("input");

fn main() {
    use std::collections::VecDeque;
    let mut gift_circle: VecDeque<_> = (1..=CIRCLE_SIZE).collect();
    while gift_circle.len() > 1 {
        gift_circle.rotate_left(1);
        let _ = gift_circle.pop_front();
    }
    println!("{}", gift_circle[0]);
}
