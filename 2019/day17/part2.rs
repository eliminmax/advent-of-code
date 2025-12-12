// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 17 Part 2

//! This solution makes several assumptions, which hold for my input but are in no way guaranteed by
//! the problem description.
//!
//! * it assumes that there is an optimal path which covers all locations, starting with the
//!   robot's location, and with no branching paths.
//!
//! * it assumes only time a grid space needs to be revisited is when it's at an intersection.
//!
//! it also that requires the user to find the 3 functions to split the code into themselves, and
//! write them to a file in the proper format, and re-run the executable with an environment
//! variable pointing to that file.
//!
//! Hacky solution, and not fully automatic, but it works.

mod intcode;

use intcode::Interpreter;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Location {
    y: usize,
    x: usize,
}

impl Location {
    fn next_in(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                y: self.y - 1,
                ..self
            },
            Direction::Right => Self {
                x: self.x + 1,
                ..self
            },
            Direction::Down => Self {
                y: self.y + 1,
                ..self
            },
            Direction::Left => Self {
                x: self.x - 1,
                ..self
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    const fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    const fn rotate_left(self) -> Self {
        self.rotate_right().rotate_right().rotate_right()
    }
}

fn parse_start(mut interpreter: Interpreter) -> (BTreeSet<Location>, Location, Direction) {
    let (output, intcode::State::Halted) =
        interpreter.run_through_inputs(std::iter::empty()).unwrap()
    else {
        panic!("stuck waiting for input");
    };
    let mut map = BTreeSet::new();
    let mut bot: Option<(Location, Direction)> = None;

    // start at 1 to simplify the logic to build the directions
    let mut x = 1;
    let mut y = 1;
    for c in output {
        match u8::try_from(c).unwrap() {
            b'.' => x += 1,
            b'\n' => {
                x = 1;
                y += 1
            }

            b'#' => {
                x += 1;
                map.insert(Location { x, y });
            }
            b'v' => {
                assert!(bot.is_none());
                x += 1;
                map.insert(Location { x, y });
                bot = Some((Location { x, y }, Direction::Down));
            }
            b'<' => {
                assert!(bot.is_none());
                x += 1;
                map.insert(Location { x, y });
                bot = Some((Location { x, y }, Direction::Left));
            }
            b'^' => {
                assert!(bot.is_none());
                x += 1;
                map.insert(Location { x, y });
                bot = Some((Location { x, y }, Direction::Up));
            }
            b'>' => {
                assert!(bot.is_none());
                x += 1;
                map.insert(Location { x, y });
                bot = Some((Location { x, y }, Direction::Right));
            }
            b'X' => panic!("bot not on scaffolding"),
            b => panic!("Unknown output {}", b.escape_ascii()),
        }
    }

    let (bot_loc, bot_dir) = bot.expect("bot location");
    (map, bot_loc, bot_dir)
}

fn build_instructions(
    mut grid: BTreeSet<Location>,
    mut bot_loc: Location,
    mut bot_dir: Direction,
) -> String {
    use std::fmt::Write;
    let mut instructions = String::new();

    let mut move_len: u32 = 0;
    while !grid.is_empty() {
        // remove the current location unless it's an intersectio that'll need to be crossed again
        if !grid.contains(&bot_loc.next_in(bot_dir.rotate_right()))
            && !grid.contains(&bot_loc.next_in(bot_dir.rotate_left()))
        {
            grid.remove(&bot_loc);
        }
        if grid.contains(&bot_loc.next_in(bot_dir)) {
            move_len += 1;
            bot_loc = bot_loc.next_in(bot_dir);
            continue;
        }
        if move_len > 0 {
            write!(&mut instructions, "{move_len},").expect("write!(&mut String) always succeeds");
            move_len = 0;
        }

        if grid.contains(&bot_loc.next_in(bot_dir.rotate_right())) {
            instructions.push_str("R,");
            bot_dir = bot_dir.rotate_right()
        } else if grid.contains(&bot_loc.next_in(bot_dir.rotate_left())) {
            instructions.push_str("L,");
            bot_dir = bot_dir.rotate_left()
        } else {
            assert!(grid.is_empty(), "solution assumes linear path");
        }
    }

    assert!(instructions.pop().is_none_or(|i| i == ','));
    instructions
}

fn main() {
    use std::env::{self, args};
    use std::fs::{OpenOptions, read_to_string};

    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let interpreter = Interpreter::new(input.trim().split(",").map(|i| i.parse().unwrap()));
    let (grid, loc, dir) = parse_start(interpreter.clone());
    let instructions = build_instructions(grid, loc, dir);

    if let Some(code_file) = env::var_os("AOC_ROBOT_MOVES") {
        let mut split_moves = read_to_string(code_file).expect("Failed to read moves!");
        assert!(split_moves.ends_with('\n'), "Missing trailing newline");
        let lines: Vec<&str> = split_moves.lines().collect();
        // validation of the structure
        assert_eq!(
            lines.len(),
            4,
            "Expected 1 main routine and 3 function definitions"
        );
        assert!(
            lines[0].len() <= 20,
            "Main routine must be at most 20 characters"
        );
        for (index, func) in "_ABC".chars().enumerate().skip(1) {
            assert!(
                lines[index].len() <= 20,
                "Function {func} must be at most 20 characters"
            );
        }
        let reconstructed = lines[0]
            .replace("A", lines[1])
            .replace("B", lines[2])
            .replace("C", lines[3]);

        assert_eq!(
            instructions, reconstructed,
            "Provided sequence is not equivalent to required moves"
        );
        assert!(
            lines[1..].iter().all(|f| !f.contains(['A', 'B', 'C'])),
            "Functions can't call other functions"
        );
        let mut interpreter = interpreter;
        interpreter.mem_override(0, 2);
        drop(lines);
        split_moves.push_str("n\n"); // disable video feed
        let (output, intcode::State::Halted) = interpreter
            .run_through_inputs(split_moves.chars().map(|c| i64::from(u32::from(c))))
            .unwrap()
        else {
            panic!("out of input");
        };
        println!("{}", output.last().unwrap());
    } else {
        use std::io::Write;

        let mut outfile = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("raw_moves.txt")
            .unwrap();

        writeln!(&mut outfile, "{instructions}").expect("Failed to write file");
        println!(concat!(
            "Wrote sequence to \"raw_moves.txt\". ",
            "Split the sequence into the 3 functions and save to a file.",
            " Once done, re-run with the AOC_ROBOT_MOVES",
            " environment variable set to that file"
        ));
    }
}
