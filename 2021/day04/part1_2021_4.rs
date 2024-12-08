// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 4 Part 1

use std::convert::TryInto;
use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
struct Space {
    space_num: u8,
    occupied: bool,
}

impl From<u8> for Space {
    fn from(n: u8) -> Self {
        Space {
            space_num: n,
            occupied: false,
        }
    }
}

#[derive(Debug)]
struct BingoBoard([[Space; 5]; 5]);

impl BingoBoard {
    fn score(&self) -> u32 {
        self.0
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|c| {
                        if c.occupied {
                            None
                        } else {
                            Some(c.space_num as u32)
                        }
                    })
                    .sum::<u32>()
            })
            .sum()
    }
    fn check_victory(&self, row: usize, col: usize) -> Option<u32> {
        if self.0[row].iter().all(|c| c.occupied) || self.0.iter().all(|r| r[col].occupied) {
            Some(self.score())
        } else {
            None
        }
    }

    fn mark(&mut self, n: u8) -> Option<u32> {
        for (row_num, row) in self.0.iter_mut().enumerate() {
            for (col, cell) in row.iter_mut().enumerate() {
                if cell.space_num == n {
                    cell.occupied = true;
                    return self.check_victory(row_num, col);
                }
            }
        }
        None
    }
}

impl FromStr for BingoBoard {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells: Result<Vec<u8>, Self::Err> = s
            .split_whitespace()
            .map(|c| u8::from_str(c).map_err(|_| "failed to parse u8"))
            .collect();
        let cells: Vec<Space> = cells?.into_iter().map(Space::from).collect();
        match cells.len() {
            i if i > 25 => Err("Too many cells!"),
            i if i < 25 => Err("Too few cells!"),
            _ => {
                let mut cell_slices = cells.chunks_exact(5);
                macro_rules! next_row {
                    () => {{
                        let row: [Space; 5] = cell_slices
                            .next()
                            .expect("Failed to get row")
                            .try_into()
                            .expect("Wrong number of elements");
                        row
                    }};
                }
                Ok(BingoBoard([
                    next_row!(),
                    next_row!(),
                    next_row!(),
                    next_row!(),
                    next_row!(),
                ]))
            }
        }
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut line_groups = input.split("\n\n");
    let nums: Vec<u8> = line_groups
        .next()
        .expect("Failed to read first line")
        .split(',')
        .map(|c| u8::from_str(c.trim()).expect("Failed to parse number"))
        .collect();
    let mut boards: Vec<BingoBoard> = line_groups
        .map(|b| BingoBoard::from_str(b).expect("Failed to parse board"))
        .collect();
    'bingo: for num in nums.iter() {
        for board in boards.iter_mut() {
            if let Some(score) = board.mark(*num) {
                println!("{}", score * *num as u32);
                break 'bingo;
            }
        }
    }
}
