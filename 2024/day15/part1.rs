// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 15 Part 1

use std::collections::VecDeque;
use std::env::args;
use std::fmt;
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum GridSpace {
    Empty,
    Wall,
    Box,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum BotMove {
    Up,
    Right,
    Down,
    Left,
}

struct WarehouseFloor {
    grid: Vec<Vec<GridSpace>>,
    bot_location: (usize, usize),
    bot_moves: VecDeque<BotMove>,
}

#[derive(Debug)]
enum WarehouseParseError {
    /// warehouse string doesn't have an empty line separating grid from moves
    MissingDelimiter,
    /// Grid has a byte other than `b'#'`, `b'@'`, `b'.'`, or `b'O'`
    InvalidGridSpace,
    /// Moves have a byte other than `b'<'`, `b'>'`, `b'v'`, `b'^'`, or `b'\n'`
    MisalignedGrid,
    /// Grid has a height of 0 or top row width of `0`
    InvalidBotMove,
    /// One or more grid rows after the top row don't share the top row's length
    EmptyGrid,
    /// Multiple `b'@'` bytes were in the grid
    MultipleBots,
    /// No `b'@'` bytes were in the grid
    NoBots,
    /// The outer edges of the grid were not all `b'#'`
    OpenEnd,
}

impl FromStr for WarehouseFloor {
    type Err = WarehouseParseError;
    fn from_str(s: &str) -> Result<WarehouseFloor, WarehouseParseError> {
        let (grid_str, move_str) = s
            .split_once("\n\n")
            .ok_or(WarehouseParseError::MissingDelimiter)?;

        let mut bot_location: Option<(usize, usize)> = None;

        let grid: Vec<Vec<GridSpace>> = grid_str
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(col, c)| match c {
                        b'#' => Ok(GridSpace::Wall),
                        b'.' => Ok(GridSpace::Empty),
                        b'O' => Ok(GridSpace::Box),
                        b'@' => {
                            if bot_location.is_none() {
                                bot_location = Some((row, col));
                                Ok(GridSpace::Empty)
                            } else {
                                Err(WarehouseParseError::MultipleBots)
                            }
                        }
                        _ => Err(WarehouseParseError::InvalidGridSpace),
                    })
                    .collect::<Result<Vec<GridSpace>, WarehouseParseError>>()
            })
            .collect::<Result<Vec<Vec<GridSpace>>, WarehouseParseError>>()?;

        let bot_location: (usize, usize) = bot_location.ok_or(WarehouseParseError::NoBots)?;

        let cols = grid.first().ok_or(WarehouseParseError::EmptyGrid)?.len();
        let rows = grid.len();

        if cols == 0 {
            return Err(WarehouseParseError::EmptyGrid);
        }

        if grid.iter().any(|row| row.len() != cols) {
            return Err(WarehouseParseError::MisalignedGrid);
        }

        if grid[0].iter().any(|c| *c != GridSpace::Wall)
            || grid[grid.len() - 1].iter().any(|c| *c != GridSpace::Wall)
        {
            return Err(WarehouseParseError::OpenEnd);
        }
        for row in grid[1..(rows - 1)].iter() {
            if row[0] != GridSpace::Wall || row[cols - 1] != GridSpace::Wall {
                return Err(WarehouseParseError::OpenEnd);
            }
        }

        let bot_moves: VecDeque<BotMove> = move_str
            .bytes()
            .filter_map(|m| match m {
                b'^' => Some(Ok(BotMove::Up)),
                b'>' => Some(Ok(BotMove::Right)),
                b'v' => Some(Ok(BotMove::Down)),
                b'<' => Some(Ok(BotMove::Left)),
                b'\n' => None,
                _ => Some(Err(WarehouseParseError::InvalidBotMove)),
            })
            .collect::<Result<_, WarehouseParseError>>()?;
        Ok(WarehouseFloor {
            grid,
            bot_location,
            bot_moves,
        })
    }
}

impl fmt::Debug for WarehouseFloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid_string: String = self
            .grid
            .iter()
            .enumerate()
            .map(|(r, l)| {
                l.iter()
                    .enumerate()
                    .map(|(c, s)| match s {
                        GridSpace::Empty => {
                            if (r, c) == self.bot_location {
                                '@'
                            } else {
                                '.'
                            }
                        }
                        GridSpace::Wall => '#',
                        GridSpace::Box => 'O',
                    })
                    .collect()
            })
            .collect::<Vec<String>>()
            .join("\n");

        let moves_string: String = self
            .bot_moves
            .iter()
            .map(|&m| match m {
                BotMove::Up => '^',
                BotMove::Right => '>',
                BotMove::Down => 'v',
                BotMove::Left => '<',
            })
            .collect();
        write!(f, "{}\n\n{}", grid_string, moves_string)
    }
}

impl Index<(usize, usize)> for WarehouseFloor {
    type Output = GridSpace;
    fn index(&self, loc: (usize, usize)) -> &Self::Output {
        &self.grid[loc.0][loc.1]
    }
}

impl IndexMut<(usize, usize)> for WarehouseFloor {
    fn index_mut(&mut self, loc: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[loc.0][loc.1]
    }
}

impl WarehouseFloor {
    fn tally_gps(&self) -> usize {
        let mut score = 0;
        for (row_num, row) in self.grid.iter().enumerate() {
            row.iter().enumerate().for_each(|(col_num, col)| {
                if *col == GridSpace::Box {
                    score += (row_num * 100) + col_num;
                }
            });
        }
        score
    }

    fn swap(&mut self, block_a: (usize, usize), block_b: (usize, usize)) {
        let temp = self[block_a];
        self[block_a] = self[block_b];
        self[block_b] = temp;
    }

    fn process_moves(&mut self) -> usize {
        macro_rules! process_move {
            (ROW, $op: tt) => {{
                let new_loc = (self.bot_location.0 $op 1, self.bot_location.1);
                match self[new_loc] {
                    GridSpace::Empty => self.bot_location = new_loc,
                    GridSpace::Wall => (), // nop
                    GridSpace::Box => {
                        let mut box_ends = new_loc;
                        let mut offset = 2usize;
                        while self[box_ends] == GridSpace::Box {
                            box_ends = (self.bot_location.0 $op offset, self.bot_location.1);
                            offset += 1;
                        }
                        if self[box_ends] == GridSpace::Empty{
                            self.swap(new_loc, box_ends);
                            self.bot_location = new_loc;
                        }
                    }
                }
            }};
            (COL, $op: tt) => {{
                let new_loc = (self.bot_location.0, self.bot_location.1 $op 1);
                match self[new_loc] {
                    GridSpace::Empty => self.bot_location = new_loc,
                    GridSpace::Wall => (), // nop
                    GridSpace::Box => {
                        let mut box_ends = new_loc;
                        let mut offset = 2usize;
                        while self[box_ends] == GridSpace::Box {
                            box_ends = (self.bot_location.0, self.bot_location.1 $op offset);
                            offset += 1;
                        }
                        if self[box_ends] == GridSpace::Empty{
                            self.swap(new_loc, box_ends);
                            self.bot_location = new_loc;
                        }
                    }
                }
            }};
        }
        while let Some(bot_move) = self.bot_moves.pop_front() {
            match bot_move {
                BotMove::Up => process_move!(ROW, -),
                BotMove::Down => process_move!(ROW, +),
                BotMove::Left => process_move!(COL, -),
                BotMove::Right => process_move!(COL, +),
            }
        }
        self.tally_gps()
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut floor =
        WarehouseFloor::from_str(&input).expect("Failed to parse input as WarehouseFloor");
    println!("{}", floor.process_moves());
}
