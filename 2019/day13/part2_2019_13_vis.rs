// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Visualizer for solution to AoC 2019 Day 13 Part 2

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

impl std::fmt::Display for Tile {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                Tile::Empty => " ",
                Tile::HorizPaddle => "\x1b[38;5;226m─\x1b[39m", // bright yellow line
                Tile::Block => "\x1b[38;5;68m❒\x1b[39m",        // cornflower blue box
                Tile::Ball => "\x1b[38;5;196m●\x1b[39m",        // red circle
                Tile::Wall => "\x1b[38;5;28m█\x1b[39m",         // green block
            }
        )
    }
}

fn render(x: i64, y: i64, tile: Tile) {
    let x = u8::try_from(x).unwrap() + 1;
    let y = u8::try_from(y).unwrap() + 2;
    print!("\x1b[{y};{x}H{tile}");
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    use std::io::{stdout, Write};
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut interpreter =
        intcode::Interpreter::new(input.trim().split(",").map(|s| s.parse().unwrap()));
    interpreter.mem_override(0, 2);

    let mut screen = Screen::new();
    let mut ball_x: i64 = 0;
    let mut paddle_x: i64 = 0;

    let mut inputs = Vec::new();
    print!("\x1b[?25l\x1b[H\x1b[J\x1b[3J");
    stdout().lock().flush().unwrap();
    loop {
        let (draw_instructions, outcome) =
            interpreter.run_through_inputs(inputs.drain(..)).unwrap();
        for trio in draw_instructions.chunks(3) {
            match (trio[0], trio[1]) {
                (-1, 0) => print!("\x1b[H\x1b[2K\x1b[1;7mScore: {}\x1b[22;27m", trio[2]),
                (x, y) => {
                    let tile = Tile::try_from(trio[2]).unwrap();
                    if tile == Tile::Ball {
                        ball_x = x;
                    } else if tile == Tile::HorizPaddle {
                        paddle_x = x;
                    }
                    screen.insert((x, y), tile);
                    render(x, y, tile);
                }
            }
        }
        stdout().lock().flush().unwrap();
        inputs.push(ball_x.cmp(&paddle_x) as i64);
        std::thread::sleep(std::time::Duration::from_millis(15));
        if outcome == intcode::State::Halted {
            break;
        }
    }
    let y_max = screen.keys().map(|&(_, y)| y + 2).max().unwrap_or(1);
    println!("\x1b[{y_max}H\x1b[?25h");
}
