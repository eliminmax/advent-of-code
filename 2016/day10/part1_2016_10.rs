// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 10 Part 1
use std::convert::{TryFrom, TryInto};
use std::default::Default;

#[derive(Debug, Copy, Clone)]
enum TransferTarget {
    Bot(usize),
    Output(usize),
}

impl TryFrom<(&str, &str)> for TransferTarget {
    type Error = RuleParseError;
    fn try_from((t, n): (&str, &str)) -> Result<Self, Self::Error> {
        let num: usize = n.parse()?;
        match t {
            "bot" => Ok(TransferTarget::Bot(num)),
            "output" => Ok(TransferTarget::Output(num)),
            _ => Err(RuleParseError),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct TransferRule {
    low: TransferTarget,
    high: TransferTarget,
}

#[derive(Debug, Default, Clone)]
struct Bot {
    items: Vec<u8>,
    rule: Option<TransferRule>,
}

#[derive(Debug)]
struct RuleParseError;
impl From<std::num::ParseIntError> for RuleParseError {
    fn from(_err: std::num::ParseIntError) -> Self {
        RuleParseError
    }
}

#[derive(Debug)]
struct BotSystem {
    bots: Vec<Bot>,
    outputs: Vec<Vec<u8>>,
}

#[derive(Debug)]
enum ProcessingError{
    IncompleteBot,
    MatchNotFound
}

impl BotSystem {
    fn new() -> Self {
        BotSystem {
            bots: vec![Default::default(); 255],
            outputs: vec![Default::default(); 32],
        }
    }

    fn parse_rule(&mut self, rule: &str) -> Result<(), RuleParseError> {
        let words: Vec<&str> = rule.split_whitespace().collect();
        match &words[..] {
            ["value", v, "goes", "to", "bot", b] => {
                let bot_num: usize = b.parse()?;
                let val: u8 = v.parse()?;
                self.bots[bot_num].items.push(val);
            }
            // not checking every single word for this one
            ["bot", b, _, _, _, low_dtype, low_dn, _, _, _, high_dtype, high_dn] => {
                let bot_num: usize = b.parse()?;
                let low: TransferTarget = (*low_dtype, *low_dn).try_into()?;
                let high: TransferTarget = (*high_dtype, *high_dn).try_into()?;
                self.bots[bot_num].rule = Some(TransferRule { low, high });
            }
            _ => return Err(RuleParseError),
        }
        Ok(())
    }

    fn find_comparer(&mut self, target: (u8, u8)) -> Result<usize, ProcessingError> {
        use std::collections::VecDeque;
        let mut queue: VecDeque<usize> = self
            .bots
            .iter()
            .enumerate()
            .filter_map(|(id, bot)| {
                if bot.items.len() == 2 {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();
        while let Some(id) = queue.pop_front() {
            let rule = self.bots[id].rule.ok_or(ProcessingError::IncompleteBot)?;
            let mut items = self.bots[id].items.clone();
            items.sort();
            if (items[0], items[1]) == target {
                return Ok(id);
            }
            macro_rules! process_item {
                ($half: ident, $index: literal) => {{
                    match rule.$half {
                        TransferTarget::Bot(next_id) => {
                            self.bots[next_id].items.push(items[$index]);
                            if self.bots[next_id].items.len() == 2 {
                                queue.push_back(next_id);
                            }
                        }
                        TransferTarget::Output(output_id) => {
                            self.outputs[output_id].push(items[$index])
                        }
                    }
                }};
            }
            self.bots[id].items.clear();
            process_item!(low, 0);
            process_item!(high, 1);
        }
        Err(ProcessingError::MatchNotFound)
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut system = BotSystem::new();
    input
        .lines()
        .for_each(|line| system.parse_rule(line).expect("Failed to parse input"));
    println!("{}", system.find_comparer((17, 61)).expect("Failed to find bot of interest"));
}
