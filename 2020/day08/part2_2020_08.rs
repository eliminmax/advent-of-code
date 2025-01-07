// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 8 Part 2

use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone, Copy)]
struct Instruction(Op, i32);

fn try_changing(change_index: usize, code: &[Instruction]) -> Option<i32> {
    let mut instructions: Vec<(Instruction, bool)> = code.iter().map(|i| (*i, true)).collect();
    let mut index: usize = 0;
    let mut accumulator: i32 = 0;
    match code[change_index] {
        Instruction(Op::Acc, _) => return None,
        Instruction(Op::Nop, i) => instructions[change_index].0 = Instruction(Op::Jmp, i),
        Instruction(Op::Jmp, i) => instructions[change_index].0 = Instruction(Op::Nop, i),
    }

    while let Some(instruction) = instructions.get_mut(index) {
        if !instruction.1 {
            return None;
        }
        instruction.1 = false;
        match instruction.0 {
            Instruction(Op::Nop, _) => index += 1,
            Instruction(Op::Acc, i) => {
                index += 1;
                accumulator += i;
            }
            Instruction(Op::Jmp, i) => {
                index = index
                    .checked_add_signed(i as isize)
                    .expect("instruction pointer out-of-bounds");
            }
        }
    }
    Some(accumulator)
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let mut instructions: Vec<Instruction> = Vec::new();
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
            instructions.push(Instruction(op, val));
        } else {
            panic!("Unable to parse {:?}", line);
        }
    }

    for i in 0usize..(instructions.len()) {
        if let Some(index) = try_changing(i, &instructions[..]) {
            println!("{index}");
            break;
        }
    }
}
