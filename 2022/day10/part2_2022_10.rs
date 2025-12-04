// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 10 Part 2

use std::io::{Write, stdout};
use std::num::NonZero;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum State {
    Adding { n: i16, finishing: bool },
    Idling,
    Finalizing,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    Addx(NonZero<i16>),
    Noop,
}

struct Cpu<I: Iterator<Item = Instruction>> {
    register: i16,
    instructions: I,
    state: State,
}

macro_rules! ansi_esc {
    (position [$r:expr, $c:expr]) => {
        print!("\x1b[{};{}H", $r + 1, $c + 1)
    };
    (invert) => {
        print!("\x1b[7m")
    };
    (uninvert) => {
        print!("\x1b[27m")
    };
    (reset_format) => {
        print!("\x1b[m")
    };
    (clear) => {
        ansi_esc!(position[0, 0]);
        print!("\x1bc")
    }
    
}

impl<I: Iterator<Item = Instruction>> Cpu<I> {
    fn new(instructions: I) -> Self {
        let mut s = Self {
            register: 1,
            instructions,
            state: State::Idling,
        };
        s.load_instruction();
        s
    }

    fn load_instruction(&mut self) {
        match self.instructions.next() {
            Some(Instruction::Noop) => self.state = State::Idling,
            Some(Instruction::Addx(n)) => {
                self.state = State::Adding {
                    n: n.get(),
                    finishing: false,
                }
            }
            None => self.state = State::Finalizing,
        }
    }


    fn render(self) {
        ansi_esc!(clear);
        ansi_esc!(reset_format);
        let mut row = 0;
        let mut col = 0u8;
        for x in self {
            ansi_esc!(position [row, col]);
            if x.abs_diff(col.into()) <= 1 {
                ansi_esc!(invert);
            } else {
                ansi_esc!(uninvert);
            }
            print!(" ");
            stdout().flush().expect("writable stdout");
            col += 1;
            if col == 40 {
                col = 0;
                row += 1;
                row %= 6;
            }
        }
        ansi_esc!(position [6, 0]);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let cpu = Cpu::new(
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
    cpu.render();

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
impl<I: Iterator<Item = Instruction>> Iterator for Cpu<I> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = self.register;
        match self.state {
            State::Adding { n, finishing: false } => {
                self.state = State::Adding { n, finishing: true };
                return Some(self.register);
            }
            State::Adding { n, finishing: true } => {
                ret = self.register;
                self.register += n;
                self.load_instruction();
            }
            State::Idling => self.load_instruction(),
            State::Finalizing => return None,
        }
        Some(ret)
    }
}
