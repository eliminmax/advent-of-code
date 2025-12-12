// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 11 Part 2

use std::cell::RefCell;
use std::collections::VecDeque;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MonkeyOp {
    Plus(u64),
    Times(u64),
    Squared,
}

impl MonkeyOp {
    const fn apply(self, worry: u64) -> u64 {
        match self {
            MonkeyOp::Plus(n) => worry + n,
            MonkeyOp::Times(n) => worry * n,
            MonkeyOp::Squared => worry * worry,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MonkeyToss {
    target: usize,
    worry: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct MonkeyTest {
    divisor: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Debug)]
struct Monkey {
    inventory: RefCell<VecDeque<u64>>,
    inspect_count: RefCell<u64>,
    operation: MonkeyOp,
    test: MonkeyTest,
}

/// Make it more obvious that a block is being used to ensure that the contained operation should
/// be kept in a narrow scope, to drop locks as soon as no longer needed.
macro_rules! scoped {
    {$b: block} => {{ $b }};
}

impl Monkey {
    fn toss(&self, dividing_factor: u64) -> impl IntoIterator<Item = MonkeyToss> {
        let mut v = Vec::new();
        while let Some(item) = scoped!({ self.inventory.borrow_mut().pop_front() }) {
            scoped!({ *self.inspect_count.borrow_mut() += 1 });
            let worry = self.operation.apply(item) % dividing_factor;
            v.push(MonkeyToss {
                worry,
                target: if worry.is_multiple_of(self.test.divisor) {
                    self.test.if_true
                } else {
                    self.test.if_false
                },
            })
        }
        v
    }
}

fn run_round(monkeys: &[Monkey], dividing_factor: u64) {
    for monkey in monkeys {
        for MonkeyToss { worry, target } in monkey.toss(dividing_factor) {
            // Could one call this a scoped Monkey trial?
            scoped!({ monkeys[target].inventory.borrow_mut().push_back(worry) });
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
    let dividing_factor = monkeys
        .iter()
        .fold(1, |acc, monk| lcm(acc, monk.test.divisor));
    for _ in 0..10_000 {
        run_round(&monkeys, dividing_factor);
    }
    monkeys.sort_by_key(|m| *m.inspect_count.borrow());
    let monkey_business =
        monkeys.pop().unwrap().inspect_count.take() * monkeys.pop().unwrap().inspect_count.take();
    println!("{monkey_business}");
}

mod parse_impls {
    use super::*;
    #[derive(Debug)]
    pub enum MonkeyOpParseError {
        UnknownFormat(#[allow(dead_code)] Box<str>),
        InvalidOp(#[allow(dead_code)] Box<str>),
        IntParse(#[allow(dead_code)] ParseIntError),
    }

    #[derive(Debug)]
    pub enum MonkeyTestParseError {
        MissingField(#[allow(dead_code)] &'static str),
        MissingLiteral(#[allow(dead_code)] &'static str),
        MismatchedToken {
            #[allow(dead_code)]
            expected: &'static str,
            #[allow(dead_code)]
            got: Box<str>,
        },
        ExtraToken(#[allow(dead_code)] Box<str>),
        IntParse(#[allow(dead_code)] ParseIntError),
    }

    #[derive(Debug)]
    pub enum MonkeyParseError {
        UnknownFormat(#[allow(dead_code)] Box<str>),
        BadId(#[allow(dead_code)] ParseIntError),
        BadInventoryNum(#[allow(dead_code)] ParseIntError),
        BadOp(#[allow(dead_code)] MonkeyOpParseError),
        BadTest(#[allow(dead_code)] MonkeyTestParseError),
    }

    macro_rules! impl_into_variant{
    {$from: ty, $to: ty, $variant: ident} => {
        impl From<$from> for $to {
            fn from(e: $from) -> Self {
                <$to>::$variant(e)
            }
        }
    }
}

    impl_into_variant! { MonkeyTestParseError, MonkeyParseError, BadTest }
    impl_into_variant! { MonkeyOpParseError, MonkeyParseError, BadOp }
    impl_into_variant! { ParseIntError, MonkeyTestParseError, IntParse }
    impl_into_variant! { ParseIntError, MonkeyOpParseError, IntParse }

    impl FromStr for MonkeyTest {
        type Err = MonkeyTestParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut words = s.split_ascii_whitespace();
            macro_rules! consume_tokens {
            ($tok: literal) => {{
                let tok = words.next().ok_or(Self::Err::MissingLiteral($tok))?;
                if tok != $tok {
                    return Err(Self::Err::MismatchedToken {
                        expected: $tok,
                        got: Box::from(tok),
                    });
                }
            }};
            ($tok: literal $($remaining: literal)+) => {
                consume_tokens!($tok);
                consume_tokens!($($remaining)+);
            }
        }
            macro_rules! field {
                ($field: ident) => {{
                    words
                        .next()
                        .ok_or(Self::Err::MissingField(stringify!($field)))?
                        .parse()?
                }};
            }
            consume_tokens!("Test:" "divisible" "by");
            let divisor = field!(divisor);
            consume_tokens!("If" "true:" "throw" "to" "monkey");
            let if_true = field!(if_true);
            consume_tokens!("If" "false:" "throw" "to" "monkey");
            let if_false = field!(if_false);
            if let Some(tok) = words.next() {
                Err(Self::Err::ExtraToken(Box::from(tok)))
            } else {
                Ok(Self {
                    divisor,
                    if_true,
                    if_false,
                })
            }
        }
    }

    impl FromStr for MonkeyOp {
        type Err = MonkeyOpParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (op, rhs) = s
                .trim()
                .strip_prefix("Operation: new = old ")
                .and_then(|s| s.split_once(' '))
                .ok_or_else(|| Self::Err::UnknownFormat(Box::from(s)))?;
            match op {
                "+" => Ok(Self::Plus(rhs.parse()?)),
                "*" => {
                    if rhs == "old" {
                        Ok(Self::Squared)
                    } else {
                        Ok(Self::Times(rhs.parse()?))
                    }
                }
                badop => Err(Self::Err::InvalidOp(Box::from(badop))),
            }
        }
    }

    impl FromStr for Monkey {
        type Err = MonkeyParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let fmt_err = || Self::Err::UnknownFormat(Box::from(s));
            let (id, remaining) = s
                .strip_prefix("Monkey ")
                .and_then(|s| s.split_once(":\n"))
                .ok_or_else(fmt_err)?;
            let _ = u8::from_str(id).map_err(Self::Err::BadId)?;
            let remaining = remaining
                .trim()
                .strip_prefix("Starting items: ")
                .ok_or_else(fmt_err)?;
            let (inv, remaining) = remaining.split_once('\n').ok_or_else(fmt_err)?;
            let inventory = RefCell::new(
                inv.split(",")
                    .map(str::trim)
                    .map(str::parse)
                    .collect::<Result<_, _>>()
                    .map_err(Self::Err::BadInventoryNum)?,
            );
            let (operation, test) = remaining.split_once('\n').ok_or_else(fmt_err)?;
            let operation = operation.parse()?;
            let test = test.parse()?;

            Ok(Self {
                inventory,
                inspect_count: RefCell::new(0),
                operation,
                test,
            })
        }
    }
}

// Function copied from solution to 2019 day 12 part 2
/// Calculate the lowest common denominator of 2 `u64`s
fn lcm(a: u64, b: u64) -> u64 {
    let gcd = {
        let (mut a, mut b) = (a, b);
        // greatest common denominator algorithm adapted for `u64` from 2019 day 10
        while a != 0 {
            (a, b) = (b % a, a);
        }
        b
    };
    a * b / gcd
}
