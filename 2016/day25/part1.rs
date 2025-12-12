// SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 25 Part 1

#[derive(Debug)]
struct AssembunnyParseError;
impl From<std::num::ParseIntError> for AssembunnyParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        AssembunnyParseError
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
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

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
struct Regs {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl std::ops::Index<&RegId> for Regs {
    type Output = i32;
    fn index(&self, reg_id: &RegId) -> &i32 {
        match reg_id {
            RegId::A => &self.a,
            RegId::B => &self.b,
            RegId::C => &self.c,
            RegId::D => &self.d,
        }
    }
}

impl std::ops::IndexMut<&RegId> for Regs {
    fn index_mut(&mut self, reg_id: &RegId) -> &mut i32 {
        match reg_id {
            RegId::A => &mut self.a,
            RegId::B => &mut self.b,
            RegId::C => &mut self.c,
            RegId::D => &mut self.d,
        }
    }
}

impl Regs {
    fn value_of(&self, param: &Param) -> i32 {
        match param {
            Param::RegMode(reg_id) => self[reg_id],
            Param::Imm(i) => *i,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Instruction {
    Cpy(Param, Param),
    Inc(Param),
    Dec(Param),
    Jnz(Param, Param),
    Tgl(Param),
    Out(Param),
}

impl Instruction {
    fn toggle(&mut self) {
        use Instruction as I;
        *self = match self {
            I::Dec(x) | I::Out(x) | I::Tgl(x) => I::Inc(*x),
            I::Inc(x) => I::Dec(*x),
            I::Jnz(x, y) => I::Cpy(*x, *y),
            I::Cpy(x, y) => I::Jnz(*x, *y),
        };
    }
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
            ["tgl", x] => Ok(Instruction::Tgl(x.parse()?)),
            ["out", x] => Ok(Instruction::Out(x.parse()?)),
            _ => Err(AssembunnyParseError),
        }
    }
}

fn creates_clock(start_signal: i32, code: &[Instruction]) -> bool {
    use std::collections::HashSet;
    use std::convert::TryInto;
    use std::iter::FromIterator;
    let mut seen_states: HashSet<(usize, Regs, Vec<Instruction>)> = HashSet::new();
    let mut index: usize = 0;
    let mut regs = Regs {
        a: start_signal,
        ..Regs::default()
    };
    let mut code = code.to_owned();
    let mut outputs: Vec<i32> = Vec::new();
    while let Some(instr) = code.get(index) {
        if !seen_states.insert((index, regs.clone(), code.clone())) {
            // this block runs if the current state is identical to a previous one, meaning that
            // it's looping.
            // I'm assuming that that's the only way to find the solution, as the only other idea I
            // have would be solving the Halting Problem for Assembunny, which is famously
            // impossible for Turing-complete systems (which Assembunny may or may not be).
            return outputs == [0, 1].repeat(outputs.len() / 2);
        }
        let mut next_index = index + 1;
        match instr {
            Instruction::Cpy(x, Param::RegMode(y)) => regs[y] = regs.value_of(x),
            Instruction::Inc(Param::RegMode(x)) => regs[x] += 1,
            Instruction::Dec(Param::RegMode(x)) => regs[x] -= 1,
            Instruction::Jnz(x, y) => {
                if regs.value_of(x) != 0 {
                    let offset: isize = TryInto::<isize>::try_into(regs.value_of(y))
                        .expect("Offset larger than platform address size");
                    next_index = index
                        .checked_add_signed(offset)
                        .expect("Can't jump to before program start");
                }
            }
            Instruction::Tgl(x) => {
                let offset: isize = TryInto::<isize>::try_into(regs.value_of(x))
                    .expect("Offset larger than platform address size");
                if let Some(tgt) = index
                    .checked_add_signed(offset)
                    .and_then(|i| code.get_mut(i))
                {
                    tgt.toggle();
                }
            }
            Instruction::Out(x) => {
                outputs.push(regs.value_of(x));
                let expected =
                    Vec::from_iter([0i32, 1].iter().cycle().take(outputs.len()).cloned());
                if outputs != expected {
                    return false;
                }
            }
            _ => (),
        }
        index = next_index;
    }
    false
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
    for i in 1.. {
        if creates_clock(i, &code[..]) {
            println!("{i}");
            break;
        }
    }
}
