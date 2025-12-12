// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2023 Day 6 Part 2

use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

fn distance_gone(held_time: u64, race_time: u64) -> u64 {
    if race_time < held_time {
        0
    } else {
        let remaining_time = race_time - held_time;
        held_time * remaining_time
    }
}

fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    // I have a song about the quadratic formula from 9th grade stuck in my head now.
    let sqrt_part = f64::sqrt((b * b) - (4f64 * a * c));
    if sqrt_part.is_nan() {
        None
    } else {
        Some(((-b - sqrt_part) / (2f64 * a), (-b + sqrt_part) / (2f64 * a)))
    }
}

fn possible_wins(time: u64, dist: u64) -> u64 {
    // playing with desmos.com/calculator, I determined that asking which charge times will beat a
    // distance `d` with a race time of `d` seems to be equivalent to asking what integers fall
    // between the zeroes of the parabola defined with `-x² + tx - d`.
    // Unfortunately, I accidentally closed the tab before I could save my work.
    let bounds =
        quadratic(-1.0, time as f64, -(dist as f64)).expect("Unbeatable distance/time combo");

    let (fuzzy_upper, fuzzy_lower) = if bounds.0 > bounds.1 {
        (bounds.0, bounds.1)
    } else {
        (bounds.1, bounds.0)
    };
    // if above 2⁵³ - 1, it's precision is less than an integer, and it can't simply be used
    // directly, but given that distance is around an order of magnitude or two less than that (at
    // least for my input), I didn't think I'd need to worry about that, but I was getting results
    // that were off by one for much smaller test inputs due to rounding issues.
    // It still cuts the search range down massively.
    let approx_lower = fuzzy_lower.ceil() as u64;
    let approx_upper = fuzzy_upper.floor() as u64;
    let mut lower: u64 = u64::MAX;
    let mut upper: u64 = u64::MIN;
    'find_lower: for i in (approx_lower.saturating_sub(2))..(approx_lower + 2) {
        if distance_gone(i, time) > dist {
            lower = i;
            break 'find_lower;
        }
    }
    'find_upper: for i in (approx_upper.saturating_sub(2))..(approx_upper + 2) {
        if distance_gone(i, time) <= dist {
            upper = i;
            break 'find_upper;
        }
    }
    upper - lower
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let (time_str, distance_str) = input
        .split_once("\n")
        .expect("Input can't be all on one line");

    // parse only the digits as u64 ints
    let time = u64::from_str(
        &time_str
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>(),
    )
    .expect("Failed to parse time as u64");

    let distance = u64::from_str(
        &distance_str
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>(),
    )
    .expect("Failed to parse distance as u64");
    println!("{}", possible_wins(time, distance));
}
