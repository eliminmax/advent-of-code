// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 10 Part 1

use std::env::args;
use std::fs::read_to_string;

/// Takes a u8, interprets it as an ASCII digit character, and returns its numeric value.
/// Panics if it's not in the range 0x30..=0x39 (i.e. b'0'..=b'9')
fn u8_digit_to_value(c: u8) -> u8 {
    assert!(c.is_ascii_digit());
    c - b'0'
}

fn find_end_positions(
    row: usize,
    col: usize,
    grid: &Vec<Vec<u8>>,
    next: u8,
) -> Vec<(usize, usize)> {
    match grid.get(row).and_then(|row_vec| row_vec.get(col)) {
        Some(i) if *i == next => {
            if next == 9 {
                vec![(row, col)]
            } else {
                let mut vec = match (row, col) {
                    (0, 0) => vec![(0, 1), (1, 0)],
                    (0, c) => vec![(0, c + 1), (0, c - 1), (1, c)],
                    (r, 0) => vec![(r + 1, 0), (r - 1, 0), (r, 1)],
                    (r, c) => vec![(r, c + 1), (r, c - 1), (r + 1, c), (r - 1, c)],
                }
                .into_iter()
                .map(|(r, c)| find_end_positions(r, c, grid, next + 1))
                .collect::<Vec<Vec<(usize, usize)>>>()
                .concat();
                vec.sort();
                vec.dedup();
                vec
            }
        }
        _ => vec![],
    }
}

fn main() {
    let grid: Vec<Vec<u8>> = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!")
        .lines()
        .map(|line| line.bytes().map(u8_digit_to_value).collect())
        .collect();
    println!(
        "{}",
        (0..(grid.len()))
            .map(|row| (0..(grid[row].len()))
                .map(|col| find_end_positions(row, col, &grid, 0).len())
                .sum::<usize>())
            .sum::<usize>()
    );
}
