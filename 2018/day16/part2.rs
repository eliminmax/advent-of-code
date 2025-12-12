// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 16 Part 2
use std::collections::BTreeSet;
use std::convert::TryInto;
use std::num::ParseIntError;

type Regs = [usize; 4];
type InstrImpl = fn(Regs, usize, usize, usize) -> Regs;

macro_rules! instr_impl {
    (r, $op: tt) => {{
        |mut regs: Regs, a: usize, b: usize, c: usize| -> Regs {
            regs[c] = regs[a] $op regs[b];
            regs
        }
    }};
    (i, $op: tt) => {{
        |mut regs: Regs, a: usize, b: usize, c: usize| -> Regs {
            regs[c] = regs[a] $op b;
            regs
        }
    }};
    (ir, $op: tt) => {{
        |mut regs: Regs, a: usize, b: usize, c: usize| -> Regs {
            regs[c] = usize::from(a $op regs[b]);
            regs
        }
    }};
    (ri, $op: tt) => {{
        |mut regs: Regs, a: usize, b: usize, c: usize| -> Regs {
            regs[c] = usize::from(regs[a] $op b);
            regs
        }
    }};
    (rr, $op: tt) => {{
        |mut regs: Regs, a: usize, b: usize, c: usize| -> Regs {
            regs[c] = usize::from(regs[a] $op regs[b]);
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
    |mut regs: Regs, a: usize, _b: usize, c: usize| -> Regs {
        regs[c] = regs[a];
        regs
    },
    // seti
    |mut regs: Regs, a: usize, _b: usize, c: usize| -> Regs {
        regs[c] = a;
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

type Mapping = [usize; 16];
#[derive(Debug, PartialEq)]
struct SampleOutcome {
    before: Regs,
    instruction: [usize; 4],
    after: Regs,
}

fn create_mapping<'a, I>(samples: I) -> Result<Mapping, MapResolveFailure>
where
    I: Iterator<Item = &'a str>,
    I: Clone,
{
    use INSTRUCTION_IMPLS as INSTR_IMPS;
    use MapResolveFailure::NoPossibleResolution;
    let samples = samples.map(|sample| sample.parse()).cycle();
    let mut possibilities: [BTreeSet<usize>; 16] = core::array::from_fn(|_| (0..16).collect());
    for sample in samples {
        let SampleOutcome {
            before,
            instruction: [op, a, b, c],
            after,
        } = sample?;
        possibilities[op].retain(|i| INSTR_IMPS[*i](before, a, b, c) == after);
        if possibilities[op].is_empty() {
            return Err(NoPossibleResolution(op));
        }
        if possibilities[op].len() == 1 {
            let resolved_opcode = possibilities[op]
                .first()
                .copied()
                .unwrap_or_else(|| unreachable!());
            possibilities
                .iter_mut()
                .enumerate()
                .filter_map(|(i, set)| if i != op { Some(set) } else { None })
                .for_each(|set| set.retain(|opcode| *opcode != resolved_opcode));
        }
        if possibilities.iter().all(|set| set.len() == 1) {
            return Ok(core::array::from_fn(|i| {
                possibilities[i].first().copied().unwrap_or_else(|| {
                    unreachable!("All entries have exactly one possibility left")
                })
            }));
        }
    }
    unreachable!("will keep cycling until everything is resolved")
}
fn execute_with<'a, I>(opcode_map: Mapping, program: I) -> Result<Regs, ProgramParseError>
where
    I: Iterator<Item = &'a str>,
{
    let operations: [_; 16] = core::array::from_fn(|i| INSTRUCTION_IMPLS[opcode_map[i]]);
    let mut regs: Regs = [0; 4];
    for line in program {
        let [op, a, b, c]: [usize; 4] = line
            .split_whitespace()
            .map(|word| word.parse())
            .collect::<Result<Vec<_>, _>>()?
            .try_into()?;
        regs = operations[op](regs, a, b, c);
    }
    Ok(regs)
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let (samples, code) = input
        .split_once("\n\n\n\n")
        .expect("Failed to separate examples from program");
    let mapping =
        create_mapping(samples.split("\n\n")).expect("Mapping could not be generated from input");
    println!(
        "{}",
        execute_with(mapping, code.lines()).expect("Program should be parseable")[0]
    );
}

impl std::str::FromStr for SampleOutcome {
    type Err = MapResolveFailure;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        use MapResolveFailure::{FormattingError, IntParseFailure};

        // This is nasty. I wish I had something like scanf, despite it's known flaws
        let before: Regs = lines
            .next()
            .and_then(|line| line.strip_suffix(']'))
            .and_then(|line| line.strip_prefix("Before: ["))
            .map(|line| line.split(", "))
            .ok_or_else(|| FormattingError(s.into()))?
            .map(|n| n.parse().map_err(|e| IntParseFailure(s.into(), e)))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| FormattingError(s.into()))?;
        let instruction: [usize; 4] = lines
            .next()
            .ok_or_else(|| FormattingError(s.into()))?
            .split_whitespace()
            .map(|n| n.parse().map_err(|e| IntParseFailure(s.into(), e)))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| FormattingError(s.into()))?;
        let after: Regs = lines
            .next()
            .and_then(|line| line.strip_suffix(']'))
            .and_then(|line| line.strip_prefix("After:  ["))
            .map(|line| line.split(", "))
            .ok_or_else(|| FormattingError(s.into()))?
            .map(|n| n.parse().map_err(|e| IntParseFailure(s.into(), e)))
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|_| FormattingError(s.into()))?;
        Ok(Self {
            before,
            instruction,
            after,
        })
    }
}

#[derive(Debug)]
enum MapResolveFailure {
    FormattingError(#[allow(unused)] Box<str>),
    IntParseFailure(#[allow(unused)] Box<str>, #[allow(unused)] ParseIntError),
    NoPossibleResolution(#[allow(unused)] usize),
}

#[derive(Debug)]
enum ProgramParseError {
    BadInstructionSize(#[allow(unused)] Vec<usize>),
    IntParseFailure(#[allow(unused)] ParseIntError),
}

impl From<ParseIntError> for ProgramParseError {
    fn from(e: ParseIntError) -> Self {
        Self::IntParseFailure(e)
    }
}

// failure case for TryFrom<Vec<usize>> for [usize; 4]
impl From<Vec<usize>> for ProgramParseError {
    fn from(e: Vec<usize>) -> ProgramParseError {
        Self::BadInstructionSize(e)
    }
}
