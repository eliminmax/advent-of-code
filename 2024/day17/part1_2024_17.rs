// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 17 Part 1

use std::env::args;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::str::FromStr;
use std::ops::Deref;

#[derive(Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

#[derive(Debug)]
enum OpParseError {
    OutOfRange,
    Unparsable,
}

impl From<ParseIntError> for OpParseError {
    fn from(_err: ParseIntError) -> OpParseError {
        OpParseError::Unparsable
    }
}

#[derive(Debug)]
struct Op(u8);

impl Deref for Op {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}

impl FromStr for Op {
    type Err = OpParseError;
    fn from_str(s: &str) -> Result<Op, OpParseError> {
        let val = u8::from_str_radix(s, 8)?;
        if val < 0o10 {
            Ok(Op(val))
        } else {
            Err(OpParseError::OutOfRange)
        }
    }
}

#[derive(Debug, PartialEq)]
enum State {
    Normal,
    Jumping(u64),
}

fn interpret(code: &[Op], index: usize, output: &mut String, regs: &mut Registers) -> State {

    let opcode = *code[index];
    let val = *code[index + 1] as u64;

    macro_rules! combo_op {
        () => {{
            match val {
                i if i < 4 => i,
                4 => regs.a,
                5 => regs.b,
                6 => regs.c,
                _ => unreachable!(),
            }
        }};
    }


    macro_rules! dv {
        ($reg: ident) => {{
            regs.$reg = (regs.a as f64 / (2f64).powf(combo_op!() as f64)) as u64;
            State::Normal
        }};
    }

    match opcode {
        0 => dv!(a), // adv
        1 => { // bxl
            regs.b ^= val;
            State::Normal
        }
        2 => { // bst
            regs.b = combo_op!() % 8;
            State::Normal
        }
        3 => { // jnz
            if regs.a == 0 {
                State::Normal
            } else {
                State::Jumping(val)
            }
        }
        4 => { // bxc
            regs.b ^= regs.c;
            State::Normal
        }
        5 => { // out
            if !output.is_empty() {
                output.push(',');
            }
            output.push_str(&(combo_op!() % 8).to_string());
            State::Normal
        }
        6 => dv!(b), // bdv
        7 => dv!(c), // cdv
        _ => unreachable!(),
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut input_lines = input.lines();

    let reg_vals: [u64; 3] = core::array::from_fn(|_| input_lines
            .next()
            .expect("Missing register line")
            .split_once(": ")
            .expect("Failed to find \": \" delimiter within a register line")
            .1
            .trim()
            .parse()
            .expect("Failed to parse register value")
    );
    let mut regs = Registers {
        a: reg_vals[0],
        b: reg_vals[1],
        c: reg_vals[2],
    };

    if input_lines.next() != Some("") {
        panic!("Delimiter line is missing from input");
    }

    let code: Vec<Op> = input_lines
        .next()
        .expect("Program line is missing from input")
        .split_once(": ")
        .expect("Failed to find \": \" delimiter within the program line")
        .1
        .split(',')
        .map(|s| Op::from_str(s).expect("Failed to parse Op"))
        .collect();

    if input_lines.next().is_some() {
        panic!("Leftover lines in input after parsing");
    }
    let mut i: usize = 0;

    let mut output = String::new();
    while i < code.len() - 1 {
        match interpret(code.as_slice(), i, &mut output, &mut regs) {
            State::Normal => i += 2,
            State::Jumping(addr) => i = addr as usize,
        }
    }
    println!("{output}");
}
