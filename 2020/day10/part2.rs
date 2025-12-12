// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 10 Part 2

use std::collections::BTreeMap;

fn count_paths(links: impl IntoIterator<Item = u8>) -> u64 {
    let mut joltages: BTreeMap<u8, Option<u64>> = BTreeMap::new();
    joltages.insert(0, Some(1));
    for link in links {
        if joltages.insert(link, None).is_some() {
            panic!("Duplicate link {link}");
        }
    }
    let max_key = joltages.keys().copied().last().unwrap_or_default() + 3;
    joltages.insert(max_key, None);

    let keys: Vec<_> = joltages.keys().copied().skip(1).collect();
    for key in keys {
        let possible_paths: u64 = [key.checked_sub(1), key.checked_sub(2), key.checked_sub(3)]
            .into_iter()
            .flatten()
            .map(|i| joltages.get(&i).copied().flatten().unwrap_or_default())
            .sum();
        joltages.insert(key, Some(possible_paths));
    }

    joltages
        .get(&max_key)
        .copied()
        .flatten()
        .unwrap_or_default()
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    println!("{}", count_paths(input.lines().map(|l| l.parse().unwrap())));
}
