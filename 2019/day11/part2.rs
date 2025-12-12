// SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 11 Part 2

// This is sounding an awful lot like Langton's Ant to me

use std::collections::BTreeMap;

// Moved the intcode interpreter code into its own module that can be copied over for future days
pub mod intcode;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
enum PanelColor {
    #[default]
    Black,
    White,
}

impl PanelColor {
    fn report(self) -> i64 {
        i64::from(self == Self::White)
    }

    fn paint(&mut self, color: i64) {
        *self = if color == 1 {
            Self::White
        } else {
            assert_eq!(color, 0, "invalid paint color");
            Self::Black
        };
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Location {
    y: i32,
    x: i32,
}

impl Direction {
    fn rotate_left(&mut self) {
        *self = match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    fn rotate_right(&mut self) {
        *self = match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }
}

impl std::ops::AddAssign<Direction> for Location {
    fn add_assign(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let mut interpreter = intcode::Interpreter::new(
        read_to_string(args().nth(1).as_deref().unwrap_or("input"))
            .expect("Failed to read file!")
            .trim()
            .split(",")
            .map(|s| s.parse().expect("Could not parse i64")),
    );

    if cfg!(debug_assertions) {
        interpreter.enable_logging();
    }

    let mut panels: BTreeMap<Location, PanelColor> =
        BTreeMap::from([(Location::default(), PanelColor::White)]);
    let mut location = Location::default();
    let mut direction = Direction::Up;

    while let (outputs, intcode::State::Awaiting) = interpreter
        .run_through_inputs(vec![panels.entry(location).or_default().report()])
        .unwrap()
    {
        debug_assert_eq!(outputs.len(), 2);
        panels.entry(location).or_default().paint(outputs[0]);
        match outputs[1] {
            0 => direction.rotate_right(),
            1 => direction.rotate_left(),
            i => panic!("invalid direction code: {i}"),
        }
        location += direction;
    }

    let [mut min_x, mut min_y, mut max_x, mut max_y]: [Option<i32>; 4] = [None; 4];

    for &Location { x, y } in panels.keys() {
        min_x = min_x.map(|m: i32| m.min(x)).or(Some(x));
        min_y = min_y.map(|m: i32| m.min(y)).or(Some(y));
        max_x = max_x.map(|m: i32| m.max(x)).or(Some(x));
        max_y = max_y.map(|m: i32| m.max(y)).or(Some(y));
    }

    for y in min_y.unwrap()..=max_y.unwrap() {
        for x in min_x.unwrap()..=max_x.unwrap() {
            print!(
                "\x1b[48;5;{}m ",
                match panels.get(&Location { x, y }).copied().unwrap_or_default() {
                    PanelColor::Black => 16,
                    PanelColor::White => 231,
                }
            );
        }
        println!("\x1b[m");
    }
}
