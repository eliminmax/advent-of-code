// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 17 Part 1

fn spinlock_breaker() -> u16 {
    use std::collections::VecDeque;
    let mut spinlock: VecDeque<u16> = VecDeque::with_capacity(2018);
    const MOVE_BY: usize = include!("input");
    spinlock.push_back(0);
    for i in 1u16..=2017 {
        spinlock.rotate_left(MOVE_BY % spinlock.len());
        spinlock.push_back(i);
    }
    spinlock[0]
}

fn main() {
    println!("{}", spinlock_breaker());
}
