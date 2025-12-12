// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 09 Part 1

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u16)]
/// Underlying value, when reinterpreted as an `[i8; 2]`, can be used as an
enum Direction {
    Right = u16::from_ne_bytes([1, 0]),
    Up = u16::from_ne_bytes([0, N1_U8]),
    Left = u16::from_ne_bytes([N1_U8, 0]),
    Down = u16::from_ne_bytes([0, 1]),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Move(Direction, u8);

type Point = (i16, i16);

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(align(8))]
struct Rope {
    head: Point,
    tail: Point,
}

impl Rope {
    fn process_move(&mut self, m: Move) -> impl IntoIterator<Item = Point> {
        let Move(dir, dist) = m;
        let [ux, uy] = (dir as u16).to_ne_bytes();
        let head_x_off = i16::from(ux.cast_signed());
        let head_y_off = i16::from(uy.cast_signed());

        let mut positions = Vec::with_capacity(usize::from(dist) + 1);
        positions.push(self.tail);
        for _ in 0..dist {
            self.head.0 += head_x_off;
            self.head.1 += head_y_off;
            if distance(self.head, self.tail) > 1 {
                self.tail.0 -= self.tail.0.cmp(&self.head.0) as i16;
                self.tail.1 -= self.tail.1.cmp(&self.head.1) as i16;
                positions.push(self.tail);
            }
        }

        positions
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut rope = Rope::default();

    let mut visited = HashSet::new();
    for line in input.lines() {
        visited.extend(rope.process_move(line.parse()?));
    }

    println!("{}", visited.len());
    Ok(())
}

fn distance(a: Point, b: Point) -> u16 {
    let h_dist = a.0.abs_diff(b.0);
    let v_dist = a.1.abs_diff(b.1);
    h_dist.max(v_dist)
}

mod std_traits {
    use super::{Direction, Move};
    use std::error::Error;
    use std::fmt::{self, Display};
    use std::str::FromStr;

    #[derive(Debug)]
    pub enum DirectionParseError {
        BadDirection(char),
        BadFmt(String),
        BadNum(String, std::num::ParseIntError),
        EmptyString,
    }

    impl Display for DirectionParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::BadDirection(d) => write!(
                    f,
                    "direction {d:?} is invalid - should be ['U' | 'L' | 'D' | 'R']"
                ),
                Self::BadFmt(s) => write!(f, "unrecognized format: {s:?}"),
                Self::BadNum(s, e) => write!(f, "failed to parse {s:?} as i16: {e}"),
                Self::EmptyString => write!(f, "can't parse direction from empty str"),
            }
        }
    }

    impl Error for DirectionParseError {}

    impl FromStr for Move {
        type Err = DirectionParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut chars = s.chars();
            let dir = match chars.next() {
                None => return Err(DirectionParseError::EmptyString),
                Some('R') => Direction::Right,
                Some('U') => Direction::Up,
                Some('L') => Direction::Left,
                Some('D') => Direction::Down,
                Some(d) => return Err(DirectionParseError::BadDirection(d)),
            };
            if chars.next() != Some(' ') {
                return Err(DirectionParseError::BadFmt(s.into()));
            }
            let remaining: String = chars.collect();
            let dist = remaining
                .parse()
                .map_err(|e| DirectionParseError::BadNum(remaining, e))?;

            Ok(Self(dir, dist))
        }
    }
}

/// const that's more concise inline than `i8::cast_unsigned(-1)`
const N1_U8: u8 = i8::cast_unsigned(-1);
