// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 11 Part 1

// Red Blob Games has a great writup of approaches of hexagonal grids.
// https://www.redblobgames.com/grids/hexagons/
//
// The cleanest implementation of a hex grid uses a cubic coordinat system with the restriction
// that the sum of the 3 axes is 0. Because of this restriction, only two of the 3 axes need to
// actually be stored. The cubic coordinate system with an implicit axis is called the axial
// coordinate system, and in the interest of "Making invalid state unrepresentable" while also
// saving on redundant math, I'm using that.
//
// https://www.redblobgames.com/grids/hexagons/#coordinates-axial

#[derive(Debug, Default)]
struct HexPoint {
    r: i32,
    q: i32,
}

impl HexPoint {
    /// returns the implicit s coordinate
    fn s(&self) -> i32 {
        -self.r - self.q
    }

    /// returns the distance from axial position 0, 0
    fn distance_from_origin(&self) -> u32 {
        // https://www.redblobgames.com/grids/hexagons/#distances-cube
        // hexagonal Manhattan distance on cubic coordinates is half of the cubic Manhattan
        // distance.
        (self.q.unsigned_abs() + self.r.unsigned_abs() + self.s().unsigned_abs()) / 2
    }
}

#[derive(Debug, PartialEq)]
enum HexDirection {
    N,
    NE,
    SE,
    S,
    SW,
    NW,
}

#[derive(Debug)]
struct UnknownDirection;

impl std::str::FromStr for HexDirection {
    type Err = UnknownDirection;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use HexDirection as HD;
        match s {
            "n" => Ok(HD::N),
            "ne" => Ok(HD::NE),
            "se" => Ok(HD::SE),
            "s" => Ok(HD::S),
            "sw" => Ok(HD::SW),
            "nw" => Ok(HD::NW),
            _ => Err(UnknownDirection),
        }
    }
}

impl std::ops::AddAssign<HexDirection> for HexPoint {
    fn add_assign(&mut self, rhs: HexDirection) {
        use HexDirection as HD;
        match rhs {
            HD::N => self.q -= 1,
            HD::NE => {
                self.r += 1;
                self.q -= 1;
            }
            HD::SE => self.r += 1,
            HD::S => self.q += 1,
            HD::SW => {
                self.r -= 1;
                self.q += 1;
            }
            HD::NW => self.r -= 1,
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let mut position = HexPoint::default();
    input
        .trim()
        .split(",")
        .map(|s| {
            s.parse::<HexDirection>()
                .expect("failed to parse movement direction")
        })
        .for_each(|d| position += d);
    println!("{}", position.distance_from_origin());
}
