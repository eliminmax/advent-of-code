// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 12 Part 2
// at a certain point, the difference in score from generation to generation changes at a linear
// rate, so the approach I'm using is to first simulate up to 500 generations to find where that
// starts, and then extrapolate from there.

use std::collections::{BTreeMap, VecDeque};

type Ruleset = BTreeMap<[bool; 5], bool>;

const TEST_GENERATIONS: usize = 500;
const PADDING: usize = (TEST_GENERATIONS * 2) + 4;

fn score(tape: &[bool]) -> isize {
    tape.iter()
        .cloned()
        .enumerate()
        .map(|(i, b)| {
            if b {
                (i as isize) - (PADDING as isize)
            } else {
                0
            }
        })
        .sum::<isize>()
}

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

    let mut differences: VecDeque<isize> = VecDeque::new();
    let mut prev_score = score(&state[..]);
    let mut sim_stop: isize = 0;
    'sim_loop: for generation in 1..=TEST_GENERATIONS {
        let old_state = state.clone();
        for i in (PADDING - generation)..(PADDING + start_len + generation) {
            state[i] = *rules.get(&old_state[i - 2..=i + 2]).unwrap_or(&false);
        }
        let new_score = score(&state[..]);
        let difference = new_score - prev_score;
        differences.push_back(difference);
        while differences.len() == 5 {
            let test_val = differences.pop_front().unwrap_or_else(|| unreachable!());
            if differences.iter().all(|i| *i == test_val) {
                sim_stop = generation as isize;
                break 'sim_loop;
            }
        }
        prev_score = new_score;
        sim_stop = generation as isize;
    }
    assert!(differences[0] >= 0);
    let base_score = score(&state[..]);
    let remaining_gens = 50_000_000_000 - sim_stop;
    println!("{}", base_score + (remaining_gens * differences[0]));
}
