// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 21 Part 2

use std::collections::HashMap;
use std::ops;
use std::rc::Rc;

macro_rules! debug_eprintln {
    () => {
        if cfg!(debug_assertions) {
            eprintln!();
        }
    };
    ($fmt: literal) => {
        if cfg!(debug_assertions) {
            eprintln!($fmt);
        }
    };
    ($fmt: literal, $($toks: expr),+) => {
        if cfg!(debug_assertions) {
            eprintln!($fmt, $($toks,)+);
        }
    };
}

#[derive(Debug, PartialEq, Clone)]
enum RawMonkeyNumber<'a> {
    Literal(i64),
    Resolved(Resolver),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn resolve_monkey<'a>(
    monkey: &'a str,
    monkeys: &mut HashMap<&'a str, RawMonkeyNumber<'a>>,
) -> Resolver {
    if monkey == "humn" {
        return Resolver::new(ResolvedMonkeyNumber::Humn);
    }
    let resolved = match RawMonkeyNumber::clone(&monkeys[monkey]) {
        RawMonkeyNumber::Resolved(n) => return Resolver::clone(&n),
        RawMonkeyNumber::Literal(n) => Resolver::new(ResolvedMonkeyNumber::KnownValue(n)),
        RawMonkeyNumber::Add(a, b) => &resolve_monkey(a, monkeys) + &resolve_monkey(b, monkeys),
        RawMonkeyNumber::Sub(a, b) => &resolve_monkey(a, monkeys) - &resolve_monkey(b, monkeys),
        RawMonkeyNumber::Mul(a, b) => &resolve_monkey(a, monkeys) * &resolve_monkey(b, monkeys),
        RawMonkeyNumber::Div(a, b) => &resolve_monkey(a, monkeys) / &resolve_monkey(b, monkeys),
    };
    monkeys.insert(
        monkey,
        RawMonkeyNumber::Resolved(Resolver::clone(&resolved)),
    );
    resolved
}

#[derive(Debug, PartialEq, Clone)]
struct Resolver(Rc<ResolvedMonkeyNumber>);
impl Resolver {
    fn new(n: ResolvedMonkeyNumber) -> Self {
        Self(Rc::new(n))
    }
}

macro_rules! impl_op {
    {$op_name: ident, $op_fn: ident, $op: tt} => {
        impl ops::$op_name<&Resolver> for &Resolver {
            type Output = Resolver;
            fn $op_fn(self, rhs: &Resolver) -> Self::Output {
                if let &ResolvedMonkeyNumber::KnownValue(a) = self.0.as_ref()
                    && let &ResolvedMonkeyNumber::KnownValue(b) = rhs.0.as_ref()
                {
                    Resolver::new(ResolvedMonkeyNumber::KnownValue(a $op b))
                } else {
                    Resolver::new(ResolvedMonkeyNumber::$op_name([self.0.clone(), rhs.0.clone()]))
                }
            }

        }
    }
}

impl_op! { Add, add, + }
impl_op! { Sub, sub, - }
impl_op! { Mul, mul, * }
impl_op! { Div, div, / }

#[derive(Debug, PartialEq, Clone)]
enum ResolvedMonkeyNumber {
    Humn,
    KnownValue(i64),
    Add([Rc<ResolvedMonkeyNumber>; 2]),
    Sub([Rc<ResolvedMonkeyNumber>; 2]),
    Mul([Rc<ResolvedMonkeyNumber>; 2]),
    Div([Rc<ResolvedMonkeyNumber>; 2]),
}

impl ResolvedMonkeyNumber {
    fn s_expr(&self) -> String {
        format!("{self}")
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut monkeys = HashMap::new();
    let mut equality = None::<[&str; 2]>;
    for line in input.lines() {
        let (monkey, job) = line.split_once(": ").expect("proper separator");
        if monkey == "humn" {
            continue;
        }
        if monkey == "root" {
            equality = Some([&job[0..4], &job[7..11]]);
        } else if let Ok(n) = job.parse() {
            monkeys.insert(monkey, RawMonkeyNumber::Literal(n));
        } else {
            let a = &job[0..4];
            let b = &job[7..11];
            match &job[5..6] {
                "+" => {
                    monkeys.insert(monkey, RawMonkeyNumber::Add(a, b));
                }
                "-" => {
                    monkeys.insert(monkey, RawMonkeyNumber::Sub(a, b));
                }
                "*" => {
                    monkeys.insert(monkey, RawMonkeyNumber::Mul(a, b));
                }
                "/" => {
                    monkeys.insert(monkey, RawMonkeyNumber::Div(a, b));
                }
                s => panic!("bad operation: {s:?}"),
            }
        }
    }
    let mut verifier = monkeys.clone();
    let Some([a, b]) = equality else {
        panic!("no root provided")
    };
    let eq_a = resolve_monkey(a, &mut monkeys);
    let eq_b = resolve_monkey(b, &mut monkeys);
    assert!(matches!(
        (eq_a.0.as_ref(), eq_b.0.as_ref()),
        (ResolvedMonkeyNumber::KnownValue(_), _) | (_, ResolvedMonkeyNumber::KnownValue(_))
    ));
    let (mut equation, mut val) = match (eq_a.0.as_ref(), eq_b.0.as_ref()) {
        (ResolvedMonkeyNumber::KnownValue(val), equation)
        | (equation, ResolvedMonkeyNumber::KnownValue(val)) => {
            assert!(
                !matches!(equation, ResolvedMonkeyNumber::KnownValue(_)),
                "2 known values - nothing to do"
            );
            (ResolvedMonkeyNumber::clone(equation), *val)
        }
        _ => panic!("neither side of the equation could be resolved"),
    };

    debug_eprintln!("(eq {} {val})", equation.s_expr());
    while equation != ResolvedMonkeyNumber::Humn {
        use ResolvedMonkeyNumber::KnownValue as K;
        match equation {
            ResolvedMonkeyNumber::Humn | ResolvedMonkeyNumber::KnownValue(_) => unreachable!(),
            ResolvedMonkeyNumber::Add([a, b]) => {
                if let K(n) = *a {
                    val -= n;
                    equation = Rc::unwrap_or_clone(b);
                } else if let K(n) = *b {
                    val -= n;
                    equation = Rc::unwrap_or_clone(a);
                } else {
                    panic!(
                        "Could not solve (eq (add {a} {b}) {val})"
                    );
                }
            }
            ResolvedMonkeyNumber::Sub([a, b]) => {
                if let K(n) = *b {
                    val += n;
                    equation = Rc::unwrap_or_clone(a);
                } else if let K(n) = *a {
                    val = - val + n;
                    equation = Rc::unwrap_or_clone(b);
                } else {
                    panic!(
                        "Could not solve (eq (sub {} {}) {val})",
                        a.s_expr(),
                        b.s_expr()
                    );
                }
            }
            ResolvedMonkeyNumber::Mul([a, b]) => {
                if let K(n) = *a {
                    assert!(val % n == 0);
                    val /= n;
                    equation = Rc::unwrap_or_clone(b);
                } else if let K(n) = *b {
                    assert!(val % n == 0);
                    val /= n;
                    equation = Rc::unwrap_or_clone(a);
                } else {
                    panic!(
                        "Could not solve (eq (mul {a} {b}) {val})"
                    );
                }
            }
            ResolvedMonkeyNumber::Div([a, b]) => {
                if let K(n) = *b {
                    val *= n;
                    equation = Rc::unwrap_or_clone(a);
                } else {
                    panic!("Could not solve (eq (div {a} {b}) {val})");
                }
            }
        }
        debug_eprintln!("(eq {} {val})", equation.s_expr());
    }
    verifier.insert("humn", RawMonkeyNumber::Literal(val));
    assert_eq!(
        part1_resolve_monkey(a, &mut verifier),
        part1_resolve_monkey(b, &mut verifier)
    );

    println!("{val}");
}

fn part1_resolve_monkey<'a>(
    monkey: &'a str,
    monkeys: &mut HashMap<&'a str, RawMonkeyNumber<'a>>,
) -> i64 {
    let number = match monkeys[monkey] {
        RawMonkeyNumber::Literal(n) => return n,
        RawMonkeyNumber::Add(a, b) => {
            part1_resolve_monkey(a, monkeys) + part1_resolve_monkey(b, monkeys)
        }
        RawMonkeyNumber::Sub(a, b) => {
            part1_resolve_monkey(a, monkeys) - part1_resolve_monkey(b, monkeys)
        }
        RawMonkeyNumber::Mul(a, b) => {
            part1_resolve_monkey(a, monkeys) * part1_resolve_monkey(b, monkeys)
        }
        RawMonkeyNumber::Div(a, b) => {
            part1_resolve_monkey(a, monkeys) / part1_resolve_monkey(b, monkeys)
        }
        _ => panic!("Shouldn't have resolved monkeys here!"),
    };
    monkeys.insert(monkey, RawMonkeyNumber::Literal(number));
    number
}

// display the ResolvedMonkeyNumber as an S-Expression
impl std::fmt::Display for ResolvedMonkeyNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolvedMonkeyNumber::Humn => write!(f, "humn"),
            ResolvedMonkeyNumber::KnownValue(n) => write!(f, "{n}"),
            ResolvedMonkeyNumber::Add(boxed) => {
                write!(f, "(add {} {})", boxed[0].s_expr(), boxed[1].s_expr())
            }
            ResolvedMonkeyNumber::Sub(boxed) => {
                write!(f, "(sub {} {})", boxed[0].s_expr(), boxed[1].s_expr())
            }
            ResolvedMonkeyNumber::Mul(boxed) => {
                write!(f, "(mul {} {})", boxed[0].s_expr(), boxed[1].s_expr())
            }
            ResolvedMonkeyNumber::Div(boxed) => {
                write!(f, "(div {} {})", boxed[0].s_expr(), boxed[1].s_expr())
            }
        }
    }
}
