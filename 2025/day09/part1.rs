// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 09 Part 1

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let red_tiles: Vec<(u64, u64)> = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let mut max_size = 0;
    for (i, &(x0, y0)) in red_tiles.iter().enumerate() {
        for &(x1, y1) in red_tiles[i + 1..].iter() {
            let w = x0.abs_diff(x1) + 1;
            let h = y0.abs_diff(y1) + 1;
            max_size = max_size.max(w * h);
        }
    }
    println!("{max_size}");
}
