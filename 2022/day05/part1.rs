// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 5 Part 1

use std::cell::RefCell;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

fn key_index(key: u8) -> usize {
    1 + (4 * (key as usize - 1))
}

// takes an instruction of the form "moves N from N to N", where N is a positive integer value,
// and processes that instruction as described in the Advent of Code description
fn process_instruction(instruction: &str, stacks: &mut HashMap<u8, RefCell<Vec<char>>>) {
    let words: Vec<&str> = instruction.split_whitespace().collect();
    let count = usize::from_str(words[1]).expect("Invalid move instruction format");
    let from = u8::from_str(words[3]).expect("Invalid move instruction format");
    let to = u8::from_str(words[5]).expect("Invalid move instruction format");
    let from_stack: &mut Vec<char> = &mut stacks
        .get(&from)
        .expect("Failed to get \"from\" tower")
        .borrow_mut();
    let to_stack: &mut Vec<char> = &mut stacks
        .get(&to)
        .expect("Failed to get \"to\" tower")
        .borrow_mut();

    let from_index = from_stack.len() - count;
    let moved: Vec<char> = from_stack.split_off(from_index).into_iter().rev().collect();
    to_stack.extend(moved);
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let (stack_text, moves_text) = input
        .split_once("\n\n")
        .expect("Failed to find \"\\n\\n\" in input");

    // split stack text on lines, and reverse, so that tower labels are first.
    let mut stack_setup = stack_text.lines().filter(|l| !l.is_empty()).rev();
    let mut stacks: HashMap<u8, RefCell<Vec<char>>> = HashMap::new();

    stack_setup
        .next()
        .expect("Failed to read stack labels")
        .split_whitespace()
        .for_each(|label| {
            stacks.insert(
                u8::from_str(label).expect("Stack labels are supposed to be single-digit numbers"),
                RefCell::new(Vec::new()),
            );
        });

    let mut keys: Vec<u8> = stacks.keys().copied().collect();
    keys.sort();
    // once sorted, make keys immutable
    let keys = keys;

    for row in stack_setup {
        for (stack_id, stack_contents) in stacks.iter_mut() {
            let crate_contents = row.as_bytes()[key_index(*stack_id)] as char;
            if crate_contents.is_ascii_uppercase() {
                stack_contents.borrow_mut().push(crate_contents);
            }
        }
    }

    moves_text
        .lines()
        .for_each(|instruction| process_instruction(instruction, &mut stacks));

    println!(
        "{}",
        keys.iter()
            .map(|key| {
                *(stacks
                    .get(key)
                    .expect("keys shouldn't be removed from `stacks`")
                    .borrow()
                    .iter()
                    .last())
                .expect("empty stack at end of program")
            })
            .collect::<String>()
    );
}
