// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 25 Part 1

use std::collections::HashSet;

use std::num::ParseIntError;
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct SpaceTimeCoord {
    x: i8,
    y: i8,
    z: i8,
    time: i8,
}

impl SpaceTimeCoord {
    fn distance(self, other: Self) -> u8 {
        self.x.abs_diff(other.x)
            + self.y.abs_diff(other.y)
            + self.z.abs_diff(other.z)
            + self.time.abs_diff(other.time)
    }
}

#[derive(Debug)]
enum SpaceTimeParseError {
    ParseInt(#[allow(unused)] ParseIntError),
    FieldCount(#[allow(unused)] usize),
}

impl From<ParseIntError> for SpaceTimeParseError {
    fn from(err: ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<Vec<i8>> for SpaceTimeParseError {
    fn from(err: Vec<i8>) -> Self {
        Self::FieldCount(err.len())
    }
}

impl std::str::FromStr for SpaceTimeCoord {
    type Err = SpaceTimeParseError;
    fn from_str(s: &str) -> Result<Self, SpaceTimeParseError> {
        let parts: Vec<i8> = s
            .trim()
            .split(',')
            .map(|part| part.parse())
            .collect::<Result<_, _>>()?;
        let [x, y, z, time] = parts.try_into()?;
        Ok(Self { x, y, z, time })
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let mut constellations: Vec<HashSet<SpaceTimeCoord>> = Vec::new();

    for line in input.lines() {
        let coords: SpaceTimeCoord = line.parse().unwrap();
        let mut constellation: HashSet<SpaceTimeCoord> = constellations
            .extract_if(.., |collection| {
                collection.iter().any(|c| c.distance(coords) <= 3)
            })
            .flatten()
            .collect();
        constellation.insert(coords);
        constellations.push(constellation);
    }

    println!("{}", constellations.len());
}
