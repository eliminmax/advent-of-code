// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 13 Part 2

use std::collections::HashMap;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    /// Converts a char to a Direction, panicking if it's not one of '<', '>', '^', or 'v'.
    fn from_char(c: char) -> Self {
        match c {
            'v' => Direction::Down,
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            _ => panic!(),
        }
    }

    fn rotate_pos(&mut self) {
        *self = match self {
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn rotate_neg(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Down,
        }
    }

    fn rotate_cw(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn rotate_ccw(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
enum NextTurn {
    Left,
    Forwards,
    Right,
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, PartialOrd, Ord)]
struct Location {
    // y comes first so that sort prioritizes row over column
    y: usize,
    x: usize,
}

impl Location {
    fn new(x: usize, y: usize) -> Self {
        Location { x, y }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum TrackSpace {
    Vertical,
    Horizontal,
    Intersection,
    /// Curved away from a positive diagonal (i.e. '/')
    CurvedPos,
    /// Curved away from a negative diagonal (i.e. '\\')
    CurvedNeg,
}

#[derive(Debug, PartialEq, Clone)]
struct Elf {
    facing: Direction,
    location: Location,
    next_turn: NextTurn,
}

type TrackInfo = HashMap<Location, TrackSpace>;

impl Elf {
    fn advance(&mut self, tracks: &TrackInfo) {
        match self.facing {
            Direction::Up => self.location.y -= 1,
            Direction::Down => self.location.y += 1,
            Direction::Left => self.location.x -= 1,
            Direction::Right => self.location.x += 1,
        }
        match tracks[&self.location] {
            TrackSpace::CurvedNeg => self.facing.rotate_neg(),
            TrackSpace::CurvedPos => self.facing.rotate_pos(),
            TrackSpace::Intersection => match self.next_turn {
                NextTurn::Left => {
                    self.facing.rotate_ccw();
                    self.next_turn = NextTurn::Forwards;
                }
                NextTurn::Forwards => {
                    self.next_turn = NextTurn::Right;
                }
                NextTurn::Right => {
                    self.facing.rotate_cw();
                    self.next_turn = NextTurn::Left;
                }
            },
            _ => (),
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input: Vec<String> = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!")
        .lines()
        .map(str::to_string)
        .collect();
    let mut elves: Vec<Elf> = Vec::new();
    let mut tracks = TrackInfo::new();
    for (y, row) in input.into_iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let _ = match c {
                '|' => tracks.insert(Location::new(x, y), TrackSpace::Vertical),
                '-' => tracks.insert(Location::new(x, y), TrackSpace::Horizontal),
                '+' => tracks.insert(Location::new(x, y), TrackSpace::Intersection),
                '/' => tracks.insert(Location::new(x, y), TrackSpace::CurvedPos),
                '\\' => tracks.insert(Location::new(x, y), TrackSpace::CurvedNeg),
                '<' | '>' => {
                    let location = Location::new(x, y);
                    elves.push(Elf {
                        location,
                        facing: Direction::from_char(c),
                        next_turn: NextTurn::Left,
                    });
                    tracks.insert(location, TrackSpace::Horizontal)
                }
                '^' | 'v' => {
                    let location = Location::new(x, y);
                    elves.push(Elf {
                        location,
                        facing: Direction::from_char(c),
                        next_turn: NextTurn::Left,
                    });
                    tracks.insert(location, TrackSpace::Vertical)
                }
                ' ' => None,
                _ => panic!("Invalid character: {:?}", c),
            }
            .is_none_or(|_| unreachable!());
        }
    }
    let mut occupied: HashMap<Location, usize> = elves
        .iter()
        .enumerate()
        .map(|(i, elf)| (elf.location, i))
        .collect();
    while elves.len() > 1 {
        elves.sort_by(|a, b| a.location.cmp(&b.location));
        let mut crashed: Vec<usize> = Vec::new();
        'inner: for (i, elf) in elves.iter_mut().enumerate() {
            if crashed.contains(&i) {
                continue 'inner;
            }
            let _ = occupied.remove(&elf.location);
            elf.advance(&tracks);
            if let Some(other) = occupied.insert(elf.location, i) {
                crashed.push(i);
                crashed.push(other);
                let _ = occupied.remove(&elf.location);
            }
        }
        crashed.sort_by(|a, b| b.cmp(a));
        for i in crashed.into_iter() {
            let _ = elves.remove(i);
        }
    }
    println!("{},{}", elves[0].location.x, elves[0].location.y);
}
