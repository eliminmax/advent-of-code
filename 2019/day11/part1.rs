// SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 11 Part 1

// This is sounding an awful lot like Langton's Ant to me

use std::collections::HashMap;

// Moved the intcode interpreter code into its own module that can be copied over for future days
pub mod intcode;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PanelColor {
    Black { repainted: bool },
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
            Self::Black { repainted: true }
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Location {
    x: i32,
    y: i32,
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

    let mut panels: HashMap<Location, PanelColor> = HashMap::new();
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

    println!(
        "{}",
        panels
            .into_values()
            .filter(|v| *v != PanelColor::default())
            .count()
    );
}

impl Default for PanelColor {
    fn default() -> Self {
        Self::Black { repainted: false }
    }
}
