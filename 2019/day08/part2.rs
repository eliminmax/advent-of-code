// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 8 Part 2

use std::convert::{TryFrom, TryInto};
use std::env::args;
use std::fs::read_to_string;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
enum Pixel {
    Black = 0,
    White = 1,
    Transparent = 2,
}

impl Pixel {
    fn as_display_char(&self) -> char {
        match self {
            // chosen with the assumption that it'll be light text on a dark terminal background.
            Pixel::Black => ' ',
            Pixel::White => '█',
            Pixel::Transparent => '▒',
        }
    }
}

#[derive(Debug)]
struct PixelDecodeError;
impl TryFrom<char> for Pixel {
    type Error = PixelDecodeError;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(Pixel::Black),
            '1' => Ok(Pixel::White),
            '2' => Ok(Pixel::Transparent),
            _ => Err(PixelDecodeError),
        }
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let pixels: Vec<Pixel> = input
        .trim()
        .chars()
        .map(|c| Pixel::try_from(c).expect("Invalid pixel in input"))
        .collect();

    let mut layers: Vec<[Pixel; LAYER_SIZE]> = pixels
        .chunks_exact(WIDTH * HEIGHT)
        .map(|c| c.try_into().expect("Could not create image layer"))
        .collect();
    layers.reverse();

    let mut base_layer = [Pixel::Transparent; LAYER_SIZE];

    for layer in layers {
        for (i, pix) in layer.iter().enumerate() {
            if *pix != Pixel::Transparent {
                base_layer[i] = *pix;
            }
        }
    }

    let rows: Vec<String> = base_layer
        .as_slice()
        .chunks_exact(WIDTH)
        .map(|row| {
            row.iter()
                .map(|pix| pix.as_display_char())
                .collect::<String>()
        })
        .collect();

    println!("{}", rows.join("\n"));
}
