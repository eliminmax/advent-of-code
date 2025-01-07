// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 8 Part 1

use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
enum InvalidInstruction {
    BadNumber,
    UnrecognizedOp,
    UnrecognizedCond,
    BadFormat,
}

impl From<std::num::ParseIntError> for InvalidInstruction {
    fn from(_e: std::num::ParseIntError) -> Self {
        InvalidInstruction::BadNumber
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    Inc,
    Dec,
}

impl FromStr for Operation {
    type Err = InvalidInstruction;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            _ => Err(InvalidInstruction::UnrecognizedOp),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Cond {
    GreaterThan,
    LessThan,
    Equal,
    NotEqual,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl Cond {
    fn eval(&self, reg: i32, val: i32) -> bool {
        match self {
            Cond::GreaterThan => reg > val,
            Cond::LessThan => reg < val,
            Cond::Equal => reg == val,
            Cond::NotEqual => reg != val,
            Cond::LessThanOrEqual => reg <= val,
            Cond::GreaterThanOrEqual => reg >= val,
        }
    }
}

impl FromStr for Cond {
    type Err = InvalidInstruction;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Cond::GreaterThan),
            "<" => Ok(Cond::LessThan),
            "==" => Ok(Cond::Equal),
            "!=" => Ok(Cond::NotEqual),
            "<=" => Ok(Cond::LessThanOrEqual),
            ">=" => Ok(Cond::GreaterThanOrEqual),
            _ => Err(InvalidInstruction::UnrecognizedCond),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    reg: String,
    op: Operation,
    val: i32,
    cond_reg: String,
    cond: Cond,
    cond_val: i32,
}

type Registers = HashMap<String, i32>;

impl Instruction {
    fn exec(self, regs: &mut Registers) {
        let Instruction {
            reg,
            op,
            val,
            cond_reg,
            cond,
            cond_val,
        } = self;
        if cond.eval(*regs.get(&cond_reg).unwrap_or(&0), cond_val) {
            match op {
                Operation::Inc => regs.entry(reg).and_modify(|e| *e += val).or_insert(val),
                Operation::Dec => regs.entry(reg).and_modify(|e| *e -= val).or_insert(-val),
            };
        }
    }
}

impl FromStr for Instruction {
    type Err = InvalidInstruction;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        if let [reg, op, val, "if", cond_reg, cond, cond_val] = words[..] {
            let reg = String::from(reg);
            let op = Operation::from_str(op)?;
            let val = i32::from_str(val)?;
            let cond_reg = String::from(cond_reg);
            let cond = Cond::from_str(cond)?;
            let cond_val = i32::from_str(cond_val)?;
            Ok(Instruction {
                reg,
                op,
                val,
                cond_reg,
                cond,
                cond_val,
            })
        } else {
            Err(InvalidInstruction::BadFormat)
        }
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut regs = Registers::new();
    input.lines().for_each(|line| {
        Instruction::from_str(line)
            .expect("Failed to parse line as instruction")
            .exec(&mut regs);
    });

    println!("{}", regs.into_values().max().unwrap_or(0));
}
