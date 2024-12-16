// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 15 Part 2

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
    BoxRight,
    BoxLeft,
    Bot,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum BotMove {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Location(usize, usize);

struct WarehouseFloor {
    grid: Vec<Vec<GridSpace>>,
    bot_loc: Location,
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

        let mut bot_loc: Option<Location> = None;

        let grid: Vec<Vec<GridSpace>> = grid_str
            .lines()
            .enumerate()
            .map(|(row, l)| {
                let mut v: Vec<GridSpace> = Vec::new();
                let mut e: Option<WarehouseParseError> = None;
                l.bytes().enumerate().for_each(|(col, c)| match c {
                    b'#' => v.extend([GridSpace::Wall; 2].iter()),
                    b'.' => v.extend([GridSpace::Empty; 2].iter()),
                    b'O' => v.extend([GridSpace::BoxLeft, GridSpace::BoxRight].iter()),
                    b'@' => {
                        v.extend([GridSpace::Bot, GridSpace::Empty].iter());
                        if bot_loc.is_some() {
                            e = Some(WarehouseParseError::MultipleBots);
                        } else {
                            bot_loc = Some(Location(row, col * 2));
                        }
                    }
                    _ => e = Some(WarehouseParseError::InvalidGridSpace),
                });
                if let Some(err) = e {
                    Err(err)
                } else {
                    Ok(v)
                }
            })
            .collect::<Result<Vec<Vec<GridSpace>>, WarehouseParseError>>()?;

        let bot_loc: Location = bot_loc.ok_or(WarehouseParseError::NoBots)?;

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
            bot_loc,
            bot_moves,
        })
    }
}

impl fmt::Debug for WarehouseFloor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let grid_string: String = self
            .grid
            .iter()
            .map(|l| {
                l.iter()
                    .map(|gs| match gs {
                        GridSpace::Empty => '.',
                        GridSpace::Bot => '@',
                        GridSpace::Wall => '#',
                        GridSpace::BoxRight => ']',
                        GridSpace::BoxLeft => '[',
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

impl Index<Location> for WarehouseFloor {
    type Output = GridSpace;
    fn index(&self, loc: Location) -> &Self::Output {
        &self.grid[loc.0][loc.1]
    }
}

impl IndexMut<Location> for WarehouseFloor {
    fn index_mut(&mut self, loc: Location) -> &mut Self::Output {
        &mut self.grid[loc.0][loc.1]
    }
}

macro_rules! next_loc {
    ($loc: expr,  $row_op: tt, $col_op: tt) => {{
        Location($loc.0 $row_op 1, $loc.1 $col_op 1)
    }};
    ($loc: expr, Up) => {{
        next_loc!($loc, -, *)
    }};
    ($loc: expr, Down) => {{
        next_loc!($loc, +, *)
    }};
    ($loc: expr, Left) => {{
        next_loc!($loc, *, -)
    }};
    ($loc: expr, Right) => {{
        next_loc!($loc, *, +)
    }};
    ($loc: expr, $move_type: ident) => {{
        match $move_type {
            BotMove::Up => next_loc!($loc, Up),
            BotMove::Right => next_loc!($loc, Right),
            BotMove::Down => next_loc!($loc, Down),
            BotMove::Left => next_loc!($loc, Left),
        }
    }};
}

impl WarehouseFloor {
    fn tally_gps(&self) -> usize {
        let mut score = 0;
        for (row_num, row) in self.grid.iter().enumerate() {
            row.iter().enumerate().for_each(|(col_num, col)| {
                if *col == GridSpace::BoxLeft {
                    score += (row_num * 100) + col_num;
                }
            });
        }
        score
    }

    fn swap(&mut self, block_a: Location, block_b: Location) {
        let temp = self[block_a];
        self[block_a] = self[block_b];
        self[block_b] = temp;
    }

    fn check_move_vert(
        &self,
        loc: Location,
        move_type: BotMove,
        would_move: &mut Vec<Location>,
    ) -> bool {
        match self[loc] {
            GridSpace::Empty => true,
            GridSpace::Wall => false,
            GridSpace::BoxRight => {
                would_move.push(loc);
                let partner = next_loc!(loc, Left);
                self.check_move_vert(next_loc!(loc, move_type), move_type, would_move)
                    && (would_move.contains(&partner)
                        || self.check_move_vert(partner, move_type, would_move))
            }
            GridSpace::BoxLeft => {
                would_move.push(loc);
                let partner = next_loc!(loc, Right);
                self.check_move_vert(next_loc!(loc, move_type), move_type, would_move)
                    && (would_move.contains(&partner)
                        || self.check_move_vert(partner, move_type, would_move))
            }
            GridSpace::Bot => {
                would_move.push(loc);
                self.check_move_vert(next_loc!(loc, move_type), move_type, would_move)
            }
        }
    }

    fn check_move_horiz(
        &self,
        loc: Location,
        move_type: BotMove,
        would_move: &mut Vec<Location>,
    ) -> bool {
        match self[loc] {
            GridSpace::Empty => true,
            GridSpace::Wall => false,
            GridSpace::Bot | GridSpace::BoxRight | GridSpace::BoxLeft => {
                would_move.push(loc);
                self.check_move_horiz(next_loc!(loc, move_type), move_type, would_move)
            }
        }
    }

    fn check_move(&self, move_type: BotMove) -> Option<Vec<Location>> {
        let mut moves: Vec<Location> = Vec::new();
        let check = if move_type == BotMove::Up || move_type == BotMove::Down {
            self.check_move_vert(self.bot_loc, move_type, &mut moves)
        } else {
            self.check_move_horiz(self.bot_loc, move_type, &mut moves)
        };
        if check {
            Some(moves)
        } else {
            None
        }
    }

    fn process_moves(&mut self) -> usize {
        while let Some(bot_move) = self.bot_moves.pop_front() {
            if let Some(mut moves) = self.check_move(bot_move) {
                // moves should be processed going from the furthest from bot to the closest on the
                // axis that the moves are on
                match bot_move {
                    BotMove::Down => moves.sort_by(|a, b| (b.0, b.1).cmp(&(a.0, a.1))),
                    BotMove::Up => moves.sort_by(|a, b| (a.0, a.1).cmp(&(b.0, b.1))),
                    BotMove::Left => moves.sort_by(|a, b| (a.1, a.0).cmp(&(b.1, b.0))),
                    BotMove::Right => moves.sort_by(|a, b| (b.1, b.0).cmp(&(a.1, a.0))),
                }
                moves.dedup();
                for move_from in moves.iter() {
                    let next = next_loc!(move_from, bot_move);
                    if self[next] != GridSpace::Empty {
                        panic!("{:?}", moves);
                    }
                    self.swap(*move_from, next);
                }
                self.bot_loc = next_loc!(self.bot_loc, bot_move);
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
