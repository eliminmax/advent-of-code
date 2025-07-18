// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 13 Part 2

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

fn select_input(screen: &Screen) -> i64 {
    let mut ball_x: Option<i64> = None;
    let mut paddle_x: Option<i64> = None;
    for (&(x, _), &tile) in screen.iter() {
        match tile {
            Tile::Ball => {
                assert!(ball_x.is_none());
                ball_x = Some(x);
            }
            Tile::HorizPaddle => {
                assert!(paddle_x.is_none());
                paddle_x = Some(x);
            }
            _ => (),
        }
    }
    ball_x.cmp(&paddle_x) as i64
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut interpreter =
        intcode::Interpreter::new(input.trim().split(",").map(|s| s.parse().unwrap()));
    interpreter.mem_override(0, 2);

    let mut screen = Screen::new();
    let mut score = 0;

    let mut inputs = Vec::new();
    loop {
        let (draw_instructions, _) = interpreter.run_through_inputs(inputs.drain(..)).unwrap();

        for trio in draw_instructions.chunks(3) {
            match (trio[0], trio[1]) {
                (-1, 0) => score = trio[2],
                pos => {
                    let _ = screen.insert(pos, Tile::try_from(trio[2]).unwrap());
                }
            }
        }
        // render(&screen, score);
        inputs.push(select_input(&screen));

        if screen.values().all(|t| *t != Tile::Block) {
            break;
        }
    }

    println!("{score}");
}
