// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 22 Part 1

/// Constants generated from program input using the following AWK 2-liner:
/// ```awk
/// $1 == "depth:" { printf "pub const DEPTH: u64 = %s;\n", $2 }
/// $1 == "target:" { printf "pub const TARGET: (u64, u64) = (%s);\n", $2 }
/// ```
mod constants;
use constants::{DEPTH, TARGET};
use std::collections::HashMap;

const TARGET_X: u64 = TARGET.0;
const TARGET_Y: u64 = TARGET.1;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Location {
    x: u64,
    y: u64,
}

impl Location {
    fn geo_index(self, cache: &mut HashMap<Self, u64>) -> u64 {
        match self {
            Location { x: 0, y: 0 }
            | Location {
                x: TARGET_X,
                y: TARGET_Y,
            } => 0,
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
}
#[derive(PartialEq, Debug)]
enum RegionType {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

fn main() {
    let mut index_cache: HashMap<Location, u64> = HashMap::with_capacity(
        usize::try_from((TARGET_X + 1) * (TARGET_Y + 1)).expect("size small enough for usize"),
    );

    let risk_level: u64 = (0..=TARGET_X)
        .flat_map(|x| (0..=TARGET_Y).map(move |y| Location { x, y }))
        .map(|loc| loc.region_type(&mut index_cache) as u64)
        .sum();
    println!("{risk_level}");
}
