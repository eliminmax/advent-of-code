// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 18 Part 2

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Safe,
    Trapped,
}

fn row_after(row: &[Tile]) -> Vec<Tile> {
    use Tile::{Safe, Trapped};
    let mut padded_row = vec![Safe];
    padded_row.extend_from_slice(row);
    padded_row.push(Safe);
    padded_row
        .windows(3)
        .map(|w| if w[0] != w[2] { Trapped } else { Safe })
        .collect()
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut current_row: Vec<_> = input
        .trim()
        .chars()
        .map(|c| match c {
            '.' => Tile::Safe,
            '^' => Tile::Trapped,
            _ => panic!("invalid tile character: {:?}", c),
        })
        .collect();
    let mut total: usize = 0;
    for _ in 0..400000 {
        total += current_row.iter().filter(|&i| *i == Tile::Safe).count();
        current_row = row_after(&current_row[..]);
    }
    println!("{total}");
}
