// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 23 Part 1

// Most of the gnarly impls are below main

#[derive(Debug, Default, Clone)]
struct Regs([i64; 8]);

#[derive(Debug, Copy, Clone, PartialEq)]
enum RegId {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl Regs {
    fn value_of(&self, p: &Param) -> i64 {
        match p {
            Param::RegId(r) => self[r],
            Param::Imm(i) => *i,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Param {
    RegId(RegId),
    Imm(i64),
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    Set(RegId, Param),
    Sub(RegId, Param),
    Mul(RegId, Param),
    Jnz(Param, Param),
}

/// Run while possible, and return the number of mul instructions run
fn count_muls(code: &[Instruction]) -> u32 {
    use Instruction as I;
    use std::convert::TryInto;
    let mut regs = Regs::default();
    let mut index: usize = 0;
    let mut muls: u32 = 0;
    while let Some(instr) = code.get(index) {
        let mut offset: isize = 1;
        match instr {
            I::Set(x, y) => regs[x] = regs.value_of(y),
            I::Sub(x, y) => regs[x] -= regs.value_of(y),
            I::Mul(x, y) => {
                regs[x] *= regs.value_of(y);
                muls += 1;
            }
            I::Jnz(x, y) => {
                if regs.value_of(x) != 0 {
                    offset = regs
                        .value_of(y)
                        .try_into()
                        .expect("Jump too large for system");
                }
            }
        }
        if let Some(next_index) = index.checked_add_signed(offset) {
            index = next_index;
        } else {
            break;
        }
    }
    muls
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
    println!("{}", count_muls(&code[..]));
}

impl std::ops::Index<&RegId> for Regs {
    type Output = i64;
    fn index(&self, reg_id: &RegId) -> &i64 {
        &self.0[*reg_id as usize]
    }
}

impl std::ops::IndexMut<&RegId> for Regs {
    fn index_mut(&mut self, reg_id: &RegId) -> &mut i64 {
        &mut self.0[*reg_id as usize]
    }
}

#[derive(Debug)]
enum InstructionParseError {
    UnknownOp,
    NumParseFailure,
    InvalidReg,
}

impl std::str::FromStr for RegId {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "e" => Ok(Self::E),
            "f" => Ok(Self::F),
            "g" => Ok(Self::G),
            "h" => Ok(Self::H),
            _ => Err(InstructionParseError::InvalidReg),
        }
    }
}

impl std::str::FromStr for Param {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 && s.as_bytes()[0].is_ascii_lowercase() {
            Ok(Param::RegId(s.parse()?))
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
            ["set", x, y] => Ok(I::Set(x.parse()?, y.parse()?)),
            ["sub", x, y] => Ok(I::Sub(x.parse()?, y.parse()?)),
            ["mul", x, y] => Ok(I::Mul(x.parse()?, y.parse()?)),
            ["jnz", x, y] => Ok(I::Jnz(x.parse()?, y.parse()?)),
            _ => Err(InstructionParseError::UnknownOp),
        }
    }
}
