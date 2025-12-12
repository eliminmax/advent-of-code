// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 16 Part 1

type Regs = [u8; 4];
type InstrImpl = fn(Regs, u8, u8, u8) -> Regs;

macro_rules! instr_impl {
    (r, $op: tt) => {{
        |mut regs: Regs, a: u8, b: u8, c: u8| -> Regs {
            regs[usize::from(c)] = regs[usize::from(a)] $op regs[usize::from(b)];
            regs
        }
    }};
    (i, $op: tt) => {{
        |mut regs: Regs, a: u8, b: u8, c: u8| -> Regs {
            regs[usize::from(c)] = regs[usize::from(a)] $op b;
            regs
        }
    }};
    (ir, $op: tt) => {{
        |mut regs: Regs, a: u8, b: u8, c: u8| -> Regs {
            regs[usize::from(c)] = u8::from(a $op regs[usize::from(b)]);
            regs
        }
    }};
    (ri, $op: tt) => {{
        |mut regs: Regs, a: u8, b: u8, c: u8| -> Regs {
            regs[usize::from(c)] = u8::from(regs[usize::from(a)] $op b);
            regs
        }
    }};
    (rr, $op: tt) => {{
        |mut regs: Regs, a: u8, b: u8, c: u8| -> Regs {
            regs[usize::from(c)] = u8::from(regs[usize::from(a)] $op regs[usize::from(b)]);
            regs
        }
    }};
}

const INSTRUCTION_IMPLS: [InstrImpl; 16] = [
    // addr
    instr_impl!(r, +),
    // addi
    instr_impl!(i, +),
    // mulr
    instr_impl!(r, *),
    // muli
    instr_impl!(i, *),
    // banr
    instr_impl!(r, &),
    // bani
    instr_impl!(i, &),
    // borr
    instr_impl!(r, |),
    // bori
    instr_impl!(i, |),
    // setr
    |mut regs: Regs, a: u8, _b: u8, c: u8| -> Regs {
        regs[usize::from(c)] = regs[usize::from(a)];
        regs
    },
    // seti
    |mut regs: Regs, a: u8, _b: u8, c: u8| -> Regs {
        regs[usize::from(c)] = a;
        regs
    },
    // gtir
    instr_impl!(ir, >),
    // gtri
    instr_impl!(ri, >),
    // gtrr
    instr_impl!(rr, >),
    // eqir
    instr_impl!(ir, ==),
    // eqri
    instr_impl!(ri, ==),
    // eqrr
    instr_impl!(rr, ==),
];

fn matches_opcode(before: Regs, instr: [u8; 4], after: Regs) -> bool {
    INSTRUCTION_IMPLS
        .iter()
        .filter(|f| f(before, instr[1], instr[2], instr[3]) == after)
        .count()
        >= 3
}

fn main() {
    use std::convert::TryInto;
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let (tests_cases, _code) = input
        .split_once("\n\n\n\n")
        .expect("Failed to separate examples from program");

    let mut counter: u32 = 0;
    for example in tests_cases.split("\n\n") {
        let mut lines = example.lines();
        // This is nasty. I wish I had something like scanf, despite it's known flaws
        let regs_before: Regs = lines
            .next()
            .and_then(|line| line.strip_suffix(']'))
            .and_then(|line| line.strip_prefix("Before: ["))
            .expect("Unable to parse \"Before: \" line")
            .split(", ")
            .map(|n| n.parse().expect("Failed to parse starting register states"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Unable to convert into [u8; 4]");
        let instruction_bytes: [u8; 4] = lines
            .next()
            .expect("Unable to get instruction bytes")
            .split_whitespace()
            .map(|n| n.parse().expect("Failed to parse instruction bytes"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Unable to convert into [u8; 4]");
        let regs_after: Regs = lines
            .next()
            .and_then(|line| line.strip_suffix(']'))
            .and_then(|line| line.strip_prefix("After:  ["))
            .expect("Unable to parse \"After: \" line")
            .split(", ")
            .map(|n| n.parse().expect("Failed to parse ending register states"))
            .collect::<Vec<_>>()
            .try_into()
            .expect("Unable to convert into [u8; 4]");
        if matches_opcode(regs_before, instruction_bytes, regs_after) {
            counter += 1;
        }
    }
    println!("{counter}");
}
