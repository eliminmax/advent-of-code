// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 24 Part 1

// Copying parts of my work from 2017 Day 11

// One approach to a hexagonal grid is the cubic coordinate system, which has axes q, r, and s, and
// requires that q + r + s is equal to zero. The axial coordinate system is like that, except it
// doesn't store `s`, instead simply computing `-q - r` on the fly when it's needed.
// https://www.redblobgames.com/grids/hexagons/#coordinates-axial
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct HexPoint {
    q: i32,
    r: i32,
}

impl HexPoint {
    fn pos_from(sequence: &str) -> Self {
        let mut new_self = Self::default();
        let mut chars = sequence.chars();

        while let Some(c) = chars.next() {
            match c {
                'n' => match chars.next() {
                    Some('e') => new_self += HexDir::NE,
                    Some('w') => new_self += HexDir::NW,
                    Some(c) => panic!("Invalid direction \"n{}\"", c.escape_default()),
                    None => panic!("North on its own is invalid"),
                },
                's' => match chars.next() {
                    Some('e') => new_self += HexDir::SE,
                    Some('w') => new_self += HexDir::SW,
                    Some(c) => panic!("Invalid direction \"s{}\"", c.escape_default()),
                    None => panic!("South on its own is invalid"),
                },
                'e' => new_self += HexDir::E,
                'w' => new_self += HexDir::W,
                c => panic!("Invalid direction: \"{}\"", c.escape_default()),
            }
        }

        new_self
    }
}

#[derive(Debug, Clone, Copy)]
enum HexDir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

// this is more-or-less identical to the AddAssign<HexDirection> impl from my 2017 day 11 part 1
// solution, with a shortened type name, and the axes rotated - it's literally copied-and-pasted
// then edited in that way.
impl std::ops::AddAssign<HexDir> for HexPoint {
    fn add_assign(&mut self, rhs: HexDir) {
        match rhs {
            HexDir::E => self.q -= 1,
            HexDir::SE => {
                self.r += 1;
                self.q -= 1;
            }
            HexDir::SW => self.r += 1,
            HexDir::W => self.q += 1,
            HexDir::NW => {
                self.r -= 1;
                self.q += 1;
            }
            HexDir::NE => self.r -= 1,
        }
    }
}

fn main() {
    use std::collections::HashSet;
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut flipped: HashSet<HexPoint> = HashSet::new();
    for line in input.lines() {
        let tile = HexPoint::pos_from(line.trim());
        // if HashSet::insert returns false, it already has the element in question, so it should
        // be removed.
        if !flipped.insert(tile) {
            flipped.remove(&tile);
        }
    }
    println!("{}", flipped.len());
}
