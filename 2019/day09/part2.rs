// SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 9 Part 2

use std::collections::HashMap;
use std::num::TryFromIntError;

#[derive(Debug, PartialEq)]
enum State {
    Running,
    Halted,
}

#[derive(Debug, PartialEq)]
enum ErrorState {
    UnrecognizedOpcode(i64),
    UnknownMode(i64),
    NegativeMemAccess(TryFromIntError),
    WriteToImmediate(i64),
    MissingInput,
}

#[derive(Debug, PartialEq)]
struct Interpreter {
    index: u64,
    rel_offset: i64,
    code: HashMap<u64, i64>,
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

    fn exec_instruction(
        &mut self,
        inputs: &mut impl Iterator<Item = i64>,
        outputs: &mut Vec<i64>,
    ) -> Result<State, ErrorState> {
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
            ($n: literal) => {{ self.param_val(self.index + $n, modes[$n - 1])? }};
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
                        ));
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
            ($op: expr) => {{ if $op { 1 } else { 0 } }};
        }

        macro_rules! report_op {
            {$block: expr} => {
                #[cfg(debug_assertions)] $block;
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
                Ok(State::Running)
            }
            2 => {
                // multiply
                report_op4!("mul");
                set_val!(dest!(3), select_by_mode!(1) * select_by_mode!(2));
                self.index += 4;
                Ok(State::Running)
            }
            3 => {
                // input
                report_op2!("input");
                if let Some(input) = inputs.next() {
                    set_val!(dest!(1), input);
                    self.index += 2;
                    Ok(State::Running)
                } else {
                    Err(ErrorState::MissingInput)
                }
            }
            4 => {
                report_op2!("output");
                // output
                outputs.push(select_by_mode!(1));
                self.index += 2;
                Ok(State::Running)
            }
            5 => {
                report_op3!("jnz");
                // jump-if-true
                if select_by_mode!(1) == 0 {
                    self.index += 3;
                    Ok(State::Running)
                } else {
                    self.index = select_by_mode!(2).try_into()?;
                    Ok(State::Running)
                }
            }
            6 => {
                report_op3!("jz");
                // jump-if-false
                if select_by_mode!(1) != 0 {
                    self.index += 3;
                    Ok(State::Running)
                } else {
                    self.index = select_by_mode!(2).try_into()?;
                    Ok(State::Running)
                }
            }
            7 => {
                report_op4!("lt");
                // less than
                set_val!(dest!(3), comp!(select_by_mode!(1) < select_by_mode!(2)));
                self.index += 4;
                Ok(State::Running)
            }
            8 => {
                // equals
                report_op4!("eq");
                set_val!(dest!(3), comp!(select_by_mode!(1) == select_by_mode!(2)));
                self.index += 4;
                Ok(State::Running)
            }
            9 => {
                report_op2!("rbo");
                // relative base offset
                self.rel_offset += select_by_mode!(1);
                self.index += 2;
                Ok(State::Running)
            }
            99 => {
                report_op! { eprintln!("{instruction:09} [halt]") }
                Ok(State::Halted)
            }
            i => Err(ErrorState::UnrecognizedOpcode(i)),
        }
    }
    fn new(code: impl IntoIterator<Item = i64>) -> Result<Self, TryFromIntError> {
        let code = code
            .into_iter()
            .enumerate()
            .map(|(i, v)| (u64::try_from(i).map(|k| (k, v))))
            .collect::<Result<_, _>>()?;

        Ok(Self {
            index: 0,
            rel_offset: 0,
            code,
        })
    }

    /// Execute to completion - useful if inputs won't depend on outputs.
    /// If the interpreter halted, returns `Ok(v)`, where `v` is a `Vec` of outputs, otherwise, it
    /// bubbles up the error
    fn run_to_completion(
        &mut self,
        mut inputs: impl Iterator<Item = i64>,
    ) -> Result<Vec<i64>, ErrorState> {
        let mut outputs = Vec::new();
        loop {
            if self.exec_instruction(&mut inputs, &mut outputs)? == State::Halted {
                break Ok(outputs);
            }
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let mut interpreter = Interpreter::new(
        read_to_string(args().nth(1).as_deref().unwrap_or("input"))
            .expect("Failed to read file!")
            .trim()
            .split(",")
            .map(|s| s.parse().expect("Could not parse i64")),
    )
    .unwrap();
    let outputs = interpreter.run_to_completion(vec![2].into_iter()).unwrap();
    assert_eq!(
        outputs.len(),
        1,
        "Didn't have exactly 1 output: {outputs:?}"
    );
    println!("{}", outputs[0]);
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
        let mut interpreter = Interpreter::new(quine_code.clone()).unwrap();
        let outputs = interpreter
            .run_to_completion(Vec::new().into_iter())
            .unwrap();
        assert_eq!(quine_code, outputs);
    }

    /// Example program from day 9, which "should output a 16-digit number"
    #[test]
    fn output_sixteen_digit() {
        let mut interpreter = Interpreter::new([1102, 34915192, 34915192, 7, 4, 7, 99, 0]).unwrap();
        let outputs = interpreter
            .run_to_completion(Vec::new().into_iter())
            .unwrap();
        assert_eq!(outputs.len(), 1);
        assert_eq!(outputs[0].to_string().len(), 16);
    }

    /// Example program from day 9, which "should output the large number in the middle"
    #[test]
    fn large_number() {
        let mut interpreter = Interpreter::new([104, 1125899906842624, 99]).unwrap();
        let outputs = interpreter
            .run_to_completion(Vec::new().into_iter())
            .unwrap();
        assert_eq!(outputs, vec![1125899906842624]);
    }
}
