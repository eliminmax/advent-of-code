// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 14 Part 1


fn knot_hash(lengths: &[u8]) -> [u8; 16] {
    use std::collections::VecDeque;
    use std::iter::FromIterator;
    let mut knot_string = VecDeque::from_iter(0u8..=255);
    let mut current_pos: usize = 0;
    let mut skip_size: usize = 0;
    const EXTENSION: [u8; 5] = [17, 31, 73, 47, 23];

    // create the "sparse hash"
    for _ in 0..64 {
        for length in lengths.iter().cloned().chain(EXTENSION) {
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
            skip_size += 1;
        }
    }
    knot_string.rotate_right(current_pos % knot_string.len());
    // create and return the "dense hash"
    let knot_string: Vec<_> = knot_string.into();
    let mut chunks = knot_string.chunks_exact(16);
    core::array::from_fn(|_| {
        chunks
            .next()
            .unwrap_or_else(|| unreachable!())
            .iter()
            .fold(0, |acc, x| acc ^ x)
    })
}

fn main() {
    const INPUT: &str = include_str!("input").trim_ascii_end();
    let mut total = 0u32;
    for i in 0..128 {
        let hash = knot_hash(format!("{INPUT}-{i}").as_bytes());
        hash.iter().for_each(|byte| total += byte.count_ones());
    }
    println!("{total}");
}
