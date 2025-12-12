// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 25 Part 1

#[cfg(aoc_direct)]
const INPUT: &str = include_str!("input");
#[cfg(not(aoc_direct))]
const INPUT: &str = include_str!("../input");

const fn loop_num(pub_key: u64) -> u64 {
    let mut i = 0;
    let mut current_num = 1;
    while current_num != pub_key {
        current_num *= 7;
        current_num %= 20201227;
        i += 1
    }
    i
}

const fn transform(subj_num: u64, loop_num: u64) -> u64 {
    let mut i = 0;
    let mut current_num = 1;
    while i < loop_num {
        current_num *= subj_num;
        current_num %= 20201227;
        i += 1;
    }
    current_num
}

const fn crack(keys: [u64; 2]) -> u64 {
    let loop_counts = [loop_num(keys[0]), loop_num(keys[1])];
    let enc_keys = [
        transform(keys[0], loop_counts[1]),
        transform(keys[1], loop_counts[0]),
    ];
    assert!(enc_keys[0] == enc_keys[1]);
    enc_keys[0]
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

fn main() {
    println!("{}", crack(KEYS));
}
