// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 19 Part 2

mod intcode;
use intcode::Interpreter;

use std::collections::HashMap;
use std::ops::RangeInclusive;
#[derive(Debug, PartialEq, Copy, Clone)]
struct Span {
    start: i64,
    end: i64,
}

impl Span {
    fn overlap(self, other: Self) -> Option<Span> {
        if self.start > other.start {
            other.overlap(self)
        } else if self.end >= other.start {
            Some(Span {
                start: other.start,
                end: self.end.min(other.end),
            })
        } else {
            None
        }
    }

    fn len(&self) -> i64 {
        self.end - self.start + 1
    }
}

impl IntoIterator for Span {
    type Item = i64;
    type IntoIter = RangeInclusive<i64>;
    fn into_iter(self) -> RangeInclusive<i64> {
        self.start..=self.end
    }
}

struct CachingRunner {
    interpreter: Interpreter,
    row_spans: HashMap<i64, Span>,
    interpreter_cache: HashMap<i64, Interpreter>,
}

impl CachingRunner {
    fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            row_spans: HashMap::new(),
            interpreter_cache: HashMap::new(),
        }
    }

    fn get_interpreter(&mut self, x: i64) -> Interpreter {
        self.interpreter_cache
            .entry(x)
            .or_insert_with(|| {
                let mut interp = self.interpreter.clone();
                let (output, state) = interp.run_through_inputs([x]).unwrap();
                assert_eq!(state, intcode::State::Awaiting, "Finished early");
                assert!(output.is_empty(), "Early output");
                interp
            })
            .clone()
    }

    fn gen_row(&mut self, y: i64) -> Option<Span> {
        if let Some(row) = self.row_spans.get(&y).copied() {
            return Some(row);
        }

        let span = self
            .row_spans
            .get(&(y - 1))
            .copied()
            .map(|Span { start, end }| Span {
                start,
                end: end + 3,
            })
            .unwrap_or(Span { start: 0, end: 50 });

        // drop stale interpreter cache entries
        self.interpreter_cache.retain(|k, _| *k >= span.start);

        let mut start = None;

        for x in span {
            let (output, state) = self.get_interpreter(x).run_through_inputs([y]).unwrap();
            assert_eq!(state, intcode::State::Halted, "Still awaiting input");
            assert_eq!(output.len(), 1);
            if output[0] == 1 && start.is_none() {
                start = Some(x);
            } else if let Some(start) = start
                && output[0] == 0
            {
                let span = Span { start, end: x - 1 };
                self.row_spans.insert(y, span);
                return Some(span);
            }
        }
        None
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut interpreter = Interpreter::new(input.trim().split(",").map(|i| i.parse().unwrap()));
    let (output, intcode::State::Awaiting) =
        interpreter.run_through_inputs(std::iter::empty()).unwrap()
    else {
        panic!("Interpreter did not wait for input");
    };
    assert!(output.is_empty(), "Early output");

    let mut solver = CachingRunner::new(interpreter);

    // pre-compute the first 128 rows, because gen_row breaks if it's skipped
    for y in 0..128 {
        solver.gen_row(y);
    }

    'solve_loop: for y in 0.. {
        if let (Some(top), Some(bottom)) = (solver.gen_row(y), solver.gen_row(y + 99))
            && let Some(span) = top.overlap(bottom)
            && span.len() >= 100
        {
            println!("{}", (span.start * 10000) + y);
            break 'solve_loop;
        }
    }
}
