// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 19 Part 1

#[derive(Debug)]
struct Diagram {
    data: Vec<DiagramSpace>,
    width: usize,
    start_col: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Location {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq)]
enum DiagramSpace {
    Letter(char),
    Corner,
    Normal,
    Blocked,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn position_of<T, I>(item: T, iter: I) -> Option<usize>
where
    T: PartialEq,
    I: Iterator<Item = T>,
{
    for (i, c) in iter.enumerate() {
        if c == item {
            return Some(i);
        }
    }
    None
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let input: Vec<String> = input.lines().map(str::to_string).collect();
    let width = input[0].chars().count();
    assert!(
        input.iter().skip(1).all(|row| row.chars().count() == width),
        "Inconsistent row sizes"
    );
    let start_col = position_of('|', input[0].chars()).expect("No start column in top row");
    let diagram = Diagram {
        width,
        start_col,
        data: input
            .into_iter()
            .flat_map(|row| {
                row.chars()
                    .map(DiagramSpace::from)
                    .collect::<Vec<_>>()
            })
            .collect(),
    };
    println!("{}", diagram.solve());
}

impl Direction {
    fn rotate_cw(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    fn rotate_ccw(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

impl Diagram {
    fn solve(&self) -> String {
        let mut facing = Direction::Down;
        let mut location = Location {
            row: 0,
            col: self.start_col,
        };
        let mut sequence = String::new();
        loop {
            match self[location] {
                DiagramSpace::Normal => (),
                DiagramSpace::Letter(c) => sequence.push(c),
                DiagramSpace::Corner => {
                    facing = if self
                        .neigbor_of(location, facing.rotate_cw())
                        .is_some_and(|loc| self[loc] != DiagramSpace::Blocked)
                    {
                        facing.rotate_cw()
                    } else {
                        facing.rotate_ccw()
                    };
                }
                DiagramSpace::Blocked => return sequence,
            }
            if let Some(next_loc) = self.neigbor_of(location, facing) {
                location = next_loc;
            } else {
                return sequence;
            }
        }
    }

    fn neigbor_of(&self, loc: Location, direction: Direction) -> Option<Location> {
        match direction {
            Direction::Up => loc.row.checked_sub(1).map(|row| Location { row, ..loc }),
            Direction::Left => loc.col.checked_sub(1).map(|col| Location { col, ..loc }),
            Direction::Down => {
                let row = loc.row + 1;
                if row < self.data.len() / self.width {
                    Some(Location { row, ..loc })
                } else {
                    None
                }
            }
            Direction::Right => {
                let col = loc.col + 1;
                if col < self.width {
                    Some(Location { col, ..loc })
                } else {
                    None
                }
            }
        }
    }
}

impl From<char> for DiagramSpace {
    fn from(c: char) -> Self {
        match c {
            'A'..='Z' => DiagramSpace::Letter(c),
            '|' | '-' => DiagramSpace::Normal,
            '+' => DiagramSpace::Corner,
            _ => DiagramSpace::Blocked,
        }
    }
}

impl std::ops::Index<Location> for Diagram {
    type Output = DiagramSpace;
    fn index(&self, loc: Location) -> &DiagramSpace {
        &self.data[loc.row * self.width + loc.col]
    }
}
