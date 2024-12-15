// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 7 Part 1

use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum State {
    Running(usize),
    Jump(usize),
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
            // add
            let dest = code[index + 3] as usize;
            code[dest] = select_by_mode!(1) + select_by_mode!(2);
            Ok(State::Running(4))
        }
        2 => {
            // multiply
            let dest = code[index + 3] as usize;
            code[dest] = select_by_mode!(1) * select_by_mode!(2);
            Ok(State::Running(4))
        }
        3 => {
            // input
            if let Some(input) = inputs.next() {
                let dest = code[index + 1] as usize;
                code[dest] = input;
                Ok(State::Running(2))
            } else {
                Err(ErrorState::MissingInput)
            }
        }
        4 => {
            // output
            outputs.push(select_by_mode!(1));
            Ok(State::Running(2))
        }
        5 => {
            // jump-if-true
            if select_by_mode!(1) == 0 {
                Ok(State::Running(3))
            } else {
                Ok(State::Jump(select_by_mode!(2) as usize))
            }
        }
        6 => {
            // jump-if-false
            if select_by_mode!(1) != 0 {
                Ok(State::Running(3))
            } else {
                Ok(State::Jump(select_by_mode!(2) as usize))
            }
        }
        7 => {
            // less than
            let dest = code[index + 3] as usize;
            code[dest] = if select_by_mode!(1) < select_by_mode!(2) {
                1
            } else {
                0
            };
            Ok(State::Running(4))
        }
        8 => {
            // equals
            let dest = code[index + 3] as usize;
            code[dest] = if select_by_mode!(1) == select_by_mode!(2) {
                1
            } else {
                0
            };
            Ok(State::Running(4))
        }
        99 => Ok(State::Halted),
        i => Err(ErrorState::UnrecognizedOpcode(i)),
    }
}

fn run_with_inputs<I: Iterator<Item = i32>>(
    code: &[i32],
    mut inputs: I,
) -> Result<Vec<i32>, ErrorState> {
    let mut code: Vec<i32> = code.to_owned();
    let mut outputs: Vec<i32> = Vec::new();
    let mut i = 0usize;
    while i < code.len() {
        match interpret(code.as_mut_slice(), i, &mut inputs, &mut outputs)? {
            State::Running(n) => i += n,
            State::Jump(n) => i = n,
            State::Halted => break,
        };
    }
    Ok(outputs)
}

fn heap_permutations(vals: &mut [i32]) -> Vec<Vec<i32>> {
    // implementation of Heap's Algorithm for generating permutations
    if vals.len() == 1 {
        return vec![vals.to_owned()];
    }
    let mut permutations: Vec<Vec<i32>> = Vec::new();
    let mut new_perms: Vec<Vec<i32>>;
    let last = vals.len() - 1;
    for i in 0..=last {
        new_perms = heap_permutations(&mut vals[..last]);
        new_perms.iter_mut().for_each(|v| v.push(vals[last]));
        permutations.append(&mut new_perms);
        if last % 2 == 0 {
            vals.swap(0, last);
        } else {
            vals.swap(i, last);
        }
    }
    permutations
}

fn main() {
    let code: Vec<i32> = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!")
        .trim()
        .split(",")
        .map(|s| s.parse().expect("Could not parse i32"))
        .collect();
    let mut max = 0i32;
    for permutation in heap_permutations(&mut [0, 1, 2, 3, 4]).into_iter() {
        let mut signal = 0i32;
        for amp in permutation.iter() {
            let output =
                run_with_inputs(&code, vec![*amp, signal].into_iter()).unwrap_or_else(|e| match e {
                    ErrorState::UnrecognizedOpcode(i) => panic!("Unrecognized opcode: {}", i),
                    ErrorState::MissingInput => panic!("Missing input!"),
                });
            if output.len() != 1 {
                panic!("Expected output of length 1, got {}", output.len());
            }
            signal = output[0];
        }
        if signal > max {
            max = signal;
        }
    }
    println!("{max}");
}
