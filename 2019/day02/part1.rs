// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 2 Part 1

use std::env::args;
use std::fs::read_to_string;

#[derive(PartialEq)]
enum State {
    Running,
    Halted,
}

// Consume and interpret first instruction in code
// Per advent of code description, if opcode is 1, consume index
fn interpret(code: &mut [i32], index: usize) -> Result<State, i32> {
    match code[index..=index + 3] {
        [1, a, b, dest] => {
            code[dest as usize] = code[a as usize] + code[b as usize];
            Ok(State::Running)
        }
        [2, a, b, dest] => {
            code[dest as usize] = code[a as usize] * code[b as usize];
            Ok(State::Running)
        }
        [99, _a, _b, _dest] => Ok(State::Halted),
        [err_op, _a, _b, _dest] => Err(err_op),
        [] => Ok(State::Halted),
        _ => unreachable!(),
    }
}

fn main() {
    let mut code: Vec<i32> = read_to_string(args().nth(1).expect("No filename provided"))
        .expect("Failed to read file!")
        .trim()
        .split(",")
        .map(|s| s.parse().expect("Could not parse i32"))
        .collect();
    // how to restore 1202 program alarm, according to AoC instructions
    code.as_mut_slice()[1] = 12;
    code.as_mut_slice()[2] = 2;
    for i in (0..code.len()).step_by(4) {
        match interpret(code.as_mut_slice(), i) {
            Err(i) => panic!("Unrecognized opcode: {}", i),
            Ok(State::Running) => continue,
            Ok(State::Halted) => break,
        };
    }
    println!("{}", code.first().expect("Could not get code"));
}
