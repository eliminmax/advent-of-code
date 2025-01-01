// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 18 Part 1

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
    let mut rows: Vec<Vec<Tile>> = vec![input
        .trim()
        .chars()
        .map(|c| match c {
            '.' => Tile::Safe,
            '^' => Tile::Trapped,
            _ => panic!("invalid tile character: {:?}", c),
        })
        .collect()];
    while rows.len() < 40 {
        rows.push(row_after(
            &rows
                .last()
                .unwrap_or_else(|| unreachable!("rows known to be non-empty"))[..],
        ));
    }
    println!(
        "{}",
        rows.into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|i| *i == Tile::Safe)
            .count()
    );
}
