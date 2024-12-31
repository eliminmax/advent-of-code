// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 12 Part 2

#[derive(Debug)]
struct AssembunnyParseError;
impl From<std::num::ParseIntError> for AssembunnyParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        AssembunnyParseError
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum RegId {
    A,
    B,
    C,
    D,
}

impl std::str::FromStr for RegId {
    type Err = AssembunnyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(RegId::A),
            "b" => Ok(RegId::B),
            "c" => Ok(RegId::C),
            "d" => Ok(RegId::D),
            _ => Err(AssembunnyParseError),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Param {
    RegMode(RegId),
    Imm(i32),
}

impl std::str::FromStr for Param {
    type Err = AssembunnyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(r) = RegId::from_str(s) {
            Ok(Param::RegMode(r))
        } else {
            Ok(Param::Imm(s.parse()?))
        }
    }
}

#[derive(Debug, Default)]
struct Regs {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl std::ops::Index<RegId> for Regs {
    type Output = i32;
    fn index(&self, reg_id: RegId) -> &i32 {
        match reg_id {
            RegId::A => &self.a,
            RegId::B => &self.b,
            RegId::C => &self.c,
            RegId::D => &self.d,
        }
    }
}

impl std::ops::IndexMut<RegId> for Regs {
    fn index_mut(&mut self, reg_id: RegId) -> &mut i32 {
        match reg_id {
            RegId::A => &mut self.a,
            RegId::B => &mut self.b,
            RegId::C => &mut self.c,
            RegId::D => &mut self.d,
        }
    }
}

impl Regs {
    fn value_of(&self, param: Param) -> i32 {
        match param {
            Param::RegMode(reg_id) => self[reg_id],
            Param::Imm(i) => i,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Cpy(Param, RegId),
    Inc(RegId),
    Dec(RegId),
    Jnz(Param, i32),
}

impl std::str::FromStr for Instruction {
    type Err = AssembunnyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_whitespace().collect();
        match &words[..] {
            ["cpy", x, y] => Ok(Instruction::Cpy(x.parse()?, y.parse()?)),
            ["inc", x] => Ok(Instruction::Inc(x.parse()?)),
            ["dec", x] => Ok(Instruction::Dec(x.parse()?)),
            ["jnz", x, y] => Ok(Instruction::Jnz(x.parse()?, y.parse()?)),
            _ => Err(AssembunnyParseError),
        }
    }
}

fn interpret(code: &[Instruction]) -> Regs {
    use std::convert::TryInto;
    let mut regs = Regs::default();
    regs[RegId::C] = 1;
    let mut index: usize = 0;
    while let Some(instr) = code.get(index) {
        let mut next_index = index + 1;
        match instr {
            Instruction::Cpy(x, y) => regs[*y] = regs.value_of(*x),
            Instruction::Inc(x) => regs[*x] += 1,
            Instruction::Dec(x) => regs[*x] -= 1,
            Instruction::Jnz(x, y) => {
                if regs.value_of(*x) != 0 {
                    let offset: isize = TryInto::<isize>::try_into(*y)
                        .expect("Offset larger than platform address size");
                    next_index = index
                        .checked_add_signed(offset)
                        .expect("Can't jump to before program start");
                }
            }
        }
        index = next_index;
    }
    regs
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let code: Vec<Instruction> = input
        .lines()
        .map(|l| l.parse().expect("Failed to parse instruction"))
        .collect();
    let ending_regs = interpret(&code[..]);
    println!("{}", ending_regs[RegId::A]);
}
