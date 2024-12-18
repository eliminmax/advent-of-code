// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 8 Part 1

use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug)]
struct Instruction(Op, i32);

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let mut instructions: Vec<(Instruction, bool)> = Vec::new();
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if let [op, val] = words[..] {
            let op = match op {
                "nop" => Op::Nop,
                "acc" => Op::Acc,
                "jmp" => Op::Jmp,
                i => panic!("Invalid instruction code: {:?}", i),
            };
            let val = val.parse::<i32>().expect("Failed to parse number as isize");
            instructions.push((Instruction(op, val), true));
        } else {
            panic!("Unable to parse {:?}", line);
        }
    }

    let mut accumulator: i32 = 0;
    let mut index: usize = 0;
    while instructions[index].1 {
        instructions[index].1 = false;
        match instructions[index].0 {
            Instruction(Op::Nop, _) => index += 1,
            Instruction(Op::Acc, i) => {
                accumulator += i;
                index += 1;
            }
            Instruction(Op::Jmp, i) => {
                index = index
                    .checked_add_signed(i as isize)
                    .expect("instruction pointer out-of-bounds");
            }
        }
    }
    println!("{accumulator}");
}
