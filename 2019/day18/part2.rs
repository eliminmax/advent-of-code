// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 18 Part 2

// Built with the assumption that the modification for part 2 splits it into 4 separate
// quarters, so each key can only be reached by 1 specific bot. This is true of my input, but not
// guaranteed to be true generally, and is not true of all of the sample inputs.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

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
    starts: [Location; 4],
}

impl MazeGrid {
    fn reachable(&self, inventory: Inventory, bot: Location) -> HashMap<Key, (Location, u32)> {
        let mut queue = BinaryHeap::from([Reverse((0, bot))]);
        let mut distances: HashMap<Location, u32> = HashMap::new();
        let mut key_distances: HashMap<Key, (Location, u32)> = HashMap::new();

        while let Some(Reverse((dist, loc))) = queue.pop() {
            if distances.get(&loc).is_some_and(|prev| *prev < dist) {
                continue;
            }

            'neighbors: for neighbor in loc.neighbors() {
                let new_dist = dist + 1;
                match self.grid_data.get(&neighbor).cloned() {
                    Some(GridSpace::Key(key)) => {
                        if key_distances
                            .get(&key)
                            .is_none_or(|&(_, prev)| prev > new_dist)
                        {
                            key_distances.insert(key, (neighbor, new_dist));
                        }
                    }
                    Some(GridSpace::Door(d)) if inventory.have_key(d.needed_key) => {
                        if distances.get(&neighbor).is_none_or(|prev| *prev > new_dist) {
                            distances.insert(neighbor, new_dist);
                            queue.push(Reverse((new_dist, neighbor)));
                        }
                    }
                    Some(GridSpace::Open) => {
                        if distances.get(&neighbor).is_none_or(|prev| *prev > new_dist) {
                            distances.insert(neighbor, new_dist);
                            queue.push(Reverse((new_dist, neighbor)));
                        }
                    }
                    _ => continue 'neighbors,
                }
            }
        }

        key_distances
    }

    fn solve(&self) -> u32 {
        type Node = (Inventory, [Location; 4]);
        type SubMap = HashMap<Key, (Location, u32)>;

        // relative distance for the given inventory and starting location to all reachable keys
        // for a given inventory and starting location, this does not change, so each entry can be
        // calculated once when needed, and reused from that point on.
        let mut link_distances: HashMap<(Inventory, Location), SubMap> = HashMap::new();

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

        let mut queue = BinaryHeap::from([Reverse((0, Inventory::default(), self.starts))]);
        let mut min_dist: Option<u32> = None;

        while let Some(Reverse((dist, inv, locs))) = queue.pop() {
            // skip the branch if it's at least as long than the minimum distance seen.
            if min_dist.is_some_and(|md| md <= dist)
                || distances.get(&(inv, locs)).is_some_and(|prev| *prev < dist)
            {
                continue;
            }

            for i in 0..4 {
                let sm: &SubMap = link_distances
                    .entry((inv, locs[i]))
                    .or_insert(self.reachable(inv, locs[i]));

                'keys: for (&key, &(loc, key_dist)) in sm.iter() {
                    let next_dist = dist + key_dist;

                    if min_dist.is_some_and(|md| md <= next_dist) {
                        continue 'keys;
                    }

                    let mut next_inv = inv;
                    next_inv.pick_up(key);

                    if next_inv == full_inventory {
                        if min_dist.is_none_or(|md| md > next_dist) {
                            min_dist = Some(next_dist);
                            queue.retain(|Reverse((dist, _, _))| *dist < next_dist);
                        }
                        continue 'keys;
                    }

                    let mut next_locs = locs;
                    next_locs[i] = loc;

                    if distances
                        .get(&(next_inv, next_locs))
                        .is_none_or(|prev| *prev > next_dist)
                    {
                        distances.insert((next_inv, next_locs), next_dist);
                        queue.push(Reverse((next_dist, next_inv, next_locs)));
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
        Some(self.cmp(other))
    }
}
impl Ord for Inventory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0.count_ones(), self.0).cmp(&(other.0.count_ones(), other.0))
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
                        for neighbor in loc.neighbors() {
                            grid_data.insert(neighbor, GridSpace::Blocked);
                        }

                        grid_data.entry(loc).or_insert(GridSpace::Open);
                    }
                    '.' => {
                        grid_data.entry(loc).or_insert(GridSpace::Open);
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
            let starts = [
                Location {
                    x: start.x - 1,
                    y: start.y - 1,
                },
                Location {
                    x: start.x + 1,
                    y: start.y - 1,
                },
                Location {
                    x: start.x - 1,
                    y: start.y + 1,
                },
                Location {
                    x: start.x + 1,
                    y: start.y + 1,
                },
            ];
            Ok(Self { grid_data, starts })
        } else {
            Err(MapParseError::MissingStart)
        }
    }
}
