// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Rust 2024 edition

// Solution to AoC 2018 Day 20 Part 2

use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Clone, Copy)]
enum Instruction {
    Move(Direction),
    OptionStart,
    Divider,
    OptionStop,
}

impl TryFrom<u8> for Instruction {
    type Error = u8;
    fn try_from(byte: u8) -> Result<Self, u8> {
        match byte {
            b'N' => Ok(Self::Move(Direction::North)),
            b'E' => Ok(Self::Move(Direction::East)),
            b'S' => Ok(Self::Move(Direction::South)),
            b'W' => Ok(Self::Move(Direction::West)),
            b'(' => Ok(Self::OptionStart),
            b'|' => Ok(Self::Divider),
            b')' => Ok(Self::OptionStop),
            b => Err(b),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
struct RoomLocation {
    y: i16,
    x: i16,
}

/// A room within the facility
///
/// essentially implemented as a bitfield of directions
#[derive(PartialEq, Default, Clone, Copy)]
struct Room(u8);

impl Room {
    const fn check_if_open(&self, dir: Direction) -> bool {
        self.0 & (dir as u8) != 0
    }
    const fn set_open(&mut self, dir: Direction) {
        self.0 |= dir as u8
    }
}

impl From<Instruction> for char {
    fn from(other: Instruction) -> Self {
        match other {
            Instruction::Move(Direction::North) => 'N',
            Instruction::Move(Direction::West) => 'W',
            Instruction::Move(Direction::South) => 'S',
            Instruction::Move(Direction::East) => 'E',
            Instruction::OptionStart => '(',
            Instruction::Divider => '|',
            Instruction::OptionStop => ')',
        }
    }
}
impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
enum Direction {
    North = 1 << 0,
    East = 1 << 1,
    South = 1 << 2,
    West = 1 << 3,
}

type FacilityMap = HashMap<RoomLocation, Room>;

fn neighbors(room: Room, loc: RoomLocation) -> impl Iterator<Item = RoomLocation> {
    macro_rules! neighbor_check {
        ($dir: expr, x = $x: expr, y = $y: expr) => {{
            room.check_if_open($dir)
                .then_some(RoomLocation { x: $x, y: $y })
        }};
    }
    [
        neighbor_check!(Direction::North, x = loc.x, y = loc.y - 1),
        neighbor_check!(Direction::East, x = loc.x + 1, y = loc.y),
        neighbor_check!(Direction::South, x = loc.x, y = loc.y + 1),
        neighbor_check!(Direction::West, x = loc.x - 1, y = loc.y),
    ]
    .into_iter()
    .flatten()
}


/// The quick-and-dirty logic to process `regex` and return the distance to the furthest room.
/// Panics on error
fn process_pattern(regex: &[u8]) -> u32 {

    assert!(regex.starts_with(b"^") && regex.ends_with(b"$"));

    let mut facility = FacilityMap::new();
    let mut location_stack = Vec::new();
    let mut location = RoomLocation { x: 0, y: 0 };

    let seq = regex[1..regex.len() - 1].iter().map(|i| Instruction::try_from(*i).unwrap());
    for step in seq {
        match step {
                Instruction::Move(dir) => {
                    facility.entry(location).or_default().set_open(dir);
                    match dir {
                        Direction::North => location.y -= 1,
                        Direction::East => location.x += 1,
                        Direction::South => location.y += 1,
                        Direction::West => location.x -= 1,
                    }
                    facility.entry(location).or_default().set_open(dir.invert());
                }
                Instruction::OptionStart => {
                    location_stack.push(location);
                }
                Instruction::Divider => {
                    location = *location_stack.last().unwrap();
                }
                Instruction::OptionStop => {
                    location_stack.pop();
                }
        }
    }

    find_max_distance(facility)
}

fn main() {
    use std::env::args;
    use std::fs::read;
    let raw_input =
        read(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    println!("{}", process_pattern(raw_input.trim_ascii_end()));
}

fn find_max_distance(map: HashMap<RoomLocation, Room>) -> u32 {
    use std::cmp::Reverse as Rev;
    let mut distances: HashMap<RoomLocation, u32> = HashMap::new();
    let mut queue: BinaryHeap<Rev<(u32, RoomLocation)>> =
        BinaryHeap::from([Rev((0_u32, RoomLocation { x: 0, y: 0 }))]);
    while let Some(Rev((dist, loc))) = queue.pop() {
        if distances.get(&loc).is_some_and(|prev| *prev < dist) {
            continue;
        }
        for neighbor in neighbors(map[&loc], loc) {
            let new_dist = dist + 1;
            if distances.get(&neighbor).is_none_or(|prev| *prev > new_dist) {
                let _ = distances.insert(neighbor, new_dist);
                queue.push(Rev((new_dist, neighbor)));
            }
        }
    }
    distances.into_values().max().unwrap()
}

impl Direction {
    const fn invert(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}
