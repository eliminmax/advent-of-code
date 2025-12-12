// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 12 Part 2

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
        let map = dijkstra::dijkstra(self.end, |pos| {
            pos.neighbors().filter_map(move |neighbor| {
                // reversed from part 1 to check if `pos` is reachable from `neighbor`, rather than
                // if it can reach it.
                if self.contains(&neighbor) && self[pos] <= self[neighbor] + 1 {
                    Some((neighbor, 1))
                } else {
                    None
                }
            })
        });
        map.into_iter()
            .filter_map(|(pos, dist)| if self[pos] == 0 { Some(dist) } else { None })
            .min()
            .unwrap()
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
        let mut end: Option<Position> = None;

        // for valid values, return a height from 0 to 25, setting start and end to `pos` where
        // appropriate. On invalid values, including a second `E` or `S`, returns an appropriate
        // `Err` to bubble up.
        let mut process = |pos, b| match b {
            b'a'..=b'z' => Ok(b - b'a'),
            b'\n' => unreachable!(),
            b'S' => Ok(0),
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
        let end = end.ok_or(Self::Err::NoEnd)?;

        let heights = heights.into_boxed_slice();
        let rows = heights.len() / cols;
        Ok(Self {
            heights,
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
    NoEnd,
    DuplicateEnd,
    BadId,
}
