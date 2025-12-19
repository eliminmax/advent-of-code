// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 18 Part 1

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    x: i8,
    y: i8,
    z: i8,
}

impl Cube {
    const fn neighbors(&self) -> [Self; 6] {
        macro_rules! n {
            [$off: tt $axis: ident] => { Self { $axis: self.$axis $off 1, ..*self } }
        }

        [n![-x], n![+x], n![-y], n![+y], n![-z], n![+z]]
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let cubes: HashSet<Cube> = input
        .lines()
        .map(|line| {
            let l: Vec<&str> = line.split(',').collect();
            let x = l[0].parse().unwrap();
            let y = l[1].parse().unwrap();
            let z = l[2].parse().unwrap();
            Cube { x, y, z }
        })
        .collect();
    let total = cubes
        .iter()
        .map(|c| c.neighbors().iter().filter(|n| !cubes.contains(n)).count())
        .sum::<usize>();
    println!("{total}");
}
