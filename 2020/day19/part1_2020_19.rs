// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 19 Part 1
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Clone)]
enum Rule {
    Resolved(HashSet<String>),
    Ref(u8),
    SingleSeq(u8, u8),
    Choice(u8, u8),
    ChoiceOfSeq((u8, u8), (u8, u8)),
}

fn resolve_rule_0(rule_text: &str) -> HashSet<String> {
    let mut rules = HashMap::new();

    for line in rule_text.lines() {
        let (num, rule) = line.trim().split_once(": ").unwrap();
        let num: u8 = num.parse().unwrap();
        let rule_tokens: Vec<_> = rule.split_whitespace().collect();
        match &rule_tokens[..] {
            [a, b, "|", c, d] => {
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                let c = c.parse().unwrap();
                let d = d.parse().unwrap();
                rules.insert(num, Rule::ChoiceOfSeq((a, b), (c, d)));
            }
            [a, "|", b] => {
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                rules.insert(num, Rule::Choice(a, b));
            }
            [a, b] => {
                let a = a.parse().unwrap();
                let b = b.parse().unwrap();
                rules.insert(num, Rule::SingleSeq(a, b));
            }
            [s] if s.starts_with("\"") => {
                let s = s
                    .trim()
                    .strip_prefix("\"")
                    .unwrap()
                    .strip_suffix("\"")
                    .unwrap()
                    .to_owned();
                rules.insert(num, Rule::Resolved(HashSet::from([s])));
            }
            [n] => {
                rules.insert(num, Rule::Ref(n.parse().unwrap()));
            }
            _ => panic!("invalid rule token sequence {rule_tokens:?}"),
        }
    }
    let mut unresolved: VecDeque<u8> = rules.keys().cloned().collect();

    while let Some(id) = unresolved.pop_front() {
        // macro to get a resolved entry, restoring `id` to the queue and moving on if not yet
        // resolved
        macro_rules! get_entry {
            ($k: ident) => {{
                let Rule::Resolved(k) = rules[&$k].clone() else {
                    unresolved.push_back(id);
                    continue;
                };
                k
            }};
        }
        let resolved = match rules[&id] {
            Rule::Resolved(_) => continue,
            Rule::SingleSeq(a, b) => {
                let a = get_entry!(a);
                let b = get_entry!(b);
                let mut resolved_set = HashSet::with_capacity(a.len() * b.len());
                for start in a.iter() {
                    for end in b.iter() {
                        resolved_set.insert(format!("{start}{end}"));
                    }
                }
                Rule::Resolved(resolved_set)
            }
            Rule::Ref(n) => {
                let n = get_entry!(n);
                Rule::Resolved(n)
            }
            Rule::Choice(a, b) => {
                let mut a = get_entry!(a);
                a.extend(get_entry!(b));
                Rule::Resolved(a)
            }
            Rule::ChoiceOfSeq((a, b), (c, d)) => {
                let a = get_entry!(a);
                let b = get_entry!(b);
                let c = get_entry!(c);
                let d = get_entry!(d);
                let mut resolved_set =
                    HashSet::with_capacity((a.len() * b.len()) + (c.len() * d.len()));
                for start in a.iter() {
                    for end in b.iter() {
                        resolved_set.insert(format!("{start}{end}"));
                    }
                }
                for start in c.iter() {
                    for end in d.iter() {
                        resolved_set.insert(format!("{start}{end}"));
                    }
                }
                Rule::Resolved(resolved_set)
            }
        };
        rules.entry(id).and_modify(|e| *e = resolved);
    }

    let Rule::Resolved(ret) = rules.remove(&0).unwrap() else {
        panic!("unresolved rule 0");
    };
    ret
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let (rule_text, patterns) = input.split_once("\n\n").unwrap();
    let rule_0 = resolve_rule_0(rule_text);

    let valid_rule_count = patterns
        .lines()
        .filter(|line| rule_0.contains(*line))
        .count();

    println!("{valid_rule_count}");
}
