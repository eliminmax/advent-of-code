// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 8 Part 1

use std::env::args;
use std::fs::read_to_string;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let input: Vec<u8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit in input") as u8)
        .collect();

    let layers: Vec<&[u8]> = input.chunks_exact(WIDTH * HEIGHT).collect();

    let min_index: usize = layers
        .iter()
        .enumerate()
        .min_by_key(|(_index, layer)| (layer).iter().filter(|&i| *i == 0).count())
        .expect("No layers in input")
        .0;

    let num_1s = layers[min_index].iter().filter(|&i| *i == 1).count();
    let num_2s = layers[min_index].iter().filter(|&i| *i == 2).count();

    println!("{}", num_1s * num_2s);
}
