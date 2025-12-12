// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 10 Part 1

fn knot_hash(lengths: &[u8]) -> u16 {
    use std::collections::VecDeque;
    use std::iter::FromIterator;
    let mut knot_string = VecDeque::from_iter(0u16..=255);
    let mut current_pos: usize = 0;
    // clippy pointed out that skip_size is being used as a loop counter, even if that wasn't how I
    // was thinking about it. Thanks, clippy!
    for (skip_size, length) in lengths.iter().cloned().enumerate() {
        let length: usize = length.into();
        // collecting then calling into_iter to avoid keeping a reference to knot_string
        let mut reversed = knot_string
            .range(..length)
            .cloned()
            .rev()
            .collect::<Vec<_>>()
            .into_iter();
        knot_string
            .range_mut(..length)
            .for_each(|v| *v = reversed.next().unwrap_or_else(|| unreachable!()));
        current_pos += skip_size + length;
        knot_string.rotate_left((skip_size + length) % knot_string.len());
    }
    knot_string.rotate_right(current_pos % knot_string.len());
    knot_string[0] * knot_string[1]
}

fn main() {
    let lengths: Vec<u8> = include_str!("input")
        .trim()
        .split(',')
        .map(|s| s.parse().expect("Failed to parse number from input"))
        .collect();
    println!("{}", knot_hash(&lengths[..]));
}
