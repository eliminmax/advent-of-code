// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 6 Part 1

use std::env::args;
use std::fs::read;

#[derive(Debug)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug, PartialEq)]
enum GuardState {
    Moving,
    Done,
}

#[derive(Debug)]
struct Guard {
    // easier for row and col to be isizes and cast as needed then to handle underflow properly,
    // when both would be detected as being out-of-bounds anyway.
    row: isize,
    col: isize,
    unique: usize,
    direction: Direction,
}

impl Guard {
    fn rotate(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::East => self.direction = Direction::South,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
        }
    }

    fn next_state(&mut self, grid: &mut Vec<Vec<Cell>>) -> GuardState {
        let (next_row, next_col) = match self.direction {
            Direction::North => (self.row - 1, self.col),
            Direction::East => (self.row, self.col + 1),
            Direction::South => (self.row + 1, self.col),
            Direction::West => (self.row, self.col - 1),
        };
        match grid
            .get(next_row as usize)
            .and_then(|row| row.get(next_col as usize))
        {
            Some(Cell::Blocked) => {
                self.rotate();
                self.next_state(grid)
            }
            Some(Cell::Visited) => {
                self.row = next_row;
                self.col = next_col;
                GuardState::Moving
            }
            Some(Cell::Unvisited) => {
                self.unique += 1;
                self.row = next_row;
                self.col = next_col;
                *(grid
                    .get_mut(self.row as usize)
                    .expect("This row just existed!")
                    .get_mut(self.col as usize)
                    .expect("This cell just existed!")) = Cell::Visited;
                GuardState::Moving
            }
            None => GuardState::Done,
        }
    }
}

#[derive(Debug)]
enum Cell {
    Blocked,
    Unvisited,
    Visited,
}

fn main() {
    let mut guard = Guard {
        row: 0,
        col: 0,
        unique: 1,
        direction: Direction::North,
    };
    let mut grid: Vec<Vec<Cell>> = read(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!")
        .split(|c| *c == b'\n')
        .filter_map(|row| {
            if !row.is_empty() {
                Some(row.to_owned())
            } else {
                None
            }
        })
        .enumerate()
        .map(|(row_num, row)| {
            row.iter()
                .enumerate()
                .map(|(col_num, c)| match *c {
                    b'#' => Cell::Blocked,
                    b'.' => Cell::Unvisited,
                    b'^' => {
                        guard.row = row_num as isize;
                        guard.col = col_num as isize;
                        Cell::Visited
                    }
                    _ => panic!("{:?} is not a valid cell character!", c),
                })
                .collect()
        })
        .collect();
    loop {
        if guard.next_state(&mut grid) == GuardState::Done {
            break;
        }
    }

    println!("{}", guard.unique);
}
