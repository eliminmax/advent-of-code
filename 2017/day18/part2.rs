// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 18 Part 2

use std::collections::VecDeque;

// Most of the gnarly impls are below main

#[derive(Debug, Default, Clone)]
struct Regs([i64; 26]);

impl Regs {
    fn value_of(&self, p: &Param) -> i64 {
        match p {
            Param::RegId(r) => self[r],
            Param::Imm(i) => *i,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ExitStatus {
    Awaiting,
    Halted,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Param {
    RegId(RegId),
    Imm(i64),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Snd(Param),
    Set(RegId, Param),
    Add(RegId, Param),
    Mul(RegId, Param),
    Mod(RegId, Param),
    Rcv(RegId),
    Jgz(Param, Param),
}

type SignalQueue = VecDeque<i64>;

#[derive(Debug, Clone)]
struct RunState<'a> {
    code: &'a [Instruction],
    regs: Regs,
    snd_queue: SignalQueue,
    index: usize,
    total_sent: u32,
    state: ExitStatus,
}

impl<'a> RunState<'a> {
    fn new(id: i64, code: &'a [Instruction]) -> Self {
        let mut regs = Regs::default();
        regs[&b'p'] = id;
        RunState {
            code,
            regs,
            snd_queue: SignalQueue::new(),
            index: 0,
            total_sent: 0,
            state: ExitStatus::Awaiting,
        }
    }
}

impl RunState<'_> {
    /// Run until either halted or waiting on next signal
    fn run_until_blocked(&mut self, other: &mut Self) {
        use std::convert::TryInto;
        if self.state == ExitStatus::Halted {
            return;
        }
        use Instruction as I;
        while let Some(instr) = self.code.get(self.index) {
            let mut offset: isize = 1;
            match instr {
                I::Snd(x) => {
                    self.snd_queue.push_back(self.regs.value_of(x));
                    self.total_sent += 1;
                }
                I::Set(x, y) => self.regs[x] = self.regs.value_of(y),
                I::Add(x, y) => self.regs[x] += self.regs.value_of(y),
                I::Mul(x, y) => self.regs[x] *= self.regs.value_of(y),
                I::Mod(x, y) => self.regs[x] %= self.regs.value_of(y),
                I::Rcv(x) => {
                    if let Some(sig) = other.snd_queue.pop_front() {
                        self.regs[x] = sig;
                    } else {
                        if other.state == ExitStatus::Halted {
                            // if other is halted, then queue will never fill, so halt
                            self.state = ExitStatus::Halted;
                        } else if self.snd_queue.is_empty() {
                            // if snd_queues are both empty, it's a deadlock, so both should halt
                            self.state = ExitStatus::Halted;
                            other.state = ExitStatus::Halted;
                        }
                        return;
                    }
                }
                I::Jgz(x, y) => {
                    if self.regs.value_of(x) > 0 {
                        offset = self
                            .regs
                            .value_of(y)
                            .try_into()
                            .expect("Jump too large for system");
                    }
                }
            }
            if let Some(next_index) = self.index.checked_add_signed(offset) {
                self.index = next_index;
            } else {
                self.state = ExitStatus::Halted;
                return;
            }
        }
        self.state = ExitStatus::Halted;
    }
}

fn run_pair(code: &[Instruction]) -> u32 {
    let mut runner0 = RunState::new(0, code);
    let mut runner1 = RunState::new(1, code);
    while (runner0.state, runner1.state) != (ExitStatus::Halted, ExitStatus::Halted) {
        runner0.run_until_blocked(&mut runner1);
        runner1.run_until_blocked(&mut runner0);
    }
    runner1.total_sent
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let code: Vec<Instruction> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse instruction"))
        .collect();
    println!("{}", run_pair(&code[..]));
}

type RegId = u8;

impl std::ops::Index<&RegId> for Regs {
    type Output = i64;
    fn index(&self, reg_id: &RegId) -> &i64 {
        assert!(reg_id.is_ascii_lowercase());
        &self.0[(*reg_id - b'a') as usize]
    }
}

impl std::ops::IndexMut<&RegId> for Regs {
    fn index_mut(&mut self, reg_id: &RegId) -> &mut i64 {
        assert!(reg_id.is_ascii_lowercase());
        &mut self.0[(*reg_id - b'a') as usize]
    }
}

#[derive(Debug)]
enum InstructionParseError {
    UnknownOp,
    NumParseFailure,
    InvalidReg,
}

fn reg_id_from(s: &str) -> Result<RegId, InstructionParseError> {
    if s.len() == 1 && s.as_bytes()[0].is_ascii_lowercase() {
        Ok(s.as_bytes()[0])
    } else {
        Err(InstructionParseError::InvalidReg)
    }
}

impl std::str::FromStr for Param {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 && s.as_bytes()[0].is_ascii_lowercase() {
            Ok(Param::RegId(s.as_bytes()[0]))
        } else {
            Ok(Param::Imm(s.parse()?))
        }
    }
}

impl From<std::num::ParseIntError> for InstructionParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        InstructionParseError::NumParseFailure
    }
}

impl std::str::FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction as I;
        let words: Vec<_> = s.split_whitespace().collect();
        match &words[..] {
            ["snd", x] => Ok(I::Snd(x.parse()?)),
            ["set", x, y] => Ok(I::Set(reg_id_from(x)?, y.parse()?)),
            ["add", x, y] => Ok(I::Add(reg_id_from(x)?, y.parse()?)),
            ["mul", x, y] => Ok(I::Mul(reg_id_from(x)?, y.parse()?)),
            ["mod", x, y] => Ok(I::Mod(reg_id_from(x)?, y.parse()?)),
            ["rcv", x] => Ok(I::Rcv(reg_id_from(x)?)),
            ["jgz", x, y] => Ok(I::Jgz(x.parse()?, y.parse()?)),
            _ => Err(InstructionParseError::UnknownOp),
        }
    }
}
