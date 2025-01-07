// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 9 Part 2

use std::collections::VecDeque;
use std::env::args;
use std::fs::read_to_string;
use std::num::NonZero;

fn winning_total(players: NonZero<usize>, points: NonZero<u32>) -> u32 {
    let mut player_order = (0..players.into()).cycle();
    let mut marbles: VecDeque<u32> = VecDeque::from([0]);
    let mut scores: Vec<u32> = [0].repeat(players.into());
    for marble in 1..=(points.into()) {
        let player = player_order.next().expect("Cycle should always be Some");
        if marble % 23 != 0 {
            marbles.rotate_left(1);
            marbles.push_back(marble);
        } else {
            marbles.rotate_right(8);
            scores[player] += marble;
            scores[player] += marbles.pop_front().expect("There will always be an element here");
            marbles.rotate_left(1);
        }
    }
    scores.into_iter().max().expect("scores is non-empty")
}

fn main() {
    const { assert!(usize::BITS >= 32) };
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let words: Vec<&str> = input.split_whitespace().collect();
    let players: NonZero<usize>;
    let points: NonZero<u32>;
    if let &[a, "players;", "last", "marble", "is", "worth", b, "points"] = &words[..] {
        players = NonZero::<usize>::new(
            a.parse::<usize>()
                .expect("Failed to parse player count as a u32"),
        )
        .expect("Can't work with zero players");
        points = NonZero::<u32>::new(
            100 * b.parse::<u32>()
                .expect("Failed to parse point total as a u32"),
        )
        .expect("Can't work with zero marbles");
    } else {
        panic!("Invalid input format: {:?}", input);
    }
    println!("{}", winning_total(players, points));
}
