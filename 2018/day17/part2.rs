// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 17 Part 2

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Clone, Copy)]
struct Location {
    // y first to ensure that it gets higher sort priority
    y: u16,
    x: u16,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum GroundContents {
    FlowingWater,
    // it's OK, the North Pole is too cold for mosquitoes
    StillWater,
    Clay,
}

#[derive(Debug)]
struct Underground {
    grid_contents: BTreeMap<Location, GroundContents>,
    min_y: u16,
    max_y: u16,
}

impl Underground {
    fn get(&self, loc: Location) -> Option<&GroundContents> {
        self.grid_contents.get(&loc)
    }

    fn water_fill(&mut self, start: Location) {
        // using a BTreeSet to ensure that queue doesn't have duplicate elements, and that it's
        // possible to defer handling of upper layers until lower ones they depend on are
        // processed.
        let mut queue: BTreeSet<Location> = BTreeSet::from([start]);
        while let Some(loc) = queue.pop_last() {
            if self.get(loc).is_some() {
                continue;
            }
            if loc.y == self.max_y {
                self.grid_contents.insert(loc, GroundContents::FlowingWater);
                continue;
            }
            let below_loc = Location {
                y: loc.y + 1,
                ..loc
            };
            match self.get(below_loc) {
                None => {
                    // not enough information - determine what's below, then try again
                    queue.insert(below_loc);
                    queue.insert(loc);
                }
                Some(GroundContents::FlowingWater) => {
                    self.grid_contents.insert(loc, GroundContents::FlowingWater);
                }
                Some(GroundContents::Clay) | Some(GroundContents::StillWater) => {
                    let mut current_loc = Location {
                        x: loc.x - 1,
                        ..loc
                    };
                    let mut top_layer = false;
                    let mut layer_locs: BTreeSet<Location> = BTreeSet::from([loc]);
                    macro_rules! directional_spread {
                        ($op: tt) => {{
                            while self.get(current_loc).is_none() {
                                let current_below_loc = Location {
                                    y: current_loc.y + 1,
                                    ..current_loc
                                };
                                if self.get(current_below_loc).is_none() {
                                    self.water_fill(current_loc);
                                    // The following simple check took me days to figure out. I had
                                    // everything else in this file more-or-less as-is, but it did
                                    // not handle cases like the following properly:
                                    // #......#
                                    // #.#..#.#
                                    // #.####.#
                                    // #......#
                                    // ########
                                    if self.get(current_loc) ==
                                        Some(&GroundContents::FlowingWater) {
                                        top_layer = true;
                                    }
                                    break;
                                }
                                layer_locs.insert(current_loc);
                                current_loc.x $op 1;
                            }
                        }}
                    }
                    directional_spread!(-=);
                    current_loc = Location {
                        x: loc.x + 1,
                        ..loc
                    };
                    directional_spread!(+=);
                    let layer_member_type = if top_layer {
                        GroundContents::FlowingWater
                    } else {
                        GroundContents::StillWater
                    };
                    layer_locs.into_iter().for_each(|l| {
                        self.grid_contents.insert(l, layer_member_type);
                    });
                }
            }
        }
    }

    fn remaining_water_amount(&mut self) -> usize {
        self.water_fill(Location { x: 500, y: 0 });
        // need to copy these values out to avoid borrowing from self while modifying self.
        let min_y = self.min_y;
        let max_y = self.max_y;
        self.grid_contents
            .retain(|&Location { y, .. }, _gc| y >= min_y && y <= max_y);
        self.grid_contents
            .values()
            .filter(|&gc| *gc == GroundContents::StillWater)
            .count()
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut underground: Underground = input.parse().expect("Failed to parse water system");
    println!("{}", underground.remaining_water_amount());
}

#[derive(Debug)]
enum UndergroundParseError {
    IntParseFailure(#[allow(unused)] std::num::ParseIntError),
    BadFormat(#[allow(unused)] Box<str>),
}

impl From<&str> for UndergroundParseError {
    fn from(e: &str) -> Self {
        Self::BadFormat(Box::from(e))
    }
}

impl From<std::num::ParseIntError> for UndergroundParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::IntParseFailure(e)
    }
}

impl std::str::FromStr for Underground {
    type Err = UndergroundParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid_contents: BTreeMap<Location, GroundContents> = BTreeMap::new();
        type SpannerFn = fn(u16, u16, u16) -> Vec<Location>;
        let vertical_spanner: SpannerFn = |x, start, stop| {
            (start..=stop)
                .map(|y| Location { x, y })
                .collect::<Vec<_>>()
        };
        let horizontal_spanner: SpannerFn = |y, start, stop| {
            (start..=stop)
                .map(|x| Location { x, y })
                .collect::<Vec<_>>()
        };

        let mut max_y: u16 = 0;
        let mut min_y: u16 = u16::MAX;
        for line in s.lines() {
            let (offset, span) = line.split_once(", ").ok_or(line)?;
            let (offset_axis, offset_coordinate) = offset.split_once('=').ok_or(line)?;
            let offset_coordinate: u16 = offset_coordinate.parse()?;

            let (spanner, prefix): (SpannerFn, _) = match offset_axis {
                "x" => Ok((vertical_spanner, "y=")),
                "y" => Ok((horizontal_spanner, "x=")),
                _ => Err(line),
            }?;

            let (start, stop) = span
                .strip_prefix(prefix)
                .and_then(|l| l.split_once(".."))
                .ok_or(line)?;

            let start: u16 = start.parse()?;
            let stop: u16 = stop.parse()?;
            if prefix == "y=" {
                max_y = max_y.max(stop);
                min_y = min_y.min(start);
            } else {
                max_y = max_y.max(offset_coordinate);
                min_y = min_y.min(offset_coordinate);
            }

            for location in spanner(offset_coordinate, start, stop).into_iter() {
                grid_contents.insert(location, GroundContents::Clay);
            }
        }
        Ok(Underground {
            grid_contents,
            min_y,
            max_y,
        })
    }
}
