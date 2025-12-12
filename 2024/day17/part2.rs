// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 17 Part 2

use std::env::args;
use std::fs::read_to_string;
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug, Clone)]
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

#[derive(Debug, PartialEq, Copy, Clone)]
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

fn interpret(code: &[Op], index: usize, output: &mut Vec<Op>, regs: &mut Registers) -> State {
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
        1 => {
            // bxl
            regs.b ^= val;
            State::Normal
        }
        2 => {
            // bst
            regs.b = combo_op!() % 8;
            State::Normal
        }
        3 => {
            // jnz
            if regs.a == 0 {
                State::Normal
            } else {
                State::Jumping(val)
            }
        }
        4 => {
            // bxc
            regs.b ^= regs.c;
            State::Normal
        }
        5 => {
            // out
            output.push(Op((combo_op!() % 8) as u8));
            State::Normal
        }
        6 => dv!(b), // bdv
        7 => dv!(c), // cdv
        _ => unreachable!(),
    }
}

fn with_regs(code: &[Op], regs: &mut Registers) -> Vec<Op> {
    let mut i: usize = 0;
    let mut output: Vec<Op> = Vec::new();
    let mut regs = regs.clone();
    while i < code.len() {
        match interpret(code, i, &mut output, &mut regs) {
            State::Normal => i += 2,
            State::Jumping(addr) => i = addr as usize,
        }
    }
    output
}

fn narrow_down(code: &[Op], start_a: u64, b: u64, c: u64, exp: u32) -> Vec<u64> {
    let chunk_size = 8u64.pow(exp);

    let possible: Vec<(u64, Registers)> = (0u64..8)
        .filter_map(|i| {
            let mut test_regs = Registers {
                a: start_a + (chunk_size * i),
                b,
                c,
            };
            let res = with_regs(code, &mut test_regs);
            if res.len() == code.len() && res[exp as usize] == code[exp as usize] {
                Some((start_a + (chunk_size * i), test_regs))
            } else {
                None
            }
        })
        .collect();

    if exp == 0 {
        possible.into_iter().map(|i| i.0).collect()
    } else {
        possible
            .into_iter()
            .map(|(start, new_regs)| narrow_down(code, start, new_regs.b, new_regs.c, exp - 1))
            .flatten()
            .collect()
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut input_lines = input.lines().skip(1);

    let reg_vals: [u64; 2] = core::array::from_fn(|_| {
        input_lines
            .next()
            .expect("Missing register line")
            .split_once(": ")
            .expect("Failed to find \": \" delimiter within a register line")
            .1
            .trim()
            .parse()
            .expect("Failed to parse register value")
    });

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

    // I've worked out a faster approach than checking every single possibility, but it relies on a
    // bunch of preconditions that are true of both my input and the sample input, but I can't say
    // whether or not they'd for all inputs.
    //
    // The preconditions are as follows:
    //  * the only jnz instruction is `jnz 0` at the very end, and the code length is even
    //  * there is only 1 single output instruction
    //  * the only adv instruction is `adv 3`
    //  * the out is second-to-last
    //
    //
    // Explanation:
    //
    // `a` is the starting value in the A register, and `l` is the number of loops, and if the
    // preconditions are met, `l` is also going to be equal to len(code)
    //
    // The first precondition is necessary only to simplify checking the other preconditions, as
    // the program will always loop until A contains 0
    //
    // The next 2 are necessary, as otherwise the number of outputs per loop will not be 1, and/or
    // the number of loops will not be equal to `(a/8) + 1`.
    //
    // If there's one output per round, and the number of loops will equal `(a/8) + 1,` then it's
    // possible to narrow down the set of numbers to check significantly, to the range
    // `(8.pow(l -1))..(8.pow(l))`.
    //
    // This is a potentially huge range, but it's a start.
    //
    // Next, the final precondition means that the output each round is based on (A/8)%8,
    // meaning that it's possible to figure out what a must be with a guess-and-check approach,
    // where the value in A at the start of the round decides what it is at the end, so it
    // essentially enable a search for possible octal digits of `a`.
    let mut can_go_fast = false;
    if code.len() % 2 == 0 {
        let (advs, jnzs): (Vec<(u8, u8)>, Vec<(u8, u8)>) = code
            .chunks(2)
            .filter_map(|c| {
                if *c[0] == 0 || *c[0] == 3 {
                    Some((*c[0], *c[1]))
                } else {
                    None
                }
            })
            .partition(|c| c.0 == 0);
        can_go_fast = code.chunks(2).filter(|c| *c[0] == 5).count() == 1
            && jnzs == vec![(3, 0)]
            && code[code.len() - 4] == Op(5)
            && code[code.len() - 6] == Op(0)
            && advs == vec![(0, 3)];
    }

    if input_lines.next().is_some() {
        panic!("Leftover lines in input after parsing");
    }

    if can_go_fast {
        let possible = narrow_down(
            code.as_slice(),
            0,
            reg_vals[0],
            reg_vals[1],
            (code.len() - 1) as u32,
        );
        for i in possible.iter() {
            assert!(
                with_regs(
                    code.as_slice(),
                    &mut Registers {
                        a: *i,
                        b: reg_vals[0],
                        c: reg_vals[1]
                    }
                ) == code
            );
        }
        println!(
            "{}",
            possible.into_iter().min().expect("No possible values")
        );
    } else {
        let mut a = 0;
        while with_regs(
            code.as_slice(),
            &mut Registers {
                a,
                b: reg_vals[0],
                c: reg_vals[1],
            },
        ) != code
        {
            a += 1
        }
        println!("{a}");
        return;
    }
}
