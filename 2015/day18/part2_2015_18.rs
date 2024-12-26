// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 18 Part 2

use std::env::args;
use std::fs::read_to_string;

const GRID_SIZE: usize = 100;

struct GOLGrid([[bool; GRID_SIZE]; GRID_SIZE]);

impl std::ops::Index<(usize, usize)> for GOLGrid {
    type Output = bool;
    fn index(&self, (row, col): (usize, usize)) -> &bool {
        &self.0[row][col]
    }
}

impl std::ops::IndexMut<(usize, usize)> for GOLGrid {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut bool {
        &mut self.0[row][col]
    }
}

impl GOLGrid {
    fn locations(&self) -> Vec<(usize, usize)> {
        (0..GRID_SIZE)
            .flat_map(|r| (0..GRID_SIZE).map(move |c| (r, c)))
            .collect()
    }

    fn count_neighbors(&self, (row, col): (usize, usize)) -> usize {
        let rows_start = row.saturating_sub(1);
        let rows_stop = GRID_SIZE.min(row + 2);
        let cols_start = col.saturating_sub(1);
        let cols_stop = GRID_SIZE.min(col + 2);
        (rows_start..rows_stop)
            .flat_map(|r| (cols_start..cols_stop).map(move |c| (r, c)))
            .filter(|&loc| (loc != (row, col)) && self[loc])
            .count()
    }

    fn update(&mut self) {
        let to_kill: Vec<(usize, usize)> = self.locations().into_iter().filter(|&loc| {
            self[loc] && {
                let n = self.count_neighbors(loc);
                n != 2 && n != 3
            }
        })
        .collect();

        let to_spawn: Vec<(usize, usize)> = self
            .locations()
            .into_iter()
            .filter(|&loc| (!self[loc]) && self.count_neighbors(loc) == 3)
            .collect();
        to_kill.into_iter().for_each(|loc| self[loc] = false);
        to_spawn.into_iter().for_each(|loc| self[loc] = true);
        /* the broken always on lights */
        self[(0, 0)] = true;
        self[(0, GRID_SIZE - 1)] = true;
        self[(GRID_SIZE - 1, 0)] = true;
        self[(GRID_SIZE - 1, GRID_SIZE - 1)] = true;
    }

    fn count_living(&self) -> usize {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&cell| *cell)
            .count()
    }
}

#[derive(Debug)]
enum GOLParseError {
    BadSize,
    InvalidCell,
}

impl std::str::FromStr for GOLGrid {
    type Err = GOLParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new_self = GOLGrid([[false; GRID_SIZE]; GRID_SIZE]);
        let lines: Vec<&str> = s.lines().collect();
        if lines.len() != GRID_SIZE {
            return Err(GOLParseError::BadSize);
        }
        for (row, row_str) in lines.into_iter().enumerate() {
            if row_str.len() != GRID_SIZE {
                return Err(GOLParseError::BadSize);
            }
            for (col, cell_byte) in row_str.bytes().enumerate() {
                match cell_byte {
                    b'#' => new_self[(row, col)] = true,
                    b'.' => (), // already false by default
                    _ => return Err(GOLParseError::InvalidCell),
                }
            }
        }
        /* the broken always on lights */
        new_self[(0, 0)] = true;
        new_self[(0, GRID_SIZE - 1)] = true;
        new_self[(GRID_SIZE - 1, 0)] = true;
        new_self[(GRID_SIZE - 1, GRID_SIZE - 1)] = true;
        Ok(new_self)
    }
}
fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut light_matrix: GOLGrid = input
        .parse()
        .expect("Failed to parse input into light matrix");
    (0..100).for_each(|_| light_matrix.update());
    println!("{}", light_matrix.count_living());
}
