// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 07 Part 2
use std::collections::HashMap;

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut lines = input.lines();
    let mut positions = HashMap::from([(
        lines
            .next()
            .unwrap()
            .bytes()
            .position(|b| b == b'S')
            .unwrap(),
        1,
    )]);

    for line in lines.map(str::as_bytes).filter(|l| l.contains(&b'^')) {
        let mut new_positions = HashMap::with_capacity(positions.len() * 2);
        for (&p, &t) in positions.iter() {
            macro_rules! beam_at {
                ($pos: expr) => {{
                    new_positions
                        .entry($pos)
                        .and_modify(|e| *e += t)
                        .or_insert(t);
                }};
            }
            if line[p] == b'^' {
                beam_at!(p - 1);
                beam_at!(p + 1);
            } else {
                beam_at!(p);
            }
        }
        positions = new_positions;
    }
    println!("{}", positions.into_values().sum::<u64>());
}
