// SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

//! Module providing an Intcode interpreter, which can be constructed with [`Interpreter::new`].
//!
//! # Example
//! ```rust
//! use intcode::{Interpreter, State};
//! let mut interpreter = Interpreter::new(vec![104, 1024, 99]);
//!
//! assert_eq!(
//!     interpreter.run_through_inputs(std::iter::empty()).unwrap(),
//!     (vec![1024], State::Halted)
//! );
//! ```

#![cfg_attr(
    aoc_direct,
    allow(dead_code, reason = "file written as standalone crate")
)]

use std::fmt;
use std::num::TryFromIntError;

/// A sort of logical memory management unit, using a hashmap to split memory into segments, which
/// are each contiguous in memory.
mod mmu {
    use std::collections::{BTreeMap, BTreeSet, HashMap};
    use std::fmt;

    // Using 512 to match page size of 4096 bytes on Linux
    const _: () = {
        assert!(std::mem::size_of::<[i64; 512]>() == 4096);
    };

    pub(super) struct IntcodeMem {
        segments: HashMap<u64, [i64; 512]>,
    }

    impl IntcodeMem {
        pub(super) fn get(&self, i: u64) -> i64 {
            if self.segments.contains_key(&(i & !511)) {
                self[i]
            } else {
                0
            }
        }
        fn active_segments(&self) -> BTreeSet<u64> {
            self.segments
                .iter()
                .filter_map(|(&k, &v)| if v == [0; 512] { Some(k) } else { None })
                .collect()
        }
    }

    impl PartialEq for IntcodeMem {
        fn eq(&self, other: &Self) -> bool {
            let active_segments = self.active_segments();
            other.active_segments() == active_segments
                && active_segments
                    .into_iter()
                    .all(|seg| self.segments[&seg] == other.segments[&seg])
        }
    }

    impl std::iter::FromIterator<i64> for IntcodeMem {
        fn from_iter<I: IntoIterator<Item = i64>>(iter: I) -> Self {
            let holder: Vec<i64> = iter.into_iter().collect();
            let mut page_number = 0;
            let mut segments = HashMap::with_capacity(holder.len().div_ceil(512));
            let (full_segments, partial_segment) = holder.as_chunks();
            for segment in full_segments {
                segments.insert(page_number, *segment);
                page_number += 512;
            }
            let mut final_segment = [0; 512];
            final_segment[..partial_segment.len()].clone_from_slice(partial_segment);
            segments.insert(page_number, final_segment);

            Self { segments }
        }
    }

    impl std::ops::Index<u64> for IntcodeMem {
        type Output = i64;
        fn index(&self, i: u64) -> &i64 {
            &self.segments[&(i & !511)][i as usize & 511]
        }
    }

    impl std::ops::IndexMut<u64> for IntcodeMem {
        fn index_mut(&mut self, i: u64) -> &mut i64 {
            let segment_index = i as usize & 511;
            &mut self.segments.entry(i & !511).or_insert_with(|| [0; 512])[segment_index]
        }
    }

    impl Clone for IntcodeMem {
        fn clone(&self) -> Self {
            // don't copy blank pages
            let segments = self
                .segments
                .iter()
                .filter(|(_, mem)| mem[..] != [0_i64; 512])
                .map(|(k, v)| (*k, *v))
                .collect();
            Self { segments }
        }
    }

    pub(super) struct IntcodeMemIter {
        segments: BTreeMap<u64, [i64; 512]>,
        current_segment: u64,
        segment_index: usize,
    }

    impl Iterator for IntcodeMemIter {
        type Item = i64;
        fn next(&mut self) -> Option<i64> {
            if self.current_segment > self.segments.keys().max().cloned().unwrap_or_default() {
                return None;
            }
            let ret: i64;
            if let Some(segment) = self.segments.get(&self.current_segment) {
                ret = segment[self.segment_index];
            } else {
                ret = 0;
            }

            self.segment_index += 1;
            if self.segment_index == 512 {
                self.segment_index = 0;
                self.current_segment += 512;
            }

            Some(ret)
        }
    }

    impl IntoIterator for IntcodeMem {
        type Item = i64;
        type IntoIter = IntcodeMemIter;
        fn into_iter(self) -> IntcodeMemIter {
            IntcodeMemIter {
                segments: self.segments.into_iter().collect(),
                current_segment: 0,
                segment_index: 0,
            }
        }
    }

    impl fmt::Debug for IntcodeMem {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut fmtstruct = fmt.debug_struct("IntcodeMem");
            let segment_nums: BTreeSet<_> = self.segments.keys().collect();
            for sn in segment_nums {
                fmtstruct.field(
                    &format!("{{ segment 0x{sn:04x} }}"),
                    &format_args!("{:?}", self.segments[sn]),
                );
            }

            fmtstruct.finish()
        }
    }
}

use mmu::IntcodeMem;
use std::io;

#[derive(Debug, PartialEq)]
pub enum State {
    Awaiting,
    Halted,
}

#[derive(Debug, PartialEq)]
pub enum ErrorState {
    UnrecognizedOpcode(i64),
    UnknownMode(i64),
    NegativeMemAccess(TryFromIntError),
    WriteToImmediate(i64),
    LoggerFailed(io::ErrorKind),
}

pub struct Interpreter<'a> {
    index: u64,
    rel_offset: i64,
    code: IntcodeMem,
    logger: Option<&'a mut dyn io::Write>,
}

impl PartialEq for Interpreter<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.rel_offset == other.rel_offset && self.code == other.code
    }
}

impl Clone for Interpreter<'_> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            rel_offset: self.rel_offset,
            code: self.code.clone(),
            logger: None,
        }
    }
}

impl fmt::Debug for Interpreter<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("Interpreter")
            .field("code", &self.code)
            .field("rbo", &self.rel_offset)
            .field("ip", &self.index)
            .field(
                "logger as *const _",
                &if let Some(ref logger) = self.logger {
                    logger as *const _
                } else {
                    std::ptr::null()
                },
            )
            .finish()
    }
}

/// Parameter mode for Intcode instruction
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ParamMode {
    Positional,
    Immediate,
    Relative,
}

impl fmt::Display for ParamMode {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParamMode::Positional => write!(fmt, "p"),
            ParamMode::Relative => write!(fmt, "r"),
            ParamMode::Immediate => Ok(()),
        }
    }
}

impl From<TryFromIntError> for ErrorState {
    fn from(err: TryFromIntError) -> Self {
        Self::NegativeMemAccess(err)
    }
}
impl From<io::Error> for ErrorState {
    fn from(err: io::Error) -> Self {
        Self::LoggerFailed(err.kind())
    }
}

impl TryFrom<i64> for ParamMode {
    type Error = ErrorState;
    fn try_from(i: i64) -> Result<Self, Self::Error> {
        match i {
            0 => Ok(ParamMode::Positional),
            1 => Ok(ParamMode::Immediate),
            2 => Ok(ParamMode::Relative),
            _ => Err(Self::Error::UnknownMode(i)),
        }
    }
}

impl<'a> Interpreter<'a> {
    pub fn log_with(&mut self, logger: &'a mut dyn io::Write) {
        self.logger = Some(logger);
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum OpCode {
    Add = 1,
    Mul = 2,
    In = 3,
    Out = 4,
    Jnz = 5,
    Jz = 6,
    Lt = 7,
    Eq = 8,
    Rbo = 9,
    Halt = 99,
}

impl Interpreter<'_> {
    fn param_val(&mut self, param: u64, mode: ParamMode) -> Result<i64, ErrorState> {
        match mode {
            ParamMode::Positional => {
                let i = self.code.get(param).try_into()?;
                Ok(self.code.get(i))
            }
            ParamMode::Immediate => Ok(self.code.get(param)),
            ParamMode::Relative => {
                let i = (self.code.get(param) + self.rel_offset).try_into()?;
                Ok(self.code.get(i))
            }
        }
    }

    fn parse_op(op: i64) -> Result<(OpCode, [ParamMode; 3]), ErrorState> {
        let modes: [ParamMode; 3] = [
            ((op / 100) % 10).try_into()?,  // C (hundreds place)
            ((op / 1000) % 10).try_into()?, // B (thousands place)
            (op / 10000).try_into()?,       // A (ten thousands place)
        ];
        match op % 100 {
            ..-99 | 100.. => unreachable!("modulo makes this impossible"),
            -99..=0 | 10..99 => Err(ErrorState::UnrecognizedOpcode(op % 100)),
            1 => Ok((OpCode::Add, modes)),
            2 => Ok((OpCode::Mul, modes)),
            3 => Ok((OpCode::In, modes)),
            4 => Ok((OpCode::Out, modes)),
            5 => Ok((OpCode::Jnz, modes)),
            6 => Ok((OpCode::Jz, modes)),
            7 => Ok((OpCode::Lt, modes)),
            8 => Ok((OpCode::Eq, modes)),
            9 => Ok((OpCode::Rbo, modes)),
            99 => Ok((OpCode::Halt, modes)),
        }
    }

    /// Manually set a memory location
    pub fn mem_override(&mut self, location: u64, value: i64) {
        self.code[location] = value;
    }

    fn exec_instruction(
        &mut self,
        inputs: &mut Option<i64>,
        outputs: &mut Vec<i64>,
    ) -> Result<Option<State>, ErrorState> {
        // Given a 5 digit number, digits ABCDE are used as follows:
        // DE is the two-digit opcode
        // C is the 1st parameter's mode
        // B is the 2nd parameter's mode
        // A is the 3rd parameter's mode
        //
        // So *0*1202 would be parsed as follows:
        //
        // Opcode 02 is multiply
        // C=2: 1st parameter is in relative mode
        // B=1: 2nd parameter is in immediate mode
        // A=0: 3rd parameter is in positional mode (the only valid mode for out parameters)

        let instruction = self.code.get(self.index);
        // Ensure that instruction is in range - not strictly needed, so only a debug_assert
        debug_assert!((0..100_000).contains(&instruction));

        let opcode = instruction % 100;
        let modes: [ParamMode; 3] = [
            ((instruction / 100) % 10).try_into()?,  // C (hundreds place)
            ((instruction / 1000) % 10).try_into()?, // B (thousands place)
            (instruction / 10000).try_into()?,       // A (ten thousands place)
        ];

        /// Shorthand to get the `$n`th parameter's value
        macro_rules! select_by_mode {
            ($n: literal) => {{
                self.param_val(self.index + $n, modes[$n - 1])?
            }};
        }

        /// Resolves to the destination address pointed to by the `$n`th parameter
        macro_rules! dest {
            ($n: literal) => {{
                match modes[$n - 1] {
                    ParamMode::Positional => u64::try_from(self.code.get(self.index + $n))?,
                    ParamMode::Immediate => {
                        return Err(ErrorState::WriteToImmediate(self.code.get(self.index + $n)))
                    }
                    ParamMode::Relative => {
                        u64::try_from(self.rel_offset + self.code.get(self.index + $n))?
                    }
                }
            }};
        }

        macro_rules! set_val {
            ($dest: expr, $new_val: expr) => {{
                let val: i64 = $new_val;
                let dest: u64 = $dest;
                self.code[dest] = val;
            }};
        }

        /// A comparison instruction
        macro_rules! comp {
            ($op: expr) => {{
                if $op {
                    1
                } else {
                    0
                }
            }};
        }

        macro_rules! report_op {
            ($fmt: literal) => {
                if let Some(ref mut logger) = self.logger {
                    write!(logger, "ip: {:>8} | rbo: {:>5} | ", self.index, self.rel_offset)?;
                    writeln!(logger, $fmt)?;
                }
            };
            ($fmt: literal, $($args:tt)*) => {
                if let Some(ref mut logger) = self.logger {
                    write!(logger, "ip: {:>8} | rbo: {:>5} | ", self.index, self.rel_offset)?;
                    logger.write_fmt(format_args!($fmt, $($args)*))?;
                    writeln!(logger)?;
                }
            }
        }

        macro_rules! report_op4 {
            ($name: literal) => {
                report_op!(
                    "{instruction:05} [{}({}{}, {}{}, {}{})]",
                    $name,
                    modes[0],
                    self.code.get(self.index + 1),
                    modes[1],
                    self.code.get(self.index + 2),
                    modes[2],
                    self.code.get(self.index + 3)
                )
            };
        }

        macro_rules! report_op3 {
            ($name: literal) => {
                report_op!(
                    "{instruction:05} [{}({}{}, {}{})]",
                    $name,
                    modes[0],
                    self.code.get(self.index + 1),
                    modes[1],
                    self.code.get(self.index + 2),
                )
            };
        }

        macro_rules! report_op2 {
            ($name: literal) => {
                report_op!(
                    "{instruction:05} [{}({}{})]",
                    $name,
                    modes[0],
                    self.code.get(self.index + 1),
                )
            };
        }

        match opcode {
            1 => {
                // add
                report_op4!("add");
                set_val!(dest!(3), select_by_mode!(1) + select_by_mode!(2));
                self.index += 4;
                Ok(None)
            }
            2 => {
                // multiply
                report_op4!("mul");
                set_val!(dest!(3), select_by_mode!(1) * select_by_mode!(2));
                self.index += 4;
                Ok(None)
            }
            3 => {
                // input
                if let Some(input) = inputs.take() {
                    report_op2!("input");
                    set_val!(dest!(1), input);
                    self.index += 2;
                    Ok(None)
                } else {
                    Ok(Some(State::Awaiting))
                }
            }
            4 => {
                report_op2!("output");
                // output
                outputs.push(select_by_mode!(1));
                self.index += 2;
                Ok(None)
            }
            5 => {
                report_op3!("jnz");
                // jump-if-true
                if select_by_mode!(1) == 0 {
                    self.index += 3;
                    Ok(None)
                } else {
                    self.index = select_by_mode!(2).try_into()?;
                    Ok(None)
                }
            }
            6 => {
                report_op3!("jz");
                // jump-if-false
                if select_by_mode!(1) != 0 {
                    self.index += 3;
                    Ok(None)
                } else {
                    self.index = select_by_mode!(2).try_into()?;
                    Ok(None)
                }
            }
            7 => {
                report_op4!("lt");
                // less than
                set_val!(dest!(3), comp!(select_by_mode!(1) < select_by_mode!(2)));
                self.index += 4;
                Ok(None)
            }
            8 => {
                // equals
                report_op4!("eq");
                set_val!(dest!(3), comp!(select_by_mode!(1) == select_by_mode!(2)));
                self.index += 4;
                Ok(None)
            }
            9 => {
                report_op2!("rbo");
                // relative base offset
                self.rel_offset += select_by_mode!(1);
                self.index += 2;
                Ok(None)
            }
            99 => {
                report_op!("{instruction:05} [halt]");
                Ok(Some(State::Halted))
            }
            i => Err(ErrorState::UnrecognizedOpcode(i)),
        }
    }

    /// Create a new interpreter. Collects `code` into the starting memory state.
    ///
    /// Panics if the number of entries exceeds `u64::MAX`
    pub fn new(code: impl IntoIterator<Item = i64>) -> Self {
        Self {
            index: 0,
            rel_offset: 0,
            logger: None,
            code: code.into_iter().collect(),
        }
    }

    /// Execute until either the program halts, or it tries to read nonexistent input.
    /// If the interpreter halted, returns `Ok(v)`, where `v` is a `Vec` of outputs, otherwise, it
    /// bubbles up the error
    pub fn run_through_inputs(
        &mut self,
        inputs: impl IntoIterator<Item = i64>,
    ) -> Result<(Vec<i64>, State), ErrorState> {
        let mut outputs = Vec::new();
        let mut inputs = inputs.into_iter();
        let mut current_input = None;
        loop {
            if current_input.is_none() {
                current_input = inputs.next();
            }
            match self.exec_instruction(&mut current_input, &mut outputs) {
                Ok(None) => (),
                Ok(Some(State::Halted)) => break Ok((outputs, State::Halted)),
                Ok(Some(State::Awaiting)) => break Ok((outputs, State::Awaiting)),
                Err(e) => break Err(e),
            }
        }
    }

    /// Pre-compute as much as possible - that is, run every up to, but not including, the first
    /// In, Out, or Halt instruction, bubbling up any errors that occur.
    pub fn precompute(&mut self) -> Result<(), ErrorState> {
        while Self::parse_op(self.code[self.index])
            .is_ok_and(|(opcode, _)| !matches!(opcode, OpCode::In | OpCode::Out | OpCode::Halt))
        {
            self.exec_instruction(&mut None, &mut Vec::with_capacity(0))?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    /// Example program from day 9, which takes no input and outputs its own code
    #[test]
    fn quine() {
        let quine_code = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut interpreter = Interpreter::new(quine_code.clone());
        let (outputs, State::Halted) = interpreter.run_through_inputs(Vec::new()).unwrap() else {
            panic!()
        };
        assert_eq!(quine_code, outputs);
    }

    /// Example program from day 9, which "should output a 16-digit number"
    #[test]
    fn output_sixteen_digit() {
        let mut interpreter = Interpreter::new([1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        let (outputs, State::Halted) = interpreter.run_through_inputs(Vec::new()).unwrap() else {
            panic!()
        };
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].to_string().len(), 16);
    }

    /// Example program from day 9, which "should output the large number in the middle"
    #[test]
    fn large_number() {
        let mut interpreter = Interpreter::new([104, 1125899906842624, 99]);
        let (outputs, State::Halted) = interpreter.run_through_inputs(Vec::new()).unwrap() else {
            panic!()
        };
        assert_eq!(outputs, vec![1125899906842624]);
    }

    /// Ensure that failure due to missing input leaves the interpreter in a sane state that can
    /// be recovered from
    #[test]
    fn missing_input_recoverable() {
        let mut interpreter = Interpreter::new(vec![3, 10, 4, 10, 99]);
        let old_state = interpreter.clone();

        let failed_run = interpreter.run_through_inputs(Vec::new());

        // make sure that the failure returned the right ErrorState and left both `outputs` and
        // `interpreter` unchanged
        assert_eq!(failed_run, Ok((vec![], State::Awaiting)));
        assert_eq!(interpreter, old_state);

        // make sure that interpreter can still be used
        assert_eq!(
            interpreter.run_through_inputs(vec![1].into_iter()),
            Ok((vec![1], State::Halted))
        );
    }
}
