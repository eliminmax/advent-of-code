// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 19 Part 1

mod intcode;
use intcode::Interpreter;

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut interpreter = Interpreter::new(input.trim().split(",").map(|i| i.parse().unwrap()));
    let (output, intcode::State::Awaiting) =
        interpreter.run_through_inputs(std::iter::empty()).unwrap()
    else {
        panic!("Interpreter did not wait for input");
    };

    assert!(output.is_empty());

    let mut count = 0;
    for x in 0..50 {
        for y in 0..50 {
            let mut bot_interpreter = interpreter.clone();
            let (output, intcode::State::Halted) =
                bot_interpreter.run_through_inputs([x, y]).unwrap()
            else {
                panic!("Interpreter awaiting more input for ({x}, {y})")
            };
            assert_eq!(output.len(), 1);
            count += output[0];
        }
    }
    println!("{count}");
}
