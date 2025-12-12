// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 22 Part 2

/// Constants generated from program input using the following AWK 2-liner:
/// ```awk
/// $1 == "depth:" { printf "pub const DEPTH: u64 = %s;\n", $2 }
/// $1 == "target:" { printf "pub const TARGET: (u64, u64) = (%s);\n", $2 }
/// ```
mod constants;
use constants::{DEPTH, TARGET};
use std::collections::{BinaryHeap, HashMap};

const TARGET_X: u64 = TARGET.0;
const TARGET_Y: u64 = TARGET.1;

const TARGET_LOC: Location = Location {
    x: TARGET.0,
    y: TARGET.1,
};

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Ord, Clone, Copy)]
struct Location {
    x: u64,
    y: u64,
}

impl Location {
    fn geo_index(self, cache: &mut HashMap<Self, u64>) -> u64 {
        match self {
            Location { x: 0, y: 0 } | TARGET_LOC => 0,
            Location { x, y: 0 } => x * 16807,
            Location { x: 0, y } => y * 48271,
            Location { x, y } => {
                Location { x: x - 1, y }.erosion_level(cache)
                    * Location { x, y: y - 1 }.erosion_level(cache)
            }
        }
    }

    fn erosion_level(self, cache: &mut HashMap<Self, u64>) -> u64 {
        // would like to do
        // ```
        // *cache.entry(self).or_insert((self.geo_index(depth, cache) + depth) % 20183)
        // ```
        // but that requires multiple mutable borrows for recursive calls to be able to use the
        // cache

        if let Some(lvl) = cache.get(&self) {
            *lvl
        } else {
            let lvl = (self.geo_index(cache) + DEPTH) % 20183;
            cache.insert(self, lvl);
            lvl
        }
    }
    fn region_type(self, cache: &mut HashMap<Self, u64>) -> RegionType {
        match self.erosion_level(cache) % 3 {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            _ => unreachable!(),
        }
    }

    fn neighbors(self) -> impl Iterator<Item = Self> {
        match self {
            Location { x: 0, y: 0 } => vec![Location { x: 1, y: 0 }, Location { x: 0, y: 1 }],
            Location { x, y: 0 } => vec![
                Location { x, y: 1 },
                Location { x: x + 1, y: 0 },
                Location { x: x - 1, y: 0 },
            ],
            Location { x: 0, y } => vec![
                Location { x: 1, y },
                Location { x: 0, y: y + 1 },
                Location { x: 0, y: y - 1 },
            ],
            Location { x, y } => vec![
                Location { x: x - 1, y },
                Location { x: x + 1, y },
                Location { x, y: y - 1 },
                Location { x, y: y + 1 },
            ],
        }
        .into_iter()
    }

    fn reachable_neighbors(
        self,
        equipped: Equipped,
        cache: &mut HashMap<Self, u64>,
    ) -> impl Iterator<Item = Self> {
        self.neighbors()
            .filter(move |neighbor| neighbor.region_type(cache).can_reach(equipped))
    }
}

#[derive(PartialEq, Debug)]
enum RegionType {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

impl RegionType {
    fn can_reach(self, equipment: Equipped) -> bool {
        match self {
            Self::Rocky => equipment != Equipped::Neither,
            Self::Wet => equipment != Equipped::Torch,
            Self::Narrow => equipment != Equipped::ClimbingGear,
        }
    }

    fn alternate_item(self, current: Equipped) -> Equipped {
        match self {
            Self::Rocky => {
                if current == Equipped::Torch {
                    Equipped::ClimbingGear
                } else {
                    Equipped::Torch
                }
            }
            Self::Wet => {
                if current == Equipped::Neither {
                    Equipped::ClimbingGear
                } else {
                    Equipped::Neither
                }
            }
            Self::Narrow => {
                if current == Equipped::Neither {
                    Equipped::Torch
                } else {
                    Equipped::Neither
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, PartialOrd, Ord)]
enum Equipped {
    ClimbingGear,
    Torch,
    Neither,
}

fn fastest_route_length() -> u64 {
    use std::cmp::Reverse;

    let mut erosion_cache: HashMap<Location, u64> = HashMap::new();

    let mut time_costs: HashMap<(Equipped, Location), u64> = HashMap::new();
    let mut queue: BinaryHeap<Reverse<(u64, Equipped, Location)>> =
        BinaryHeap::from([Reverse((0, Equipped::Torch, Location { x: 0, y: 0 }))]);

    // If every move required changing items it would be switching (7 mins) + moving (1 min), so
    // this is the most pessimistic upper limit. Once a route to TARGET is found, update this with
    // its time cost so that any branches that take longer than it can be pruned moving forwards
    let mut upper_limit = (TARGET_X + TARGET_Y) * 8;

    while let Some(Reverse((time, inventory, loc))) = queue.pop() {
        let id = (inventory, loc);
        if time > upper_limit || time_costs.get(&id).is_some_and(|prev| *prev < time) {
            continue;
        }

        for neighbor in loc.reachable_neighbors(inventory, &mut erosion_cache) {
            let new_time = time + 1;
            if time_costs
                .get(&(inventory, neighbor))
                .is_none_or(|prev| *prev > new_time)
            {
                if (inventory, neighbor) == (Equipped::Torch, TARGET_LOC) {
                    upper_limit = upper_limit.min(new_time);
                    time_costs.retain(|_, time| *time <= upper_limit);
                    queue.retain(|Reverse((time, _, _))| *time <= upper_limit);
                }

                let _ = time_costs.insert((inventory, neighbor), new_time);
                queue.push(Reverse((new_time, inventory, neighbor)));
            }
        }

        if time + 7 > upper_limit {
            continue;
        }

        let alt_item = loc
            .region_type(&mut erosion_cache)
            .alternate_item(inventory);
        if !time_costs.contains_key(&(alt_item, loc)) {
            let _ = time_costs.insert((inventory, loc), time + 7);
            queue.push(Reverse((time + 7, alt_item, loc)));
        }
    }

    time_costs[&(Equipped::Torch, TARGET_LOC)]
}

fn main() {
    println!("{}", fastest_route_length());
}
