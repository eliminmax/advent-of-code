// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 14 Part 1

use std::env::args;
use std::fs::read_to_string;

const RACE_TIME: u32 = 2503;

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let mut max_dist = 0u32;
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let km_s = words[3].parse::<u32>().expect("Failed to parse speed");
        let travel_time = words[6]
            .parse::<u32>()
            .expect("Failed to parse travel time");
        let rest_time = words[13].parse::<u32>().expect("Failed to parse rest time");

        let combined_time = travel_time + rest_time; // amount of time needed for a full cycle
        let mut distance = (travel_time * km_s) * (RACE_TIME / combined_time);
        distance += travel_time.min(RACE_TIME % combined_time) * km_s;
        max_dist = max_dist.max(distance);
    }
    println!("{max_dist}");
}
