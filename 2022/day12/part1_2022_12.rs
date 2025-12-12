// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 12 Part 1

mod dijkstra;
use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn neighbors(self) -> impl Iterator<Item = Self> {
        [
            self.x.checked_add(1).map(|x| Position { x, ..self }),
            self.y.checked_add(1).map(|y| Position { y, ..self }),
            self.x.checked_sub(1).map(|x| Position { x, ..self }),
            self.y.checked_sub(1).map(|y| Position { y, ..self }),
        ]
        .into_iter()
        .flatten()
    }
    const fn into_index(self, cols: usize) -> usize {
        self.y * cols + self.x
    }
}

#[derive(Debug)]
struct HeightMap {
    heights: Box<[u8]>,
    start: Position,
    end: Position,
    rows: usize,
    cols: usize,
}

impl Index<Position> for HeightMap {
    type Output = u8;

    fn index(&self, pos: Position) -> &Self::Output {
        self.heights.index(pos.into_index(self.cols))
    }
}
impl HeightMap {
    fn contains(&self, &Position { x, y }: &Position) -> bool {
        x < self.cols && y < self.rows
    }
    fn shortest_path_length(&self) -> u16 {
        dijkstra::targeted_dijkstra(self.start, self.end, |pos| {
            pos.neighbors().filter_map(move |neighbor| {
                if self.contains(&neighbor) && self[neighbor] <= self[pos] + 1 {
                    Some((neighbor, 1))
                } else {
                    None
                }
            })
        })
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let hm: HeightMap = input.parse().unwrap();
    println!("{}", hm.shortest_path_length());
}

impl std::str::FromStr for HeightMap {
    type Err = HeightMapParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut counted_lines = s.lines().enumerate();
        let mut heights: Vec<u8> = Vec::with_capacity(s.len());
        let Some((0, line_0)) = counted_lines.next() else {
            return Err(Self::Err::Empty);
        };

        let cols = line_0.len();
        let mut start: Option<Position> = None;
        let mut end: Option<Position> = None;

        // for valid values, return a height from 0 to 25, setting start and end to `pos` where
        // appropriate. On invalid values, including a second `E` or `S`, returns an appropriate
        // `Err` to bubble up.
        let mut process = |pos, b| match b {
            b'a'..=b'z' => Ok(b - b'a'),
            b'\n' => unreachable!(),
            b'S' => {
                if start.replace(pos).is_some() {
                    Err(Self::Err::DuplicateStart)
                } else {
                    Ok(0)
                }
            }
            b'E' => {
                if end.replace(pos).is_some() {
                    Err(Self::Err::DuplicateEnd)
                } else {
                    Ok(25)
                }
            }
            _ => Err(Self::Err::BadId),
        };

        for (x, b) in line_0.bytes().enumerate() {
            heights.push(process(Position { x, y: 0 }, b)?);
        }

        for (y, line) in counted_lines {
            if line.len() != cols {
                return Err(Self::Err::InconsistentDimensions);
            }
            for (x, b) in line.bytes().enumerate() {
                heights.push(process(Position { x, y }, b)?);
            }
        }
        let start = start.ok_or(Self::Err::NoStart)?;
        let end = end.ok_or(Self::Err::NoEnd)?;
        debug_assert_ne!(start, end, "{start:?}, {end:?}");

        let heights = heights.into_boxed_slice();
        let rows = heights.len() / cols;
        Ok(Self {
            heights,
            start,
            end,
            rows,
            cols,
        })
    }
}

#[derive(Debug)]
enum HeightMapParseError {
    Empty,
    InconsistentDimensions,
    NoStart,
    NoEnd,
    DuplicateStart,
    DuplicateEnd,
    BadId,
}
