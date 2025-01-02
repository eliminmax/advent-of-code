// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 24 Part 1

// A lot of code is borrowed from my solution to 2024 day 20, but with a lot of simplification
use std::collections::HashMap;

/// generic version of my implementation of Heap's Algorithm from 2019 day 07, made to accept
/// immutable slices and clone them
fn gen_permutations<T: Copy>(vals: &[T]) -> Vec<Vec<T>> {
    fn heap_permutations<T: Copy>(vals: &mut [T]) -> Vec<Vec<T>> {
        if vals.len() == 1 {
            return vec![vals.to_owned()];
        }
        let mut permutations: Vec<Vec<T>> = Vec::new();
        let mut new_perms: Vec<Vec<T>>;
        let last = vals.len() - 1;
        for i in 0..=last {
            new_perms = heap_permutations(&mut vals[..last]);
            new_perms.iter_mut().for_each(|v| v.push(vals[last]));
            permutations.append(&mut new_perms);
            if last % 2 == 0 {
                vals.swap(0, last);
            } else {
                vals.swap(i, last);
            }
        }
        permutations
    }
    heap_permutations(&mut vals.to_owned()[..])
}

type DistanceTable = HashMap<u8, HashMap<u8, u32>>;
type Location = (usize, usize);

#[derive(Debug, PartialEq, Copy, Clone)]
enum GridSpace {
    Blocked,
    Unblocked,
}

#[derive(Debug, Clone)]
struct CodeGrid {
    data: Vec<GridSpace>,
    start: Location,
    dests: HashMap<u8, Location>,
    cols: usize,
}

impl CodeGrid {
    fn get(&self, loc: Location) -> Option<&GridSpace> {
        self.data.get(loc.0 * self.cols + loc.1)
    }

    fn neighbor_locs(&self, loc: Location) -> impl Iterator<Item = Location> + use<'_> {
        let (row, col) = loc;
        vec![
            row.checked_sub(1).map(|nr| (nr, col)),
            row.checked_add(1).map(|nr| (nr, col)),
            col.checked_sub(1).map(|nc| (row, nc)),
            col.checked_add(1).map(|nc| (row, nc)),
        ]
        .into_iter()
        .filter_map(move |i| {
            if i.and_then(|l| self.get(l))
                .is_some_and(|l| *l == GridSpace::Unblocked)
            {
                i
            } else {
                None
            }
        })
    }

    fn dijkstra_score(&self, from_loc: Location) -> HashMap<u8, u32> {
        use std::iter::FromIterator;
        let mut scores: HashMap<Location, u32> = HashMap::new();
        let mut queue: Vec<(u32, Location)> = vec![(0, from_loc)];
        while let Some((dist, loc)) = queue.pop() {
            if scores.get(&loc).is_some_and(|prev| *prev < dist) {
                continue;
            }
            for neighbor in self.neighbor_locs(loc) {
                let new_dist = dist + 1;
                if scores.get(&neighbor).is_none_or(|prev| *prev > new_dist) {
                    let _ = scores.insert(neighbor, new_dist);
                    queue.push((new_dist, neighbor));
                }
            }
            // sort backwards so that the next to be popped will have the lowest distance possible
            // (I didn't use a VecDeque for the queue because it can't be sorted like this)
            queue.sort_by(|a, b| b.cmp(a));
        }
        HashMap::from_iter(self.dests.iter().map(|(k, loc)| (*k, scores[loc])))
    }

    fn gen_distance_table(&self) -> DistanceTable {
        let mut distance_table = DistanceTable::from([(b'0', self.dijkstra_score(self.start))]);
        for (k, loc) in self.dests.iter() {
            let _ = distance_table.insert(*k, self.dijkstra_score(*loc));
        }
        distance_table
    }

    fn min_distance(&self) -> u32 {
        let table = self.gen_distance_table();
        let keys: Vec<_> = self.dests.keys().cloned().collect();
        let mut permutations = gen_permutations(&keys[..]);
        let mut min_dist_seen: Option<u32> = None;
        while let Some(mut permutation) = permutations.pop() {
            permutation.insert(0, b'0');
            let dist: u32 = permutation.windows(2).map(|w| table[&w[0]][&w[1]]).sum();
            if min_dist_seen.is_none_or(|d| d > dist) {
                min_dist_seen = Some(dist);
            }
        }
        // if no nodes are there, it takes 0 to get from start to all destinations
        min_dist_seen.unwrap_or(0)
    }
}

impl std::ops::Index<Location> for CodeGrid {
    type Output = GridSpace;
    fn index(&self, loc: Location) -> &Self::Output {
        &self.data[loc.0 * self.cols + loc.1]
    }
}

impl std::ops::IndexMut<Location> for CodeGrid {
    fn index_mut(&mut self, loc: Location) -> &mut Self::Output {
        &mut self.data[loc.0 * self.cols + loc.1]
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
    /// Multiple `b'0'` bytes were in the grid
    MultipleStarts,
    /// Multiple `b'E'` bytes were in the grid
    DuplicateDest,
    /// No `b'0'` bytes were in the grid
    MissingStart,
}

impl std::str::FromStr for CodeGrid {
    type Err = GridParseError;
    fn from_str(grid_str: &str) -> Result<CodeGrid, Self::Err> {
        let mut start: Option<Location> = None;
        let mut dests: HashMap<u8, Location> = HashMap::new();

        let grid: Vec<Vec<GridSpace>> = grid_str
            .lines()
            .enumerate()
            .map(|(row, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(col, c)| match c {
                        b'#' => Ok(GridSpace::Blocked),
                        b'.' => Ok(GridSpace::Unblocked),
                        b'0' => {
                            if start.is_none() {
                                start = Some((row, col));
                                Ok(GridSpace::Unblocked)
                            } else {
                                Err(GridParseError::MultipleStarts)
                            }
                        }
                        b'1'..=b'9' => {
                            if dests.insert(c, (row, col)).is_none() {
                                Ok(GridSpace::Unblocked)
                            } else {
                                Err(GridParseError::DuplicateDest)
                            }
                        }
                        _ => Err(GridParseError::InvalidGridSpace),
                    })
                    .collect::<Result<Vec<GridSpace>, GridParseError>>()
            })
            .collect::<Result<Vec<Vec<GridSpace>>, GridParseError>>()?;

        let start = start.ok_or(GridParseError::MissingStart)?;

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
            dests,
            cols,
        })
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let grid: CodeGrid = input.parse().expect("Failed to parse input");
    println!("{}", grid.min_distance());
}
