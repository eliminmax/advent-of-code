// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 22 Part 2

use std::collections::HashMap;

type GridLocation = (i32, i32);

#[derive(Debug, PartialEq, Copy, Clone, Default)]
enum NodeState {
    #[default]
    Clean,
    Weakened,
    Infected,
    Flagged,
}

impl NodeState {
    fn update(&mut self) {
        use NodeState as NS;
        let next_state = match *self {
            NS::Clean => NS::Weakened,
            NS::Weakened => NS::Infected,
            NS::Infected => NS::Flagged,
            NS::Flagged => NS::Clean,
        };
        *self = next_state;
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Default)]
enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

impl std::ops::Not for Direction {
    type Output = Self;
    fn not(self) -> Self::Output {
        use Direction as D;
        match self {
            D::North => D::South,
            D::East => D::West,
            D::South => D::North,
            D::West => D::East,
        }
    }
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
    fn rotate(&mut self, current_state: NodeState) {
        use Direction as D;
        use NodeState as NS;
        let next_dir = match (*self, current_state) {
            (dir, NS::Weakened) => dir,
            (dir, NS::Flagged) => !dir,
            (D::North, NS::Infected) => D::East,
            (D::East, NS::Infected) => D::South,
            (D::South, NS::Infected) => D::West,
            (D::West, NS::Infected) => D::North,
            (D::North, NS::Clean) => D::West,
            (D::East, NS::Clean) => D::North,
            (D::South, NS::Clean) => D::East,
            (D::West, NS::Clean) => D::South,
        };
        *self = next_dir;
    }
}

#[derive(Debug, Default)]
struct Carrier {
    grid: HashMap<GridLocation, NodeState>,
    location: GridLocation,
    facing: Direction,
    new_infections: u32,
}

impl Carrier {
    fn burst(&mut self) {
        let current_state = *self.grid.get(&self.location).unwrap_or(&NodeState::Clean);
        if current_state == NodeState::Weakened {
            // about to infect, so increment counter
            self.new_infections += 1;
        }
        self.facing.rotate(current_state);
        let (row_off, col_off) = self.facing.as_offsets();
        self.grid
            .entry(self.location)
            .and_modify(|cell| cell.update())
            .or_insert(NodeState::Weakened);
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
                let prev_value = carrier.grid.insert(loc, NodeState::Infected);
                assert!(prev_value.is_none());
            }
        }
    }
    (0..10_000_000).for_each(|_| carrier.burst());
    println!("{}", carrier.new_infections);
}
