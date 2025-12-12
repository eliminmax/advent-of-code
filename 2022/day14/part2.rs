// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 14 Part 2
//
// Based on the solution to AoC 2018 Day 17

use std::collections::HashMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
struct Location {
    // y first to ensure that it gets higher sort priority
    y: u16,
    x: i16,
}

impl Location {
    const fn next_positions(&self) -> [Self; 3] {
        let y = self.y + 1;
        [
            Self { x: self.x, y },
            Self { x: self.x - 1, y },
            Self { x: self.x + 1, y },
        ]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum CaveContents {
    Sand,
    Rock,
}

#[derive(Debug)]
struct Cave {
    grid_contents: HashMap<Location, CaveContents>,
    max_y: u16,
}

impl Cave {
    fn is_occupied(&self, l: &Location) -> bool {
        l.y == self.max_y + 2 || self.grid_contents.contains_key(l)
    }
    fn sand_fill(&mut self, start: Location) {
        'fill_loop: while !self.grid_contents.contains_key(&start) {
            let mut loc = start;
            while loc.y < self.max_y + 2 {
                if let Some(next_loc) = loc
                    .next_positions()
                    .into_iter()
                    .find(|l| !self.is_occupied(l))
                {
                    loc = next_loc;
                } else {
                    self.grid_contents.insert(loc, CaveContents::Sand);
                    continue 'fill_loop;
                };
            }
        }
    }

    fn sand_amount(&mut self) -> usize {
        self.sand_fill(Location { x: 500, y: 0 });
        self.grid_contents
            .values()
            .filter(|&gc| *gc == CaveContents::Sand)
            .count()
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut cave: Cave = input.parse().unwrap();
    println!("{}", cave.sand_amount());
}

#[derive(Debug)]
enum CaveParseError {
    IntParseFailure(#[allow(unused)] std::num::ParseIntError),
    BadFormat(#[allow(unused)] Box<str>),
}

impl From<&str> for CaveParseError {
    fn from(e: &str) -> Self {
        Self::BadFormat(Box::from(e))
    }
}

impl From<std::num::ParseIntError> for CaveParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::IntParseFailure(e)
    }
}

impl std::str::FromStr for Cave {
    type Err = CaveParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid_contents: HashMap<Location, CaveContents> = HashMap::new();

        for line in s.lines() {
            let segment_ends: Vec<Location> = line
                .split(" -> ")
                .map(|point| {
                    point
                        .split_once(',')
                        .map(|(x, y)| {
                            Ok::<Location, CaveParseError>(Location {
                                x: x.parse()?,
                                y: y.parse()?,
                            })
                        })
                        .ok_or_else(|| CaveParseError::BadFormat(point.into()))?
                })
                .collect::<Result<Vec<_>, _>>()?;
            let segments: Vec<(Location, Location)> =
                segment_ends.windows(2).map(|w| (w[0], w[1])).collect();
            let to_entry = |x, y| (Location { x, y }, CaveContents::Rock);

            for (start, stop) in segments {
                if start.x == stop.x {
                    let mut y_ends = [start.y, stop.y];
                    y_ends.sort();
                    grid_contents.extend((y_ends[0]..=y_ends[1]).map(|y| to_entry(start.x, y)));
                } else {
                    assert!(start.y == stop.y);
                    let mut x_ends = [start.x, stop.x];
                    x_ends.sort();
                    grid_contents.extend((x_ends[0]..=x_ends[1]).map(|x| to_entry(x, start.y)));
                }
            }
        }

        let max_y = grid_contents
            .keys()
            .map(|&Location { y, .. }| y)
            .max()
            .unwrap();

        Ok(Cave {
            grid_contents,
            max_y,
        })
    }
}
