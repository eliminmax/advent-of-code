// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 09 Part 1
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position {
    row: i16,
    col: i16,
}

impl Position {
    fn neighbors(self) -> [Self; 4] {
        macro_rules! n {
            {$op: tt $member: ident} => {
                Self {
                    $member: self.$member $op 1,
                    ..self
                }
            };
        }
        [n! {- row}, n! {+ row}, n! {- col}, n! {+ col}]
    }
}

fn build_height_map(s: &str) -> BTreeMap<Position, u32> {
    let mut map = BTreeMap::new();
    for (r, line) in s.lines().enumerate() {
        let row = i16::try_from(r + 1).unwrap();
        for (c, num) in line.chars().map(|d| d.to_digit(10).unwrap()).enumerate() {
            let col = i16::try_from(c + 1).unwrap();
            map.insert(Position { row, col }, num);
        }
    }
    map
}

fn is_local_minimum(pos: &Position, map: &BTreeMap<Position, u32>) -> bool {
    let height = map[pos];
    for neighbor in pos.neighbors() {
        if map.get(&neighbor).is_some_and(|n| *n <= height) {
            return false;
        }
    }
    true
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let map = build_height_map(&input);

    let sum_of_risk: u32 = map
        .iter()
        .filter(|(p, _)| is_local_minimum(p, &map))
        .map(|(_, h)| h + 1)
        .sum();
    println!("{sum_of_risk}");
}
