// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 18 Part 2

use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
    let [
        (mut x_min, mut x_max),
        (mut y_min, mut y_max),
        (mut z_min, mut z_max),
    ] = [(i8::MAX, i8::MIN); 3];
    let cubes: HashSet<Cube> = input
        .lines()
        .map(|line| {
            let l: Vec<&str> = line.split(',').collect();
            let x = l[0].parse().unwrap();
            let y = l[1].parse().unwrap();
            let z = l[2].parse().unwrap();
            x_min = x_min.min(x - 1);
            y_min = y_min.min(y - 1);
            z_min = z_min.min(z - 1);
            x_max = x_max.max(x + 1);
            y_max = y_max.max(y + 1);
            z_max = z_max.max(z + 1);
            Cube { x, y, z }
        })
        .collect();

    x_min -= 1;

    let mut exterior = BTreeSet::new();

    let mut queue = BTreeSet::from([Cube {
        x: x_min,
        y: y_min,
        z: z_min,
    }]);

    while let Some(position) = queue.pop_first() {
        exterior.insert(position);

        'inner: for n in position.neighbors() {
            macro_rules! check {
                { $axis: ident in $start: ident ..= $end: ident} => {
                    if n.$axis < $start || n.$axis > $end { continue 'inner; }
                };
                { $cube:ident in $set: ident } => {
                    if $set.contains(&$cube) { continue 'inner; }
                };

            }
            check! {n in exterior};
            check! {n in cubes};
            check! {x in x_min..=x_max};
            check! {y in y_min..=y_max};
            check! {z in z_min..=z_max};

            queue.insert(n);
        }
    }

    let mut empty_faced: HashMap<Cube, u16> = HashMap::new();

    for faced_cube in cubes.iter().flat_map(|c| c.neighbors()) {
        if exterior.contains(&faced_cube) {
            empty_faced
                .entry(faced_cube)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }

    println!("{}", empty_faced.values().copied().sum::<u16>());
}
