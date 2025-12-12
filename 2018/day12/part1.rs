// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 12 Part 1

use std::collections::BTreeMap;

type Ruleset = BTreeMap<[bool; 5], bool>;

const GENERATIONS: usize = 20;
const PADDING: usize = (GENERATIONS * 2) + 4;

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut state = vec![false; PADDING];
    let mut lines = input.lines();
    let seed_str = lines
        .next()
        .and_then(|line| line.split_whitespace().last())
        .expect("Missing initial state");
    let start_len = seed_str.len();
    state.extend(seed_str.bytes().map(|c| c == b'#'));
    state.extend_from_within(..PADDING);
    let mut rules = Ruleset::new();

    let _ = lines.next();
    for line in lines {
        let (rule, next_gen) = line
            .split_once(" => ")
            .expect("Line delimiter should be present");
        let rule: [bool; 5] = core::array::from_fn(|i| rule.as_bytes()[i] == b'#');
        let next_gen: bool = next_gen == "#";
        assert!(rules.insert(rule, next_gen).is_none());
    }

    assert!(!rules.get(&[false; 5]).unwrap_or(&false));

    for generation in 1..=GENERATIONS {
        let old_state = state.clone();
        for i in (PADDING - generation)..(PADDING + start_len + generation) {
            state[i] = *rules.get(&old_state[i - 2..=i + 2]).unwrap_or(&false);
        }
    }
    println!(
        "{}",
        state
            .into_iter()
            .enumerate()
            .map(|(i, b)| if b {
                (i as isize) - (PADDING as isize)
            } else {
                0
            })
            .sum::<isize>()
    );
}
