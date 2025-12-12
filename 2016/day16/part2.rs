// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 16 Part 2

const TARGET_SIZE: usize = 35651584;

fn dragon(a: &mut Vec<bool>) {
    let mut b = a.clone();
    b.reverse();
    b.iter_mut().for_each(|val| *val ^= true); // ^= true on a boolean will flip its value
    a.push(false);
    a.append(&mut b);
}

fn cksum(mut a: Vec<bool>) -> String {
    loop {
        a = a.chunks_exact(2).map(|c| c[0] == c[1]).collect();
        if a.len() % 2 == 1 {
            return a.into_iter().map(|b| if b { '1' } else { '0' }).collect();
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut state: Vec<bool> = input
        .trim()
        .chars()
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => panic!("Invalid character: {}", c),
        })
        .collect();
    while state.len() < TARGET_SIZE {
        dragon(&mut state);
    }
    state.resize(TARGET_SIZE, false);
    println!("{}", cksum(state));
}
