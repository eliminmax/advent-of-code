// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 10 Part 1

use std::iter::Cycle;
use std::num::NonZero;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Adding { n: i64, finishing: bool },
    Idling,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Addx(NonZero<i64>),
    Noop,
}

struct Cpu<I: Clone + Iterator<Item = Instruction>> {
    register: i64,
    instructions: Cycle<I>,
    state: State,
}

impl<I: Clone + Iterator<Item = Instruction>> Cpu<I> {
    fn new(instructions: I) -> Self {
        Self {
            register: 1,
            instructions: instructions.cycle(),
            state: State::Idling,
        }
    }

    fn load_instruction(&mut self) {
        match self.instructions.next().unwrap() {
            Instruction::Noop => self.state = State::Idling,
            Instruction::Addx(n) => {
                self.state = State::Adding {
                    n: n.get(),
                    finishing: false,
                }
            }
        }
    }

    fn cycle(&mut self) -> i64 {
        let mut ret = self.register;
        match self.state {
            State::Adding {
                n,
                finishing: false,
            } => {
                self.state = State::Adding { n, finishing: true };
            }
            State::Adding { n, finishing: true } => {
                ret = self.register;
                self.register += n;
                self.load_instruction();
            }
            State::Idling => self.load_instruction(),
        }
        ret
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut cpu = Cpu::new(
        input
            .lines()
            .map(|l| match l.trim() {
                "noop" => Ok(Instruction::Noop),
                s => Ok(Instruction::Addx(
                    s.strip_prefix("addx ").ok_or(BadInstr(l.into()))?.parse()?,
                )),
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error>>>()?
            .into_iter(),
    );
    let mut total = 0;

    for cycle_count in 0..=220 {
        let x = cpu.cycle();
        if matches!(cycle_count, 20 | 60 | 100 | 140 | 180 | 220) {
            total += cycle_count * x;
        }
    }

    println!("{total}");
    Ok(())
}

#[derive(Debug)]
struct BadInstr(String);
impl std::fmt::Display for BadInstr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "could not parse {:?} as an instruction", self.0)
    }
}
impl std::error::Error for BadInstr {}
