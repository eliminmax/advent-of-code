// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 22 Part 1

// Before starting, I want to note that this is just Langton's Ant.
// https://en.wikipedia.org/wiki/Langton's_ant

use std::collections::HashMap;

type GridLocation = (i32, i32);

#[derive(Debug, PartialEq, Copy, Clone, Default)]
enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl Direction {
    fn as_offsets(&self) -> GridLocation {
        use Direction as D;
        match self {
            D::North => (-1, 0),
            D::East => (0, 1),
            D::South => (1, 0),
            D::West => (0, -1),
        }
    }
    fn rotate(&mut self, currently_infected: bool) {
        use Direction as D;
        let next_dir = match (*self, currently_infected) {
            (D::North, true) => D::East,
            (D::East, true) => D::South,
            (D::South, true) => D::West,
            (D::West, true) => D::North,
            (D::North, false) => D::West,
            (D::East, false) => D::North,
            (D::South, false) => D::East,
            (D::West, false) => D::South,
        };
        *self = next_dir;
    }
}

#[derive(Debug, Default)]
struct Carrier {
    grid: HashMap<GridLocation, bool>,
    location: GridLocation,
    facing: Direction,
    new_infections: u16,
}

impl Carrier {
    fn burst(&mut self) {
        let currently_infected = *self.grid.get(&self.location).unwrap_or(&false);
        if !currently_infected {
            // about to infect, so increment counter
            self.new_infections += 1;
        }
        self.facing.rotate(currently_infected);
        let (row_off, col_off) = self.facing.as_offsets();
        self.grid
            .entry(self.location)
            .and_modify(|cell| *cell = !currently_infected)
            .or_insert(true);
        self.location = (self.location.0 + row_off, self.location.1 + col_off);
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let mut carrier = Carrier::default();
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let start_grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();
    assert!(start_grid.iter().all(|row| row.len() == start_grid.len()));
    let offset = (start_grid.len() as i32) / 2;
    for (row_num, row) in start_grid.into_iter().enumerate() {
        for (col_num, infected) in row.into_iter().enumerate() {
            if infected {
                let loc = ((row_num as i32) - offset, (col_num as i32) - offset);
                let prev_value = carrier.grid.insert(loc, true);
                assert!(prev_value.is_none());
            }
        }
    }
    (0..10_000).for_each(|_| carrier.burst());
    println!("{}", carrier.new_infections);
}
