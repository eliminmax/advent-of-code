// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 9 Part 1

use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
enum GroupMarker {
    Start,
    End,
}

/// Remove all garbage, and all characters other than `{` and `}`.
fn filter_stream(stream: String) -> Vec<GroupMarker> {
    let mut ret: Vec<GroupMarker> = Vec::new();
    let mut stream_chars = stream.chars();
    while let Some(c) = stream_chars.next() {
        match c {
            '{' => ret.push(GroupMarker::Start),
            '}' => ret.push(GroupMarker::End),
            '<' => {
                'garb: while let Some(garbage) = stream_chars.next() {
                    match garbage {
                        '!' => _ = stream_chars.next(),
                        '>' => break 'garb,
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    ret
}
fn score_groups(stream: Vec<GroupMarker>) -> u64 {
    let mut total = 0u64;
    let mut nesting_level = 0u64;
    stream.into_iter().for_each(|marker| match marker {
        GroupMarker::Start => nesting_level += 1,
        GroupMarker::End => {
            total += nesting_level;
            nesting_level -= 1;
        }
    });
    total
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    println!("{:?}", score_groups(filter_stream(input)));
}
