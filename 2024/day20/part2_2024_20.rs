// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 20 Part 2

use std::env::args;
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

type Location = (usize, usize);
type Offset = (isize, isize);

#[derive(Debug, PartialEq, Copy, Clone)]
enum GridSpace {
    Blocked,
    Unblocked(Option<usize>),
}

#[derive(Debug, Clone)]
struct CodeGrid {
    data: Vec<GridSpace>,
    start: Location,
    rows: usize,
    cols: usize,
}

type CheatPath = (Location, usize);

#[derive(Debug)]
enum GridParseError {
    /// Grid has a byte other than `b'#'`, `b'S'`, `b'E'`, or `b'.'`
    InvalidGridSpace,
    /// One or more grid rows after the top row don't share the top row's length
    MisalignedGrid,
    /// Grid has a height of 0 or top row width of `0`
    EmptyGrid,
    /// The outer edges of the grid were not all `b'#'`
    OpenEnd,
    /// Multiple `b'S'` bytes were in the grid
    MultipleStarts,
    /// Multiple `b'E'` bytes were in the grid
    MultipleEnds,
    /// No `b'S'` bytes were in the grid
    MissingStart,
    /// No `b'E'` bytes were in the grid
    MissingEnd,
}

impl FromStr for CodeGrid {
    type Err = GridParseError;
    fn from_str(grid_str: &str) -> Result<CodeGrid, Self::Err> {
        let mut start: Option<Location> = None;
        let mut end: Option<Location> = None;

        let grid: Vec<Vec<GridSpace>> = grid_str
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(col, c)| match c {
                        b'#' => Ok(GridSpace::Blocked),
                        b'.' => Ok(GridSpace::Unblocked(None)),
                        b'S' => {
                            if start.is_none() {
                                start = Some((row, col));
                                Ok(GridSpace::Unblocked(Some(0)))
                            } else {
                                Err(GridParseError::MultipleStarts)
                            }
                        }
                        b'E' => {
                            if end.is_none() {
                                end = Some((row, col));
                                Ok(GridSpace::Unblocked(None))
                            } else {
                                Err(GridParseError::MultipleEnds)
                            }
                        }
                        _ => Err(GridParseError::InvalidGridSpace),
                    })
                    .collect::<Result<Vec<GridSpace>, GridParseError>>()
            })
            .collect::<Result<Vec<Vec<GridSpace>>, GridParseError>>()?;

        let start = start.ok_or(GridParseError::MissingStart)?;
        if end.is_none() {
            return Err(GridParseError::MissingEnd);
        }

        let cols = grid.first().ok_or(GridParseError::EmptyGrid)?.len();
        let rows = grid.len();

        if cols == 0 {
            return Err(GridParseError::EmptyGrid);
        }

        if grid.iter().any(|row| row.len() != cols) {
            return Err(GridParseError::MisalignedGrid);
        }

        if grid[0].iter().any(|c| *c != GridSpace::Blocked)
            || grid[grid.len() - 1]
                .iter()
                .any(|c| *c != GridSpace::Blocked)
        {
            return Err(GridParseError::OpenEnd);
        }
        for row in grid[1..(rows - 1)].iter() {
            if row[0] != GridSpace::Blocked || row[cols - 1] != GridSpace::Blocked {
                return Err(GridParseError::OpenEnd);
            }
        }

        Ok(CodeGrid {
            data: grid.into_iter().flat_map(|row| row.into_iter()).collect(),
            start,
            rows,
            cols,
        })
    }
}

impl Index<Location> for CodeGrid {
    type Output = GridSpace;
    fn index(&self, loc: Location) -> &Self::Output {
        &self.data[loc.0 * self.cols + loc.1]
    }
}

impl IndexMut<Location> for CodeGrid {
    fn index_mut(&mut self, loc: Location) -> &mut Self::Output {
        &mut self.data[loc.0 * self.cols + loc.1]
    }
}

fn manhattan_dist(a: Location, b: Location) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn neighborhood(radius: isize) -> Vec<Offset> {
    (-radius..=radius)
        .flat_map(|row| (-radius..=radius).map(move |col| (row, col)))
        .filter(|loc| {
            let dist = loc.0.abs() + loc.1.abs();
            dist > 0 && dist <= radius
        })
        .collect()
}

impl CodeGrid {
    fn apply_offset(&self, loc: Location, off: Offset) -> Option<Location> {
        if let (Some(row), Some(col)) = (
            loc.0.checked_add_signed(off.0),
            loc.1.checked_add_signed(off.1),
        ) {
            if row < self.rows && col < self.cols && self[(row, col)] != GridSpace::Blocked {
                Some((row, col))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn neighbor_locs(&self, loc: Location) -> Vec<Location> {
        neighborhood(1)
            .into_iter()
            .filter_map(|off| self.apply_offset(loc, off))
            .collect()
    }

    /// get a vector of possible cheat destinations from loc
    fn cheat_paths(&self, loc: Location) -> Vec<CheatPath> {
        let mut paths: Vec<CheatPath> = neighborhood(20)
            .into_iter()
            .filter_map(|off| self.apply_offset(loc, off))
            .map(|dst| (dst, manhattan_dist(loc, dst)))
            .collect();
        paths.sort();
        paths.dedup();
        paths
    }

    fn dijkstra_score(&mut self) {
        let mut queue: Vec<(usize, Location)> = vec![(0, self.start)];
        'dijkstra_loop: while !queue.is_empty() {
            queue.sort_by(|a, b| b.cmp(a));
            let (dist, loc) = queue.pop().expect("failed to pop from non-empty queue");
            if let GridSpace::Unblocked(Some(i)) = self[loc] {
                if i < dist {
                    continue 'dijkstra_loop;
                }
            }

            for next_loc in self.neighbor_locs(loc).into_iter() {
                let new_dist = dist + 1;
                match self[next_loc] {
                    GridSpace::Unblocked(Some(i)) if new_dist >= i => (),
                    GridSpace::Blocked => (),
                    _ => {
                        self[next_loc] = GridSpace::Unblocked(Some(new_dist));
                        queue.push((new_dist, next_loc));
                    }
                }
            }
        }
    }
}

fn test_loc(grid: &CodeGrid, loc: Location) -> u32 {
    if grid[loc] == GridSpace::Blocked {
        return 0;
    }
    let mut ret: u32 = 0;
    let GridSpace::Unblocked(Some(base_dist)) = grid[loc] else {
        panic!("invalid start location!")
    };
    for cheat in grid.cheat_paths(loc).into_iter() {
        let GridSpace::Unblocked(Some(dest_dist)) = grid[cheat.0] else {
            panic!("invalid destination location!");
        };
        let cheat_dist = base_dist + manhattan_dist(loc, cheat.0);
        if dest_dist > cheat_dist && dest_dist - cheat_dist >= 100 {
            ret += 1;
        }
    }
    ret
}

fn main() {
    let mut total: u32 = 0;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut grid = CodeGrid::from_str(&input).expect("Failed to parse code path");
    grid.dijkstra_score();
    for row in 0..(grid.rows) {
        for col in 0..(grid.cols) {
            total += test_loc(&grid, (row, col));
        }
    }
    println!("{total}");
}
