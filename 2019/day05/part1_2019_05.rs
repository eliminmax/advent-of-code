// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 5 Part 2

use std::env::args;
use std::fs::read_to_string;

#[derive(PartialEq)]
enum State {
    Running(usize),
    Halted,
}

#[derive(Debug, PartialEq)]
enum ErrorState {
    UnrecognizedOpcode(i32),
    MissingInput,
}

// interpret instruction at index
fn interpret<I: Iterator<Item = i32>>(
    code: &mut [i32],
    index: usize,
    inputs: &mut I,
    outputs: &mut Vec<i32>,
) -> Result<State, ErrorState> {
    let opcode = code[index] % 100;
    // messy way to reinterpret the higher digits as bits
    let mode_code: u8 = (code[index] / 100)
        .to_string()
        .into_bytes()
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, b)| if b == b'1' { 1 << i } else { 0 })
        .sum();

    macro_rules! select_by_mode {
        ($n: literal) => {
            if mode_code & (1 << ($n - 1)) == 0 {
                code[code[index + $n] as usize]
            } else {
                code[index + $n]
            }
        };
    }
    match opcode {
        1 => {
            let dest = code[index + 3] as usize;
            code[dest] = select_by_mode!(1) + select_by_mode!(2);
            Ok(State::Running(4))
        }
        2 => {
            let dest = code[index + 3] as usize;
            code[dest] = select_by_mode!(1) * select_by_mode!(2);
            Ok(State::Running(4))
        }
        3 => {
            if let Some(input) = inputs.next() {
                let dest = code[index + 1] as usize;
                code[dest] = input;
                Ok(State::Running(2))
            } else {
                Err(ErrorState::MissingInput)
            }
        }
        4 => {
            outputs.push(select_by_mode!(1));
            Ok(State::Running(2))
        }
        99 => Ok(State::Halted),
        i => Err(ErrorState::UnrecognizedOpcode(i)),
    }
}

fn main() {
    let mut code: Vec<i32> = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!")
        .trim()
        .split(",")
        .map(|s| s.parse().expect("Could not parse i32"))
        .collect();
    let mut inputs = vec![1].into_iter();
    let mut outputs: Vec<i32> = Vec::new();
    let mut i = 0usize;
    while i < code.len() {
        match interpret(code.as_mut_slice(), i, &mut inputs, &mut outputs) {
            Err(ErrorState::UnrecognizedOpcode(i)) => panic!("Unrecognized opcode: {}", i),
            Err(ErrorState::MissingInput) => panic!("Missing input!"),
            Ok(State::Running(n)) => i += n,
            Ok(State::Halted) => {
                let diagnostic: i32 = outputs.pop().expect("No outputs found!");
                println!("{diagnostic}");
                break;
            }
        };
    }
}
