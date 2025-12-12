// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 04 Part 1

use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i16,
    y: i16,
}

impl Position {
    const fn neighbors(&self) -> [Self; 8] {
        macro_rules! pos {
            ($x: expr, $y: expr) => {
                Position { x: $x, y: $y }
            };
        }
        [
            pos!(self.x - 1, self.y - 1),
            pos!(self.x - 1, self.y),
            pos!(self.x - 1, self.y + 1),
            pos!(self.x, self.y - 1),
            pos!(self.x, self.y + 1),
            pos!(self.x + 1, self.y - 1),
            pos!(self.x + 1, self.y),
            pos!(self.x + 1, self.y + 1),
        ]
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let mut rolls = HashSet::new();
    for (y, row) in input.lines().enumerate() {
        for (x, chr) in row.chars().enumerate() {
            if chr == '@' {
                rolls.insert(Position {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                });
            }
        }
    }

    println!(
        "{}",
        rolls
            .iter()
            .filter(|r| {
                r.neighbors()
                    .into_iter()
                    .filter(|n| rolls.contains(n))
                    .count()
                    < 4
            })
            .count()
    );
}
