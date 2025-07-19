// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 12 Part 1
use std::num::{self, NonZeroU8};

#[derive(Default, Debug, PartialEq, Clone, Copy)]
#[repr(i8)]
enum Direction {
    #[allow(dead_code, reason = "variant constructed transmute")]
    North = 0,
    #[default]
    East = 1,
    #[allow(dead_code, reason = "variant constructed transmute")]
    South = 2,
    #[allow(dead_code, reason = "variant constructed transmute")]
    West = 3,
}

impl Direction {
    const fn turn_left(&mut self, angle: TurnAngle) {
        let new_angle = (*self as i8 - angle as i8).rem_euclid(4);
        // SAFETY: the rem_euclid ensures that `new_angle` is in the range 0..4, which is the range
        // of valid values for `Direction`
        *self = unsafe { std::mem::transmute::<i8, Self>(new_angle) };
    }

    const fn turn_right(&mut self, angle: TurnAngle) {
        let new_angle = (*self as i8 + angle as i8).rem_euclid(4);
        // SAFETY: the rem_euclid ensures that `new_angle` is in the range 0..4, which is the range
        // of valid values for `Direction`
        *self = unsafe { std::mem::transmute::<i8, Self>(new_angle) };
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
    facing: Direction,
    row: i32,
    col: i32,
}

impl Ship {

    const fn run_move(&mut self, ship_move: ShipMovement) {
        match ship_move {
            ShipMovement::North(i) => self.row -= i.get() as i32,
            ShipMovement::East(i) => self.col += i.get() as i32,
            ShipMovement::South(i) => self.row += i.get() as i32,
            ShipMovement::West(i) => self.col -= i.get() as i32,
            ShipMovement::Forwards(i) => match self.facing {
                Direction::North => self.run_move(ShipMovement::North(i)),
                Direction::East => self.run_move(ShipMovement::East(i)),
                Direction::South => self.run_move(ShipMovement::South(i)),
                Direction::West => self.run_move(ShipMovement::West(i)),
            },
            ShipMovement::Left(a) => self.facing.turn_left(a),
            ShipMovement::Right(a) => self.facing.turn_right(a),
        }
    }

    #[inline]
    const fn distance_from_start(&self) -> u32 {
        self.row.unsigned_abs() + self.col.unsigned_abs()
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

#[cfg(test)]
mod turn_tests {
    use super::{Direction, TurnAngle};
    #[test]
    fn turn_right_tests() {
        let mut dir = Direction::North;

        dir.turn_right(TurnAngle::Deg90);
        assert_eq!(dir, Direction::East);
        dir.turn_right(TurnAngle::Deg90);
        assert_eq!(dir, Direction::South);
        dir.turn_right(TurnAngle::Deg90);
        assert_eq!(dir, Direction::West);
        dir.turn_right(TurnAngle::Deg90);
        assert_eq!(dir, Direction::North);

        dir = Direction::East;
        dir.turn_right(TurnAngle::Deg180);
        assert_eq!(dir, Direction::West);
        dir.turn_right(TurnAngle::Deg180);
        assert_eq!(dir, Direction::East);
        dir = Direction::North;
        dir.turn_right(TurnAngle::Deg180);
        assert_eq!(dir, Direction::South);
        dir.turn_right(TurnAngle::Deg180);
        assert_eq!(dir, Direction::North);

        dir.turn_right(TurnAngle::Deg270);
        assert_eq!(dir, Direction::West);
        dir.turn_right(TurnAngle::Deg270);
        assert_eq!(dir, Direction::South);
        dir.turn_right(TurnAngle::Deg270);
        assert_eq!(dir, Direction::East);
        dir.turn_right(TurnAngle::Deg270);
        assert_eq!(dir, Direction::North);
    }

    #[test]
    fn turn_left_tests() {
        let mut dir = Direction::North;

        dir.turn_left(TurnAngle::Deg90);
        assert_eq!(dir, Direction::West);
        dir.turn_left(TurnAngle::Deg90);
        assert_eq!(dir, Direction::South);
        dir.turn_left(TurnAngle::Deg90);
        assert_eq!(dir, Direction::East);
        dir.turn_left(TurnAngle::Deg90);
        assert_eq!(dir, Direction::North);

        dir = Direction::West;
        dir.turn_left(TurnAngle::Deg180);
        assert_eq!(dir, Direction::East);
        dir.turn_left(TurnAngle::Deg180);
        assert_eq!(dir, Direction::West);
        dir = Direction::North;
        dir.turn_left(TurnAngle::Deg180);
        assert_eq!(dir, Direction::South);
        dir.turn_left(TurnAngle::Deg180);
        assert_eq!(dir, Direction::North);

        dir.turn_left(TurnAngle::Deg270);
        assert_eq!(dir, Direction::East);
        dir.turn_left(TurnAngle::Deg270);
        assert_eq!(dir, Direction::South);
        dir.turn_left(TurnAngle::Deg270);
        assert_eq!(dir, Direction::West);
        dir.turn_left(TurnAngle::Deg270);
        assert_eq!(dir, Direction::North);
    }
}
