// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 21 Part 1

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum MonkeyNumber<'a> {
    Literal(i64),
    Resolved(i64),
    Add(&'a str, &'a str),
    Sub(&'a str, &'a str),
    Mul(&'a str, &'a str),
    Div(&'a str, &'a str),
}

fn resolve_monkey<'a>(monkey: &'a str, monkeys: &mut HashMap<&'a str, MonkeyNumber<'a>>) -> i64 {
    let number = match monkeys[monkey] {
        MonkeyNumber::Literal(n) | MonkeyNumber::Resolved(n) => return n,
        MonkeyNumber::Add(a, b) => resolve_monkey(a, monkeys) + resolve_monkey(b, monkeys),
        MonkeyNumber::Sub(a, b) => resolve_monkey(a, monkeys) - resolve_monkey(b, monkeys),
        MonkeyNumber::Mul(a, b) => resolve_monkey(a, monkeys) * resolve_monkey(b, monkeys),
        MonkeyNumber::Div(a, b) => resolve_monkey(a, monkeys) / resolve_monkey(b, monkeys),
    };
    monkeys.insert(monkey, MonkeyNumber::Resolved(number));
    number
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let (monkey, job) = line.split_once(": ").expect("proper separator");
        if let Ok(n) = job.parse() {
            monkeys.insert(monkey, MonkeyNumber::Literal(n));
        } else {
            let a = &job[0..4];
            let b = &job[7..11];
            match &job[5..6] {
                "+" => {
                    let _ = monkeys.insert(monkey, MonkeyNumber::Add(a, b));
                }
                "-" => {
                    let _ = monkeys.insert(monkey, MonkeyNumber::Sub(a, b));
                }
                "*" => {
                    let _ = monkeys.insert(monkey, MonkeyNumber::Mul(a, b));
                }
                "/" => {
                    let _ = monkeys.insert(monkey, MonkeyNumber::Div(a, b));
                }
                s => panic!("bad operation: {s:?}"),
            }
        }
    }
    println!("{}", resolve_monkey("root", &mut monkeys));
}
