// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 19 Part 1

use std::num::ParseIntError;

type Regs = [usize; 6];

fn execute_with_binding(code: &[Instruction], ip_binding: usize) -> Regs {
    let mut regs: Regs = [0; 6];
    let mut ip: usize = 0;
    while let Some(Instruction(opcode, args)) = code.get(ip) {
        regs[ip_binding] = ip;
        match opcode {
            Opcode::Addr => regs[args.c] = regs[args.a] + regs[args.b],
            Opcode::Addi => regs[args.c] = regs[args.a] + args.b,
            Opcode::Mulr => regs[args.c] = regs[args.a] * regs[args.b],
            Opcode::Muli => regs[args.c] = regs[args.a] * args.b,
            Opcode::Banr => regs[args.c] = regs[args.a] & regs[args.b],
            Opcode::Bani => regs[args.c] = regs[args.a] & args.b,
            Opcode::Borr => regs[args.c] = regs[args.a] | regs[args.b],
            Opcode::Bori => regs[args.c] = regs[args.a] | args.b,
            Opcode::Setr => regs[args.c] = regs[args.a],
            Opcode::Seti => regs[args.c] = args.a,
            Opcode::Gtir => regs[args.c] = usize::from(args.a > regs[args.b]),
            Opcode::Gtri => regs[args.c] = usize::from(regs[args.a] > args.b),
            Opcode::Gtrr => regs[args.c] = usize::from(regs[args.a] > regs[args.b]),
            Opcode::Eqir => regs[args.c] = usize::from(args.a == regs[args.b]),
            Opcode::Eqri => regs[args.c] = usize::from(regs[args.a] == args.b),
            Opcode::Eqrr => regs[args.c] = usize::from(regs[args.a] == regs[args.b]),
        }
        ip = regs[ip_binding];
        ip += 1;
    }
    regs
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Args {
    a: usize,
    b: usize,
    c: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Instruction(Opcode, Args);

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut lines = input.lines();
    let ip_binding: usize = lines
        .next()
        .and_then(|l| {
            if let ["#ip", ip] = &l.split_whitespace().collect::<Vec<_>>()[..] {
                Some(ip.parse().expect("ip binding could not be parsed"))
            } else {
                None
            }
        })
        .expect("ip binding line missing");
    let instructions: Vec<Instruction> = lines
        .map(|line| line.parse().expect("Invalid instruction"))
        .collect();
    println!("{}", execute_with_binding(&instructions[..], ip_binding)[0]);
}

impl std::str::FromStr for Instruction {
    type Err = InstructionParseFailure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        macro_rules! parse_int {
            ($str: ident) => {{
                $str.parse()
                    .map_err(|e| InstructionParseFailure::IntParseFailure(s.into(), e))
            }};
        }
        if let [opcode, a, b, c] = s.split_whitespace().collect::<Vec<_>>().as_slice() {
            let args = Args {
                a: parse_int!(a)?,
                b: parse_int!(b)?,
                c: parse_int!(c)?,
            };

            let opcode = match *opcode {
                "addr" => Ok(Opcode::Addr),
                "addi" => Ok(Opcode::Addi),
                "mulr" => Ok(Opcode::Mulr),
                "muli" => Ok(Opcode::Muli),
                "banr" => Ok(Opcode::Banr),
                "bani" => Ok(Opcode::Bani),
                "borr" => Ok(Opcode::Borr),
                "bori" => Ok(Opcode::Bori),
                "setr" => Ok(Opcode::Setr),
                "seti" => Ok(Opcode::Seti),
                "gtir" => Ok(Opcode::Gtir),
                "gtri" => Ok(Opcode::Gtri),
                "gtrr" => Ok(Opcode::Gtrr),
                "eqir" => Ok(Opcode::Eqir),
                "eqri" => Ok(Opcode::Eqri),
                "eqrr" => Ok(Opcode::Eqrr),
                _ => Err(InstructionParseFailure::UnknownOpcode(Box::from(s))),
            }?;
            Ok(Instruction(opcode, args))
        } else {
            Err(InstructionParseFailure::FormattingError(Box::from(s)))
        }
    }
}

#[derive(Debug)]
enum InstructionParseFailure {
    FormattingError(#[allow(unused)] Box<str>),
    UnknownOpcode(#[allow(unused)] Box<str>),
    IntParseFailure(#[allow(unused)] Box<str>, #[allow(unused)] ParseIntError),
}
