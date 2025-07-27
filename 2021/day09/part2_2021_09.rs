// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 09 Part 2
use std::collections::{BTreeMap, HashSet, VecDeque};

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

fn build_height_map(s: &str) -> BTreeMap<Position, u8> {
    let mut map = BTreeMap::new();
    for (r, line) in s.lines().enumerate() {
        let row = i16::try_from(r + 1).unwrap();
        for (c, num) in line.chars().map(|d| d.to_digit(10).unwrap()).enumerate() {
            let col = i16::try_from(c + 1).unwrap();
            map.insert(Position { row, col }, num as u8);
        }
    }
    map
}

fn is_local_minimum(pos: &Position, map: &BTreeMap<Position, u8>) -> bool {
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
    let mut map = build_height_map(&input);
    let low_points: HashSet<Position> = map
        .keys()
        .copied()
        .filter(|k| is_local_minimum(k, &map))
        .collect();
    map.retain(|_, v| *v != 9);

    let mut sizes: Vec<usize> = Vec::with_capacity(low_points.len());

    for point in low_points.iter().cloned() {
        let mut basin = HashSet::new();
        let mut queue = VecDeque::from([point]);
        while let Some(pos) = queue.pop_front() {
            if basin.insert(pos) {
                assert!(pos == point || !low_points.contains(&pos), "Overlapping basins");
                queue.extend(pos.neighbors().into_iter().filter(|n| map.contains_key(n)));
            }
        }
        sizes.push(basin.len());
    }
    // sort sizes from highest to lowest
    sizes.sort_by(|a, b| b.cmp(a));
    let answer: usize = sizes[..3].iter().product();
    println!("{answer}");

}
