// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 14 Part 1
use std::collections::HashMap;

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let (polymer, input) = input.split_once("\n\n").unwrap();
    let mut polymer = polymer.to_owned().into_bytes();
    let rules: HashMap<(u8, u8), u8> = (input.lines())
        .map(|line| {
            let &[a, b, b' ', b'-', b'>', b' ', out] = line.as_bytes() else {
                panic!("invalid line structure: {line}")
            };
            ((a, b), out)
        })
        .collect();

    for _ in 0..10 {
        let mut new_polymer = Vec::with_capacity(polymer.len() + (polymer.len() / 2));
        for pair in polymer.windows(2) {
            let &[a, b] = pair else { unreachable!() };
            new_polymer.push(a);
            if let Some(c) = rules.get(&(a, b)).copied() {
                new_polymer.push(c);
            }
        }
        new_polymer.push(*polymer.last().unwrap());
        polymer = new_polymer
    }

    let mut counts: HashMap<u8, u32> = HashMap::new();
    for elem in polymer {
        counts
            .entry(elem)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut min_count = u32::MAX;
    let mut max_count = u32::MIN;
    for count in counts.into_values() {
        min_count = min_count.min(count);
        max_count = max_count.max(count);
    }

    println!("{}", max_count - min_count);
}
