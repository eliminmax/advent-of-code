// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 14 Part 2

use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

// I assumed this number would change for part 2. I was wrong.
const RACE_TIME: u16 = 2503;

#[derive(Debug)]
struct Reindeer {
    km_s: u32,
    travel_time: u32,
    rest_time: u32,
    resting: bool,
    distance: u32,
    score: u32,
    timer: u32,
}

#[derive(Debug)]
struct ReindeerParseError;
impl FromStr for Reindeer {
    type Err = ReindeerParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words.len() < 14 {
            return Err(ReindeerParseError);
        }
        let km_s = words[3].parse::<u32>().map_err(|_| ReindeerParseError)?;
        let travel_time = words[6].parse::<u32>().map_err(|_| ReindeerParseError)?;
        let rest_time = words[13].parse::<u32>().map_err(|_| ReindeerParseError)?;
        Ok(Reindeer {
            km_s,
            travel_time,
            rest_time,
            resting: false,
            distance: 0,
            score: 0,
            timer: 0,
        })
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let mut reindeers: Vec<Reindeer> = input
        .lines()
        .map(|l| Reindeer::from_str(l).expect("Failed to parse reindeer"))
        .collect();
    let mut max_dist = 0u32;
    for _ in 0..RACE_TIME {
        // 2 passes per round - first updates reindeer distances, second updates reindeer scores
        for reindeer in reindeers.iter_mut() {
            let time_limit: u32 = if reindeer.resting {
                reindeer.rest_time
            } else {
                reindeer.distance += reindeer.km_s;
                max_dist = max_dist.max(reindeer.distance);
                reindeer.travel_time
            };
            reindeer.timer += 1;
            if reindeer.timer == time_limit {
                reindeer.timer = 0;
                reindeer.resting = !reindeer.resting;
            }
        }
        for reindeer in reindeers.iter_mut() {
            if reindeer.distance == max_dist {
                reindeer.score += 1;
            }
        }
    }
    println!(
        "{}",
        reindeers
            .into_iter()
            .map(|r| r.score)
            .max()
            .expect("Reindeer vec empty!")
    );
}
