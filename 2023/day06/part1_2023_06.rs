// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2023 Day 6 Part 1

use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Race {
    dist: u32,
    ms: u32,
}

fn distance_gone(held_time: u32, race_time: u32) -> u32 {
    if race_time < held_time {
        0
    } else {
        let remaining_time = race_time - held_time;
        held_time * remaining_time
    }
}

fn get_win_margin(race: Race) -> usize {
    (0..race.ms)
        .filter(|t| distance_gone(*t, race.ms) > race.dist)
        .count()
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let (time_str, distance_str) = input
        .split_once("\n")
        .expect("Input can't be all on one line");

    // skip past the labels
    let times = time_str.split_whitespace().skip(1);
    let mut distances = distance_str.split_whitespace().skip(1);
    println!(
        "{}",
        times.map(|ms| Race {
            ms: u32::from_str(ms).expect("Failed to parse time as u64"),
            dist: u32::from_str(
                distances
                    .next()
                    .expect("Found a time without an associated distance"),
            )
            .expect("Failed to parse distance as u64"),
        }).map(get_win_margin).product::<usize>()
    );
}
