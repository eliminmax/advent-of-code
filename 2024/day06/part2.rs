// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 6 Part 2

use std::env::args;
use std::fs::read;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
// values chosen for use as bit flags
enum Direction {
    North = 0b0001,
    East = 0b0010,
    South = 0b0100,
    West = 0b1000,
}

#[derive(Debug, PartialEq)]
enum GuardState {
    Moving,
    Done,
    Looped,
}

#[derive(Debug, Clone)]
struct Guard {
    // easier for row and col to be isizes and cast as needed then to handle underflow properly,
    // when both would be detected as being out-of-bounds anyway.
    row: isize,
    col: isize,
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
            Some(Cell::Unvisited) => {
                self.row = next_row;
                self.col = next_col;
                *(grid
                    .get_mut(self.row as usize)
                    .expect("This row just existed!")
                    .get_mut(self.col as usize)
                    .expect("This cell just existed!")) = Cell::Visited(self.direction as u8);
                GuardState::Moving
            }
            Some(Cell::Visited(i)) if i & (self.direction as u8) == 0 => {
                self.row = next_row;
                self.col = next_col;
                *(grid
                    .get_mut(self.row as usize)
                    .expect("This row just existed!")
                    .get_mut(self.col as usize)
                    .expect("This cell just existed!")) = Cell::Visited(i | (self.direction as u8));
                GuardState::Moving
            }
            Some(Cell::Visited(_)) => GuardState::Looped,
            None => GuardState::Done,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Cell {
    Blocked,
    Unvisited,
    Visited(u8),
}

fn main() {
    let mut guard_template = Guard {
        row: 0,
        col: 0,
        direction: Direction::North,
    };
    let grid_template: Vec<Vec<Cell>> = read(args().nth(1).unwrap_or(String::from("input")))
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
                        guard_template.row = row_num as isize;
                        guard_template.col = col_num as isize;
                        Cell::Visited(Direction::North as u8)
                    }
                    _ => panic!("{:?} is not a valid cell character!", c),
                })
                .collect()
        })
        .collect();
    let mut counter: u32 = 0;
    grid_template.iter().enumerate().for_each(|(row_i, row)| {
        row.iter()
            .enumerate()
            .filter(|(_, cell)| *cell == &Cell::Unvisited)
            .for_each(|(col_i, _)| {
                let mut guard = guard_template.clone();
                let mut grid = grid_template.clone();
                *(grid
                    .get_mut(row_i)
                    .expect("Row index exists in template but not clone.")
                    .get_mut(col_i)
                    .expect("Column index exists in template but not clone.")) = Cell::Blocked;
                'l: loop {
                    match guard.next_state(&mut grid) {
                        GuardState::Moving => {
                            continue 'l;
                        }
                        GuardState::Done => {
                            break 'l;
                        }
                        GuardState::Looped => {
                            counter += 1;
                            break 'l;
                        }
                    }
                }
            })
    });

    println!("{counter}");
}
