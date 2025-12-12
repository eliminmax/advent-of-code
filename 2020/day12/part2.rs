// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 12 Part 2
use std::num::{self, NonZeroU8};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Waypoint {
    dx: i32,
    dy: i32,
}

impl Waypoint {
    const fn turn_right(&mut self, angle: TurnAngle) {
        match angle {
            TurnAngle::Deg90 => {
                *self = Self {
                    dx: -self.dy,
                    dy: self.dx,
                }
            }
            TurnAngle::Deg180 => {
                *self = Self {
                    dy: -self.dy,
                    dx: -self.dx,
                }
            }
            TurnAngle::Deg270 => {
                *self = Self {
                    dx: self.dy,
                    dy: -self.dx,
                }
            }
        }
    }

    const fn turn_left(&mut self, angle: TurnAngle) {
        match angle {
            TurnAngle::Deg90 => {
                *self = Self {
                    dx: self.dy,
                    dy: -self.dx,
                }
            }
            TurnAngle::Deg180 => {
                *self = Self {
                    dy: -self.dy,
                    dx: -self.dx,
                }
            }
            TurnAngle::Deg270 => {
                *self = Self {
                    dx: -self.dy,
                    dy: self.dx,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
enum TurnAngle {
    Deg90 = 1,
    Deg180 = 2,
    Deg270 = 3,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum ShipMovement {
    North(NonZeroU8),
    East(NonZeroU8),
    South(NonZeroU8),
    West(NonZeroU8),
    Right(TurnAngle),
    Left(TurnAngle),
    Forwards(NonZeroU8),
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct Ship {
    waypoint: Waypoint,
    y: i32,
    x: i32,
}

impl Ship {
    const fn run_move(&mut self, ship_move: ShipMovement) {
        match ship_move {
            ShipMovement::North(i) => self.waypoint.dy -= i.get() as i32,
            ShipMovement::East(i) => self.waypoint.dx += i.get() as i32,
            ShipMovement::South(i) => self.waypoint.dy += i.get() as i32,
            ShipMovement::West(i) => self.waypoint.dx -= i.get() as i32,
            ShipMovement::Forwards(i) => {
                self.y += self.waypoint.dy * (i.get() as i32);
                self.x += self.waypoint.dx * (i.get() as i32);
            }
            ShipMovement::Left(a) => self.waypoint.turn_left(a),
            ShipMovement::Right(a) => self.waypoint.turn_right(a),
        }
    }

    #[inline]
    const fn distance_from_start(&self) -> u32 {
        self.y.unsigned_abs() + self.x.unsigned_abs()
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut ship = Ship::default();
    for ship_move in input.lines().map(|l| l.parse::<ShipMovement>().unwrap()) {
        ship.run_move(ship_move);
    }
    println!("{}", ship.distance_from_start());
}

#[derive(Debug)]
enum ShipMovementParseError {
    EmptyString,
    BadPrefix(#[allow(dead_code)] char),
    NonNumeric(#[allow(dead_code)] num::ParseIntError),
    BadAngle(#[allow(dead_code)] i16),
}

impl std::str::FromStr for ShipMovement {
    type Err = ShipMovementParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().ok_or(Self::Err::EmptyString)? {
            'N' => Ok(Self::North(s.strip_prefix('N').unwrap().parse()?)),
            'E' => Ok(Self::East(s.strip_prefix('E').unwrap().parse()?)),
            'S' => Ok(Self::South(s.strip_prefix('S').unwrap().parse()?)),
            'W' => Ok(Self::West(s.strip_prefix('W').unwrap().parse()?)),
            'R' => {
                let angle: i16 = s.strip_prefix('R').unwrap().parse()?;
                match angle.rem_euclid(360) {
                    90 => Ok(Self::Right(TurnAngle::Deg90)),
                    180 => Ok(Self::Right(TurnAngle::Deg180)),
                    270 => Ok(Self::Right(TurnAngle::Deg270)),
                    a => Err(Self::Err::BadAngle(a)),
                }
            }
            'L' => {
                let angle: i16 = s.strip_prefix('L').unwrap().parse()?;
                match angle.rem_euclid(360) {
                    90 => Ok(Self::Left(TurnAngle::Deg90)),
                    180 => Ok(Self::Left(TurnAngle::Deg180)),
                    270 => Ok(Self::Left(TurnAngle::Deg270)),
                    a => Err(Self::Err::BadAngle(a)),
                }
            }
            'F' => Ok(Self::Forwards(s.strip_prefix('F').unwrap().parse()?)),
            c => Err(Self::Err::BadPrefix(c)),
        }
    }
}

impl From<num::ParseIntError> for ShipMovementParseError {
    fn from(e: num::ParseIntError) -> Self {
        Self::NonNumeric(e)
    }
}

impl Default for Waypoint {
    fn default() -> Self {
        Self { dx: 10, dy: -1 }
    }
}
