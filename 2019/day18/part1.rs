// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 18 Part 1

use std::collections::HashMap;

// Used as 26 bitfields to represent the inventory
//
// for a lowercase ASCII letter LETTER, `data & (1 << (LETTER as u8) - b'a') != 0`
// represents whether the letter is in use or not.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Inventory(u32);

impl Inventory {
    fn pick_up(&mut self, Key(raw_key): Key) {
        self.0 |= 1_u32 << u32::from(raw_key);
    }

    fn have_key(&self, Key(raw_key): Key) -> bool {
        self.0 & 1_u32 << u32::from(raw_key) != 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Key(u8);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Door {
    needed_key: Key,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Location {
    y: u8,
    x: u8,
}

impl Location {
    fn neighbors(self) -> impl Iterator<Item = Self> {
        [
            self.x.checked_sub(1).map(|x| Self { x, ..self }),
            self.y.checked_sub(1).map(|y| Self { y, ..self }),
            Some(Self {
                x: self.x + 1,
                ..self
            }),
            Some(Self {
                y: self.y + 1,
                ..self
            }),
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum GridSpace {
    Blocked,
    Open,
    Door(Door),
    Key(Key),
}

#[derive(Debug)]
struct MazeGrid {
    grid_data: HashMap<Location, GridSpace>,
    start: Location,
}

impl MazeGrid {
    fn solve(&self) -> u32 {
        type Node = (Inventory, Location);
        let mut distances: HashMap<Node, u32> = HashMap::new();
        let full_inventory = self
            .grid_data
            .values()
            .fold(Inventory::default(), |mut inv, x| {
                if let GridSpace::Key(k) = x {
                    inv.pick_up(*k);
                }
                inv
            });

        let mut queue = vec![(0, Inventory::default(), self.start)];
        let mut min_dist: Option<u32> = None;

        while let Some((dist, inv, loc)) = queue.pop() {
            // skip the branch if it's at least as long than the minimum distance seen.
            if min_dist.is_some_and(|md| md <= dist) {
                continue;
            }

            if distances.get(&(inv, loc)).is_some_and(|prev| *prev < dist) {
                continue;
            }

            'neighbors: for neighbor in loc.neighbors() {
                let new_dist = dist + 1;

                macro_rules! checked_add_to_queue {
                    ($inv: ident) => {
                        if distances
                            .get(&($inv, neighbor))
                            .is_none_or(|prev| *prev > new_dist)
                        {
                            distances.insert(($inv, neighbor), new_dist);
                            let i = queue.partition_point(|&a| {
                                a.1.partial_cmp(&$inv).unwrap_or_else(|| a.0.cmp(&new_dist))
                                    != std::cmp::Ordering::Less
                            });
                            queue.insert(i, (new_dist, $inv, neighbor));
                        }
                    };
                    () => {
                        checked_add_to_queue!(inv)
                    };
                }

                match self.grid_data.get(&neighbor).cloned() {
                    None | Some(GridSpace::Blocked) => continue 'neighbors,
                    Some(GridSpace::Open) => checked_add_to_queue!(),
                    Some(GridSpace::Door(d)) => {
                        if inv.have_key(d.needed_key) {
                            checked_add_to_queue!()
                        }
                    }
                    Some(GridSpace::Key(k)) => {
                        let mut new_inv = inv;
                        new_inv.pick_up(k);
                        if new_inv == full_inventory {
                            min_dist = Some(new_dist);
                            continue 'neighbors;
                        }
                        checked_add_to_queue!(new_inv);
                    }
                }
            }
        }

        min_dist.expect("no minimal path found")
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let grid: MazeGrid = input.parse().unwrap();
    println!("{}", grid.solve());
}

// Implementations of basic traits helpful for constructing the solution

impl PartialOrd for Inventory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.count_ones().partial_cmp(&other.0.count_ones())
    }
}

#[derive(Debug)]
struct DoorKeyCreateError(#[allow(dead_code, reason = "field exists for Debug impl")] char);

impl TryFrom<char> for Door {
    type Error = DoorKeyCreateError;
    fn try_from(k: char) -> Result<Self, Self::Error> {
        if k.is_ascii_uppercase() {
            Ok(Self {
                needed_key: Key(k as u8 - b'A'),
            })
        } else {
            Err(DoorKeyCreateError(k))
        }
    }
}

impl TryFrom<char> for Key {
    type Error = DoorKeyCreateError;
    fn try_from(k: char) -> Result<Self, Self::Error> {
        if k.is_ascii_lowercase() {
            Ok(Self(k as u8 - b'a'))
        } else {
            Err(DoorKeyCreateError(k))
        }
    }
}

impl std::fmt::Debug for Inventory {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let held: Vec<_> = (b'a'..=b'z')
            .filter_map(|l| {
                if self.have_key(Key(l - b'a')) {
                    Some(char::from(l).to_string())
                } else {
                    None
                }
            })
            .collect();

        write!(fmt, "Inventory {{{}}}", held.join(", "))
    }
}

#[allow(dead_code, reason = "field exists for derived Debug")]
#[derive(Debug)]
enum MapParseError {
    MapTooLarge,
    InvalidGridSpace(char),
    MissingStart,
    MultipleStarts(Location, Location),
    MissingKeys(Inventory),
}

impl std::str::FromStr for MazeGrid {
    type Err = MapParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid_data = HashMap::new();
        let mut start = None;
        let mut keys = Inventory::default();
        let mut doors = Inventory::default();

        for (y, line) in s.lines().enumerate() {
            let y = y.try_into().map_err(|_| MapParseError::MapTooLarge)?;
            for (x, b) in line.chars().enumerate() {
                let loc = Location {
                    y,
                    x: x.try_into().map_err(|_| MapParseError::MapTooLarge)?,
                };
                match b {
                    'a'..='z' => {
                        let k = b.try_into().expect("always in valid range");
                        keys.pick_up(k);
                        grid_data.insert(loc, GridSpace::Key(b.try_into().unwrap()));
                    }
                    'A'..='Z' => {
                        doors.pick_up(
                            b.to_ascii_lowercase()
                                .try_into()
                                .expect("always in valid range"),
                        );
                        grid_data.insert(loc, GridSpace::Door(b.try_into().unwrap()));
                    }
                    '@' => {
                        if let Some(prev_start) = start.replace(loc) {
                            return Err(MapParseError::MultipleStarts(prev_start, loc));
                        }
                        grid_data.insert(loc, GridSpace::Open);
                    }
                    '.' => {
                        grid_data.insert(loc, GridSpace::Open);
                    }
                    '#' => {
                        grid_data.insert(loc, GridSpace::Blocked);
                    }
                    c => return Err(MapParseError::InvalidGridSpace(c)),
                }
            }
        }
        if doors.0 & !keys.0 != 0 {
            Err(MapParseError::MissingKeys(Inventory(doors.0 & !keys.0)))
        } else if let Some(start) = start {
            Ok(Self { grid_data, start })
        } else {
            Err(MapParseError::MissingStart)
        }
    }
}
