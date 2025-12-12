// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 15 Part 1

mod intcode;
use intcode::Interpreter;
use std::collections::{BTreeMap, BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Location {
    y: i16,
    x: i16,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Location {
    fn neighbors(self) -> [(Self, Direction); 4] {
        [
            (
                Self {
                    y: self.y - 1,
                    ..self
                },
                Direction::North,
            ),
            (
                Self {
                    y: self.y + 1,
                    ..self
                },
                Direction::South,
            ),
            (
                Self {
                    x: self.x - 1,
                    ..self
                },
                Direction::West,
            ),
            (
                Self {
                    x: self.x + 1,
                    ..self
                },
                Direction::East,
            ),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RoomType {
    Wall,
    Open,
    Oxy,
}

fn build_map(interpreter: Interpreter) -> HashMap<Location, RoomType> {
    let mut rooms: HashMap<Location, RoomType> =
        HashMap::from([(Location { x: 0, y: 0 }, RoomType::Open)]);
    let mut to_process: BTreeMap<Location, Vec<Direction>> =
        BTreeMap::from([(Location { x: 0, y: 0 }, Vec::new())]);

    while let Some((room, path)) = to_process.pop_first() {
        let (output, state) = interpreter
            .clone()
            .run_through_inputs(path.iter().map(|d| *d as i64))
            .unwrap();
        debug_assert!(output.iter().all(|i| matches!(*i, 0..=2)));

        let room_type = match output.last().cloned().unwrap_or(1) {
            0 => RoomType::Wall,
            1 => RoomType::Open,
            2 => RoomType::Oxy,
            i => panic!("Invalid status code {i} reported"),
        };
        rooms.insert(room, room_type);
        if room_type == RoomType::Wall || state == intcode::State::Halted {
            continue;
        }
        for (loc, dir) in room.neighbors() {
            if !rooms.contains_key(&loc) {
                to_process.entry(loc).or_insert({
                    let mut v = path.clone();
                    v.push(dir);
                    v
                });
            }
        }
    }

    rooms
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let interpreter = Interpreter::new(input.trim().split(",").map(|i| i.parse().unwrap()));
    println!("{}", find_max_distance(build_map(interpreter)));
}

/// Implementation of Dijkstra's algorithm to find the length of the shortest path from 0, 0 to the
/// oxygen
fn find_max_distance(map: HashMap<Location, RoomType>) -> u32 {
    use std::cmp::Reverse as Rev;

    let mut distances: HashMap<Location, u32> = HashMap::with_capacity(map.len());
    let mut queue: BinaryHeap<Rev<(u32, Location)>> = BinaryHeap::with_capacity(map.len());
    queue.push(Rev((0, Location { x: 0, y: 0 })));
    while let Some(Rev((dist, loc))) = queue.pop() {
        if distances.get(&loc).is_some_and(|prev| *prev < dist) {
            continue;
        }

        for (neighbor, _) in loc.neighbors().into_iter() {
            if map[&neighbor] == RoomType::Wall {
                continue;
            }
            let new_dist = dist + 1;
            if distances.get(&neighbor).is_none_or(|prev| *prev > new_dist) {
                distances.insert(neighbor, new_dist);
                queue.push(Rev((new_dist, neighbor)));
            }
        }
    }

    map.into_iter()
        .filter(|(_, r)| *r == RoomType::Oxy)
        .map(|(l, _)| distances[&l])
        .min()
        .unwrap()
}
