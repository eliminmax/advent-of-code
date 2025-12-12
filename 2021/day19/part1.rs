// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 19 Part 1

use std::collections::{HashSet, VecDeque};

mod rotations;
use rotations::ROTATORS;

macro_rules! debug_eprintln {
    () => {
        if cfg!(debug_assertions) {
            eprintln!();
        }
    };
    ($fmt: literal) => {
        if cfg!(debug_assertions) {
            eprintln!($fmt);
        }
    };
    ($fmt: literal, $($toks: expr),+) => {
        if cfg!(debug_assertions) {
            eprintln!($fmt, $($toks,)+);
        }
    };
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position {
    x: i16,
    y: i16,
    z: i16,
}

macro_rules! pos {
    ($x: expr, $y: expr, $z: expr) => {{
        Position {
            x: $x,
            y: $y,
            z: $z,
        }
    }};
}

impl Position {
    const fn sub_const(self, rhs: Self) -> Self {
        pos!(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }

    const fn add_const(self, rhs: Self) -> Self {
        pos!(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Add for Position {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        self.add_const(other)
    }
}

impl std::ops::Sub for Position {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self.sub_const(other)
    }
}

#[derive(Debug, Clone)]
struct Scanner {
    id: u8,
    beacons: Vec<Position>,
}

#[derive(Debug, Clone)]
struct AlignedScanner {
    id: u8,
    beacons: Vec<Position>,
    location: Position,
}

impl Scanner {
    /// transform the scanner into an AlignedScanner, using its own coordinates and alignment
    fn align_to_self(self) -> AlignedScanner {
        AlignedScanner {
            id: self.id,
            beacons: self.beacons,
            location: Position { x: 0, y: 0, z: 0 },
        }
    }
}

impl AlignedScanner {
    fn locate(&self, to_align: &Scanner) -> Option<Self> {
        for rotator in ROTATORS {
            let mut rotated: Vec<Position> =
                to_align.beacons.iter().cloned().map(rotator).collect();
            for aligned_beacon in self.beacons.iter().cloned() {
                for ref_beacon in rotated.iter().cloned() {
                    let matching_count = (rotated.iter())
                        .map(|&b| b - ref_beacon + aligned_beacon)
                        .filter(|b| self.beacons.contains(b))
                        .take(12)
                        .count();
                    if matching_count >= 12 {
                        for beacon in rotated.iter_mut() {
                            *beacon = *beacon - ref_beacon + aligned_beacon;
                        }
                        return Some(Self {
                            id: to_align.id,
                            beacons: rotated,
                            location: aligned_beacon - ref_beacon,
                        });
                    }
                }
            }
        }
        None
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut queue: VecDeque<Scanner> = input
        .trim()
        .split("\n\n")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let mut aligned_scanners: Vec<AlignedScanner> = Vec::with_capacity(queue.len());

    aligned_scanners.push(queue.pop_front().unwrap().align_to_self());

    'outer: while let Some(scanner) = queue.pop_front() {
        debug_eprintln!("Trying to align {}", scanner.id);
        for aligned in aligned_scanners.iter() {
            if let Some(new_aligned) = aligned.locate(&scanner) {
                debug_eprintln!(
                    "\tAligned successfully relative to {} - it's located at {:?}",
                    aligned.id,
                    new_aligned.location
                );
                aligned_scanners.push(new_aligned);
                debug_eprintln!("\t\tQueue now has {} remaining\n\n", queue.len());
                continue 'outer;
            }
        }
        assert!(!queue.is_empty(), "Couldn't align the last scanner");
        debug_eprintln!("\tFailed to align - sending it to back of queue");
        queue.push_back(scanner);
        debug_eprintln!("\t\tQueue now has {} remaining\n", queue.len());
    }

    let beacons: HashSet<Position> = aligned_scanners
        .into_iter()
        .flat_map(|s| s.beacons)
        .collect();
    println!("{}", beacons.len());
}

#[derive(Debug)]
enum PositionParseError {
    BadStructure(#[allow(dead_code)] String),
    IntParse(#[allow(dead_code)] std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for PositionParseError {
    fn from(e: std::num::ParseIntError) -> Self {
        Self::IntParse(e)
    }
}

impl From<&str> for PositionParseError {
    fn from(e: &str) -> Self {
        Self::BadStructure(e.into())
    }
}

impl std::str::FromStr for Position {
    type Err = PositionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(',');
        let x = parts.next().ok_or(s)?.parse()?;
        let y = parts.next().ok_or(s)?.parse()?;
        let z = parts.next().ok_or(s)?.parse()?;
        if parts.next().is_some() {
            Err(PositionParseError::BadStructure(s.into()))
        } else {
            Ok(Self { x, y, z })
        }
    }
}

#[derive(Debug)]
enum ProbeParseError {
    MissingIdLine,
    BadIdLine(#[allow(dead_code)] String),
    BadPosition(#[allow(dead_code)] PositionParseError),
}

impl From<PositionParseError> for ProbeParseError {
    fn from(e: PositionParseError) -> Self {
        Self::BadPosition(e)
    }
}

impl std::str::FromStr for Scanner {
    type Err = ProbeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id_line = lines.next().ok_or(ProbeParseError::MissingIdLine)?;
        let id: u8 = id_line
            .strip_prefix("--- scanner ")
            .and_then(|l| l.strip_suffix(" ---"))
            .and_then(|l| l.parse().ok())
            .ok_or(ProbeParseError::BadIdLine(id_line.into()))?;

        let beacons: Vec<Position> = lines.map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self { id, beacons })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_scanner {
        ($id: literal, [$(($x: literal, $y: literal, $z: literal)),*]) => {{
            Scanner {
                id: $id,
                beacons: vec![$(pos!($x,$y,$z)),*],
            }
        }}
    }

    #[test]
    fn example_overlap() {
        let scanner0 = test_scanner!(
            0,
            [
                (-618, -824, -621),
                (-537, -823, -458),
                (-447, -329, 318),
                (404, -588, -901),
                (544, -627, -890),
                (528, -643, 409),
                (-661, -816, -575),
                (390, -675, -793),
                (423, -701, 434),
                (-345, -311, 381),
                (459, -707, 401),
                (-485, -357, 347)
            ]
        )
        .align_to_self();

        let scanner1 = test_scanner!(
            1,
            [
                (686, 422, 578),
                (605, 423, 415),
                (515, 917, -361),
                (-336, 658, 858),
                (-476, 619, 847),
                (-460, 603, -452),
                (729, 430, 532),
                (-322, 571, 750),
                (-355, 545, -477),
                (413, 935, -424),
                (-391, 539, -444),
                (553, 889, -390)
            ]
        );

        assert_eq!(
            scanner0.locate(&scanner1).map(|aligned| aligned.location),
            Some(pos!(68, -1246, -43))
        );
    }
}
