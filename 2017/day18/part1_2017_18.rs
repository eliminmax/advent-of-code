// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 18 Part 1

// All of the gnarly impls are below main
#[derive(Debug, Default)]
struct Regs([i64; 26]);

impl Regs {
    fn value_of(&self, p: &Param) -> i64 {
        match p {
            Param::RegId(r) => self[*r],
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
    Snd(Param),
    Set(RegId, Param),
    Add(RegId, Param),
    Mul(RegId, Param),
    Mod(RegId, Param),
    Rcv(Param),
    Jgz(Param, Param),
}

fn first_rcv(code: &[Instruction]) -> Option<i64> {
    use std::convert::TryInto;
    use Instruction as I;
    let mut index: usize = 0;
    let mut regs = Regs::default();
    let mut recent_sound: Option<i64> = None;
    while let Some(instr) = code.get(index) {
        let mut offset: isize = 1;
        match instr {
            I::Snd(x) => recent_sound = Some(regs.value_of(x)),
            I::Set(x, y) => regs[*x] = regs.value_of(y),
            I::Add(x, y) => regs[*x] += regs.value_of(y),
            I::Mul(x, y) => regs[*x] *= regs.value_of(y),
            I::Mod(x, y) => regs[*x] %= regs.value_of(y),
            I::Rcv(x) => {
                if regs.value_of(x) != 0 {
                    return recent_sound;
                }
            }
            I::Jgz(x, y) => {
                if regs.value_of(x) > 0 {
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
            return None;
        }
    }
    None
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
    println!(
        "{}",
        first_rcv(&code[..]).expect("No sounds played before rcv called with nonzero value")
    );
}

type RegId = u8;

impl std::ops::Index<RegId> for Regs {
    type Output = i64;
    fn index(&self, reg_id: RegId) -> &i64 {
        assert!(reg_id.is_ascii_lowercase());
        &self.0[(reg_id - b'a') as usize]
    }
}

impl std::ops::IndexMut<RegId> for Regs {
    fn index_mut(&mut self, reg_id: RegId) -> &mut i64 {
        assert!(reg_id.is_ascii_lowercase());
        &mut self.0[(reg_id - b'a') as usize]
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
            ["rcv", x] => Ok(I::Rcv(x.parse()?)),
            ["jgz", x, y] => Ok(I::Jgz(x.parse()?, y.parse()?)),
            _ => Err(InstructionParseError::UnknownOp),
        }
    }
}
