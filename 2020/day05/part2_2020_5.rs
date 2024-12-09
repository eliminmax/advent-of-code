// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 5 Part 2

use std::env::args;
use std::fs::read_to_string;

fn seat_id(row: u8, col: u8) -> u32 {
    (row as u32 * 8) + col as u32
}

fn handle_pass(instructions: &[u8]) -> Result<u32, u8> {
    let rows: Vec<u8> = (0u8..128).collect();
    let cols: Vec<u8> = (0u8..8).collect();
    let mut possible_rows = rows.as_slice();
    let mut possible_cols = cols.as_slice();
    let mut cutoff = 128usize;
    for i in instructions[..7].iter() {
        cutoff /= 2;
        match i {
            b'F' => possible_rows = &possible_rows[..cutoff],
            b'B' => possible_rows = &possible_rows[cutoff..],
            i => return Err(*i),
        };
    }
    cutoff = 8;
    for i in instructions[7..].iter() {
        cutoff /= 2;
        match i {
            b'L' => possible_cols = &possible_cols[..cutoff],
            b'R' => possible_cols = &possible_cols[cutoff..],
            i => return Err(*i),
        };
    }
    if possible_rows.len() != 1 {
        panic!("possible_rows not yet trimmed down: {:?}", possible_rows);
    }
    if possible_cols.len() != 1 {
        panic!("possible_cols not yet trimmed down: {:?}", possible_cols);
    }

    Ok(seat_id(possible_rows[0], possible_cols[0]))
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let pass_seat_ids: Vec<u32> = input
        .lines()
        .map(|line| {
            handle_pass(line.as_bytes())
                .unwrap_or_else(|i| panic!("Unrecoginzed instruction {}", i as char))
        })
        .collect();
    // skip 0 to avoid an underflow, and because it's going to be at least 1.
    for p in 1u32..1024 {
        /* .map(|n| (((n & 0b1111111000)>>3) * 8) + (n & 0b111)) */
        if pass_seat_ids.contains(&(p - 1))
            && pass_seat_ids.contains(&(p + 1))
            && !(pass_seat_ids.contains(&p))
        {
            println!("{p}")
        }
    }
}
