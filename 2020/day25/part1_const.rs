// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 25 Part 1

#![allow(
    long_running_const_eval,
    reason = "Solving at compile time takes a good 10+ seconds on my system"
)]

#[cfg(aoc_direct)]
const INPUT: &str = include_str!("input");
#[cfg(not(aoc_direct))]
const INPUT: &str = include_str!("../input");

const fn loop_num(pub_key: u64) -> u64 {
    let mut i = 0;
    let mut current = 1;
    while current != pub_key {
        i += 1;
        current *= 7;
        current %= 20201227;
    }
    i
}

const fn transform(mut subj_num: u64, mut loop_num: u64) -> u64 {
    let mut y = 1;
    while loop_num > 0 {
        if loop_num % 2 == 1 {
            y *= subj_num;
            y %= 20201227;
        }
        subj_num *= subj_num;
        subj_num %= 20201227;
        loop_num /= 2;
    }
    y
}

// compile-time parsing of input numbers, using only const functions
const KEYS: [u64; 2] = {
    let trimmed = INPUT.trim_ascii().as_bytes();
    let mut a: u64 = 0;
    let mut b: u64 = 0;
    let mut i = 0;
    while trimmed[i] != b'\n' {
        assert!(trimmed[i] >= b'0' && trimmed[i] <= b'9');
        a *= 10;
        a += (trimmed[i] - b'0') as u64;
        i += 1;
    }
    i += 1;

    while i < trimmed.len() {
        assert!(trimmed[i] >= b'0' && trimmed[i] <= b'9');
        b *= 10;
        b += (trimmed[i] - b'0') as u64;
        i += 1;
    }
    [a, b]
};

const LOOP_NUM: u64 = loop_num(KEYS[0]);

const ANSWER: u64 = transform(KEYS[1], LOOP_NUM);

fn main() {
    println!("{ANSWER}");
}
