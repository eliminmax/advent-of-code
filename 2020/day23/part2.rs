// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 23 Part 2

// Need to simulate 1,000,000 cups for 10,000,000 moves.
//
// Actually simulating it with a VecDeque for 1,000 rounds takes around 1.35 seconds on my system,
// and going for 10,000 rounds takes around 13.5 seconds, so I'd expect it to scale linearly.
//
// Doing some quick math, it'll take around 3.75 hours to finish all 10,000,000 rounds at a rate of
// around 0.00135 seconds per round.
//
// It's getting late at time of writing, so I'm going to leave it running overnight, and check on
// it in the morning. Whether it works or not, I will figure out how to do it right tomorrow, and
// include both this and the proper version in the repo.

use std::collections::VecDeque;

fn run_round(cups: &mut VecDeque<u32>) {
    let start = cups.pop_front().unwrap();
    cups.push_back(start);
    let held = [
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
    ];

    let mut rotation = 0;

    let mut target_point = start - 1;
    if target_point == 0 {
        target_point = 1_000_000;
    }
    while held.contains(&target_point) {
        target_point -= 1;
        if target_point == 0 {
            target_point = 1_000_000;
        }
    }

    loop {
        let popped = cups.pop_front().unwrap();
        cups.push_back(popped);
        rotation += 1;
        if popped == target_point {
            rotation += 3;
            cups.extend(held);
            break;
        }
    }
    cups.rotate_right(rotation);
    debug_assert_eq!(cups.len(), 1_000_000);
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut cups: VecDeque<u32> = VecDeque::with_capacity(1_000_000);

    for c in input.trim().chars() {
        cups.push_back(c.to_digit(10).unwrap());
    }
    let start = u32::try_from(cups.len() + 1).unwrap();
    cups.extend(start..=1_000_000);
    assert_eq!(cups.len(), 1_000_000);

    for _ in 0..10_000_000 {
        run_round(&mut cups);
    }

    loop {
        let i = cups.pop_front().unwrap();
        cups.push_back(i);
        if i == 1 {
            break;
        }
    }

    let answer = u64::from(cups[0]) * u64::from(cups[1]);
    dbg!(cups);
    println!("{answer}");
}
