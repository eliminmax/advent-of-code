// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 17 Part 2

fn spinlock_breaker() -> u32 {
    use std::collections::VecDeque;
    let mut spinlock: VecDeque<u32> = VecDeque::with_capacity(2018);
    const MOVE_BY: usize = include!("input");
    spinlock.push_back(0);
    for i in 1u32..=50_000_000 {
        spinlock.rotate_left(MOVE_BY % spinlock.len());
        spinlock.push_back(i);
    }
    spinlock.into_iter().skip_while(|&i| i != 0).nth(1).unwrap_or_else(|| unreachable!())
}

fn main() {
    println!("{}", spinlock_breaker());
}
