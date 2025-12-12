// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 20 Part 1

use std::env::args;
use std::fs::read_to_string;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum GridSpace {
    Blocked,
    Unblocked(Option<u16>),
}

#[derive(Debug, Clone)]
struct CodeGrid {
    data: Vec<GridSpace>,
    start: (usize, usize),
    end: (usize, usize),
    rows: usize,
    cols: usize,
}

impl Index<(usize, usize)> for CodeGrid {
    type Output = GridSpace;
    fn index(&self, loc: (usize, usize)) -> &Self::Output {
        &self.data[loc.0 * self.cols + loc.1]
    }
}

impl IndexMut<(usize, usize)> for CodeGrid {
    fn index_mut(&mut self, loc: (usize, usize)) -> &mut Self::Output {
        &mut self.data[loc.0 * self.cols + loc.1]
    }
}

impl CodeGrid {
    fn get(&self, loc: (usize, usize)) -> Option<&GridSpace> {
        self.data.get(loc.0 * self.cols + loc.1)
    }

    fn neighbor_locs(&self, loc: (usize, usize)) -> Vec<(usize, usize)> {
        let (row, col) = loc;
        vec![
            row.checked_sub(1).map(|nr| (nr, col)),
            row.checked_add(1).map(|nr| (nr, col)),
            col.checked_sub(1).map(|nc| (row, nc)),
            col.checked_add(1).map(|nc| (row, nc)),
        ]
        .into_iter()
        .filter_map(|i| {
            if i.and_then(|l| self.get(l)).is_some() {
                i
            } else {
                None
            }
        })
        .collect()
    }
}

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
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;

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
        let end = end.ok_or(GridParseError::MissingEnd)?;

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
            end,
            rows,
            cols,
        })
    }
}

fn dijkstra_score(grid: &CodeGrid) -> Option<u16> {
    let mut grid = grid.clone();
    let mut queue: Vec<(u16, (usize, usize))> = vec![(0, grid.start)];
    'dijkstra_loop: while !queue.is_empty() {
        queue.sort_by(|a, b| b.cmp(a));
        let (dist, loc) = queue.pop().expect("failed to pop from non-empty queue");
        if let GridSpace::Unblocked(Some(i)) = grid[loc] {
            if i < dist {
                continue 'dijkstra_loop;
            }
        }

        for next_loc in grid.neighbor_locs(loc).into_iter() {
            let new_dist = dist + 1;
            match grid[next_loc] {
                GridSpace::Unblocked(Some(i)) if new_dist >= i => (),
                GridSpace::Blocked => (),
                _ => {
                    grid[next_loc] = GridSpace::Unblocked(Some(new_dist));
                    queue.push((new_dist, next_loc));
                }
            }
        }
    }

    if let GridSpace::Unblocked(res) = grid[grid.end] {
        res
    } else {
        panic!("End is not an unblocked space");
    }
}

fn test_loc(grid: &mut CodeGrid, loc: (usize, usize), baseline: u16) -> bool {
    if grid[loc] != GridSpace::Blocked {
        return false;
    }
    let unblocked_neighbors = grid
        .neighbor_locs(loc)
        .into_iter()
        .filter(|&l| grid[l] != GridSpace::Blocked)
        .count();
    if unblocked_neighbors < 2 {
        return false;
    }
    grid[loc] = GridSpace::Unblocked(None);
    let ret = dijkstra_score(grid).is_some_and(|val| baseline - val >= 100);
    grid[loc] = GridSpace::Blocked;
    ret
}

fn main() {
    let mut total: u32 = 0;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut grid = CodeGrid::from_str(&input).expect("Failed to parse code path");
    let baseline = dijkstra_score(&grid).expect("End is unreachable in baseline");
    for row in 0..(grid.rows) {
        for col in 0..(grid.cols) {
            if test_loc(&mut grid, (row, col), baseline) {
                total += 1;
            }
        }
    }
    println!("{total}");
}
