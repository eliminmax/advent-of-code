// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 13 Part 1

mod intcode;

use std::collections::HashMap;

#[derive(Clone, Copy, Default, PartialEq, Debug)]
enum Tile {
    #[default]
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizPaddle = 3,
    Ball = 4,
}

impl TryFrom<i64> for Tile {
    type Error = i64;
    fn try_from(tile_id: i64) -> Result<Self, i64> {
        match tile_id {
            0 => Ok(Tile::Empty),
            1 => Ok(Tile::Wall),
            2 => Ok(Tile::Block),
            3 => Ok(Tile::HorizPaddle),
            4 => Ok(Tile::Ball),
            _ => Err(tile_id),
        }
    }
}

type Screen = HashMap<(i64, i64), Tile>;

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut interpreter =
        intcode::Interpreter::new(input.trim().split(",").map(|s| s.parse().unwrap()));

    let mut screen = Screen::new();

    let (draw_instructions, intcode::State::Halted) =
        interpreter.run_through_inputs(Vec::new()).unwrap()
    else {
        panic!("stuck awaiting input");
    };

    for trio in draw_instructions.chunks(3) {
        screen.insert((trio[0], trio[1]), Tile::try_from(trio[2]).unwrap());
    }

    println!(
        "{}",
        screen.into_values().filter(|t| *t == Tile::Block).count()
    );
}
