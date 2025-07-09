// SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

//! Module providing an Intcode interpreter, which can be constructed with [`Interpreter::new`].
//!
//! Currently uses a [`BTreeMap`] to store the code, so that non-contiguous memory can be stored,
//! though that's subject to change.
//!
//! # Example
//! ```rust
//! let mut interpreter = Interpreter::new(vec![104, 1024, 99]);
//!
//! assert_eq!(interpreter.run_through_inputs(Vec::new()), vec![1024]);
//! ```

use std::collections::BTreeMap;
use std::num::TryFromIntError;

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
}

#[derive(Debug, PartialEq, Clone)]
pub struct Interpreter {
    index: u64,
    rel_offset: i64,
    code: BTreeMap<u64, i64>,
    log: bool,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ParamMode {
    Positional,
    Immediate,
    Relative,
}

impl std::fmt::Display for ParamMode {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "{}",
            match self {
                ParamMode::Positional => 'p',
                ParamMode::Immediate => 'i',
                ParamMode::Relative => 'r',
            }
        )
    }
}

impl From<TryFromIntError> for ErrorState {
    fn from(err: TryFromIntError) -> Self {
        Self::NegativeMemAccess(err)
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

impl Interpreter {
    fn param_val(&mut self, param: u64, mode: ParamMode) -> Result<i64, ErrorState> {
        match mode {
            ParamMode::Positional => {
                let i = (*self.code.entry(param).or_insert(0)).try_into()?;
                Ok(*self.code.entry(i).or_insert(0))
            }
            ParamMode::Immediate => Ok(*self.code.entry(param).or_insert(0)),
            ParamMode::Relative => {
                let i = (*self.code.entry(param).or_insert(0) + self.rel_offset).try_into()?;
                Ok(*self.code.entry(i).or_insert(0))
            }
        }
    }

    #[allow(dead_code, reason = "Logging may not always be appropriate or needed")]
    pub fn enable_logging(&mut self) {
        self.log = true;
    }

    /// Manually set a memory location
    #[allow(dead_code, reason = "Not all days require overriding memory")]
    pub fn mem_override(&mut self, location: u64, value: i64) {
        self.code.insert(location, value);
    }

    fn exec_instruction(
        &mut self,
        inputs: &mut impl Iterator<Item = i64>,
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

        let instruction = *self.code.entry(self.index).or_insert(0);
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
                    ParamMode::Positional => {
                        u64::try_from(*self.code.entry(self.index + $n).or_insert(0))?
                    }
                    ParamMode::Immediate => {
                        return Err(ErrorState::WriteToImmediate(
                            self.code.get(&(self.index + $n)).copied().unwrap_or(0),
                        ))
                    }
                    ParamMode::Relative => u64::try_from(
                        self.rel_offset + *self.code.entry(self.index + $n).or_insert(0),
                    )?,
                }
            }};
        }

        macro_rules! set_val {
            ($dest: expr, $new_val: expr) => {{
                let val: i64 = $new_val;
                let dest: u64 = $dest;
                self.code
                    .entry(dest)
                    .and_modify(|e| *e = val)
                    .or_insert(val);
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
            {$block: expr} => {
                if self.log {$block};
            }
        }
        macro_rules! report_op4 {
            ($name: literal) => {
                report_op! {
                    eprintln!(
                        "{instruction:05} [{}({}{}, {}{}, {}{})]",
                        $name,
                        modes[0],
                        self.code.get(&(self.index + 1)).copied().unwrap_or(0),
                        modes[1],
                        self.code.get(&(self.index + 2)).copied().unwrap_or(0),
                        modes[2],
                        self.code.get(&(self.index + 3)).copied().unwrap_or(0)
                    )
                }
            };
        }

        macro_rules! report_op2 {
            ($name: literal) => {
                report_op! {
                    eprintln!(
                        "{instruction:05} [{}({}{})]",
                        $name,
                        modes[0],
                        self.code.get(&(self.index + 1)).copied().unwrap_or(0),
                    )
                }
            };
        }

        macro_rules! report_op3 {
            ($name: literal) => {
                report_op! {
                    eprintln!(
                        "{instruction:05} [{}({}{}, {}{})]",
                        $name,
                        modes[0],
                        self.code.get(&(self.index + 1)).copied().unwrap_or(0),
                        modes[1],
                        self.code.get(&(self.index + 2)).copied().unwrap_or(0),
                    )
                }
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
                if let Some(input) = inputs.next() {
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
                report_op! { eprintln!("{instruction:05} [halt]") }
                Ok(Some(State::Halted))
            }
            i => Err(ErrorState::UnrecognizedOpcode(i)),
        }
    }

    /// Create a new interpreter. Collects `code` into the starting memory state.
    ///
    /// Panics if the number of entries exceeds `u64::MAX`
    pub fn new(code: impl IntoIterator<Item = i64>) -> Self {
        let code = code
            .into_iter()
            .enumerate()
            .map(|(i, v)| (u64::try_from(i).expect("code shorter than u64::MAX"), v))
            .collect();

        Self {
            index: 0,
            rel_offset: 0,
            log: false,
            code,
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
        loop {
            match self.exec_instruction(&mut inputs, &mut outputs) {
                Ok(None) => (),
                Ok(Some(State::Halted)) => break Ok((outputs, State::Halted)),
                Ok(Some(State::Awaiting)) => break Ok((outputs, State::Awaiting)),
                Err(e) => break Err(e),
            }
        }
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
