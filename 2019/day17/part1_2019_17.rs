// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 17 Part 1

mod intcode;
use intcode::Interpreter;
use std::num::NonZero;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn neighbors(self) -> impl Iterator<Item = Self> {
        vec![
            self.x.checked_sub(1).map(|x| Location { x, ..self }),
            self.y.checked_sub(1).map(|y| Location { y, ..self }),
            Some(Location {
                x: self.x + 1,
                ..self
            }),
            Some(Location {
                y: self.y + 1,
                ..self
            }),
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Grid<T: std::fmt::Debug + Clone + PartialEq> {
    data: Vec<T>,
    width: NonZero<usize>,
}

impl<T: std::fmt::Debug + Clone + PartialEq> Grid<T> {
    fn get(&self, Location { x, y }: Location) -> Option<&T> {
        self.data.get(y * self.width.get() + x)
    }

    fn width(&self) -> usize {
        self.width.get()
    }

    fn height(&self) -> usize {
        self.data.len() / self.width.get()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum VacuumBot {
    Active { facing: Direction },
    Inactive,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Space {
    Scaffolding(Option<VacuumBot>),
    Empty,
}

fn parse_grid(mut interpreter: Interpreter) -> Result<Grid<Space>, String> {
    let (mut output, intcode::State::Halted) = interpreter
        .run_through_inputs(std::iter::empty())
        .map_err(|e| format!("{e:?}"))?
    else {
        return Err(String::from("stuck waiting for input"));
    };

    // for some reason, my input has extra trailing newlines that mess my logic up
    while output.last().copied() == Some(b'\n' as i64) {
        output.pop();
    }

    let width = NonZero::new(output.iter().position(|i| *i == 10).unwrap_or(output.len()))
        .ok_or("width can't be 0")?;
    assert!(
        output
            .split(|i| *i == 10)
            .skip(1)
            .all(|row| row.len() == width.get()),
        "assuming even grid"
    );
    output.retain(|i| *i != 10);

    Ok(Grid {
        data: output
            .into_iter()
            .map(|c| match u8::try_from(c) {
                Ok(b'X') => Ok(Space::Scaffolding(Some(VacuumBot::Inactive))),
                Ok(b'>') => Ok(Space::Scaffolding(Some(VacuumBot::Active {
                    facing: Direction::Right,
                }))),
                Ok(b'v') => Ok(Space::Scaffolding(Some(VacuumBot::Active {
                    facing: Direction::Down,
                }))),
                Ok(b'<') => Ok(Space::Scaffolding(Some(VacuumBot::Active {
                    facing: Direction::Left,
                }))),
                Ok(b'^') => Ok(Space::Scaffolding(Some(VacuumBot::Active {
                    facing: Direction::Up,
                }))),
                Ok(b'#') => Ok(Space::Scaffolding(None)),
                Ok(b'.') => Ok(Space::Empty),
                Ok(b'\n') => unreachable!("newlines filtered out"),
                Ok(b) => Err(format!("invalid byte value 0x{b:02x}")),
                Err(e) => Err(format!("{e:?}")),
            })
            .collect::<Result<_, String>>()?,
        width,
    })
}

fn check_alignment(grid: Grid<Space>) -> usize {
    let mut total = 0;
    for y in 1..grid.height() - 1 {
        for x in 1..grid.width() - 1 {
            let loc = Location { x, y };
            if matches!(grid.get(loc), Some(Space::Scaffolding(_)))
                && loc
                    .neighbors()
                    .all(|n| matches!(grid.get(n), Some(Space::Scaffolding(_))))
            {
                total += x * y;
            }
        }
    }
    total
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    println!(
        "{}",
        check_alignment(
            parse_grid(Interpreter::new(
                input.trim().split(",").map(|i| i.parse().unwrap()),
            ))
            .unwrap()
        )
    );
}
