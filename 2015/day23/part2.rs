// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 23 Part 2

#[derive(Debug, PartialEq, Clone, Copy)]
enum Reg {
    A = 0,
    B = 1,
}

impl std::ops::Index<Reg> for [u32; 2] {
    type Output = u32;
    fn index(&self, r: Reg) -> &u32 {
        &self[r as usize]
    }
}
impl std::ops::IndexMut<Reg> for [u32; 2] {
    fn index_mut(&mut self, r: Reg) -> &mut u32 {
        &mut self[r as usize]
    }
}

#[derive(Debug)]
struct InstructionParseError;

impl std::str::FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.split_whitespace();
        let mnemonic = words.next().ok_or(InstructionParseError)?;
        macro_rules! parse_instruction {
            ($instr: ident, REG) => {{
                let reg = match words.next() {
                    Some("a") => Reg::A,
                    Some("b") => Reg::B,
                    _ => return Err(InstructionParseError),
                };
                Instruction::$instr(reg)
            }};
            ($instr: ident, OFFSET) => {{
                let offset: isize = words
                    .next()
                    .ok_or(InstructionParseError)?
                    .parse()
                    .map_err(|_| InstructionParseError)?;
                Instruction::$instr(offset)
            }};
            ($instr: ident, BOTH) => {{
                let reg = match words.next() {
                    Some("a,") => Reg::A,
                    Some("b,") => Reg::B,
                    _ => return Err(InstructionParseError),
                };
                let offset: isize = words
                    .next()
                    .ok_or(InstructionParseError)?
                    .parse()
                    .map_err(|_| InstructionParseError)?;
                Instruction::$instr(reg, offset)
            }};
        }
        let instr = match mnemonic {
            "hlf" => Ok(parse_instruction!(Hlf, REG)),
            "tpl" => Ok(parse_instruction!(Tpl, REG)),
            "inc" => Ok(parse_instruction!(Inc, REG)),
            "jmp" => Ok(parse_instruction!(Jmp, OFFSET)),
            "jie" => Ok(parse_instruction!(Jie, BOTH)),
            "jio" => Ok(parse_instruction!(Jio, BOTH)),
            _ => Err(InstructionParseError),
        };
        if words.next().is_none() {
            instr
        } else {
            Err(InstructionParseError)
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Instruction {
    Hlf(Reg),
    Tpl(Reg),
    Inc(Reg),
    Jmp(isize),
    Jie(Reg, isize),
    Jio(Reg, isize),
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let program: Vec<Instruction> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse instruction"))
        .collect();
    let mut i = 0usize;
    let mut reg_contents: [u32; 2] = [1, 0];
    while let Some(&instr) = program.get(i) {
        let mut offset = 1isize;
        match instr {
            Instruction::Hlf(r) => reg_contents[r] /= 2,
            Instruction::Tpl(r) => reg_contents[r] *= 3,
            Instruction::Inc(r) => reg_contents[r] += 1,
            Instruction::Jmp(o) => offset = o,
            Instruction::Jie(r, o) => {
                if reg_contents[r] % 2 == 0 {
                    offset = o
                }
            }
            Instruction::Jio(r, o) => {
                if reg_contents[r] == 1 {
                    offset = o
                }
            }
        }
        i = i
            .checked_add_signed(offset)
            .expect("Program underflow error");
    }
    println!("{}", reg_contents[Reg::B]);
}
