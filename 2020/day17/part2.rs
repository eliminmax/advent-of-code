// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 17 Part 2

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ConwayCubeLocation {
    x: i16,
    y: i16,
    z: i16,
    w: i16,
}

impl ConwayCubeLocation {
    fn neighbors(&self) -> impl Iterator<Item = Self> {
        let Self { x, y, z, w } = *self;

        (x - 1..=x + 1)
            .flat_map(move |nx| {
                (y - 1..=y + 1).flat_map(move |ny| {
                    (z - 1..=z + 1).flat_map(move |nz| {
                        (w - 1..=w + 1).map(move |nw| Self {
                            x: nx,
                            y: ny,
                            z: nz,
                            w: nw,
                        })
                    })
                })
            })
            .filter(move |nself| nself != self)
    }
}

fn update(active: &mut HashSet<ConwayCubeLocation>) {
    let mut next_gen_living = HashSet::new();
    let mut inactive_neighbors = HashSet::new();

    for location in active.iter().copied() {
        let mut count = 0;
        for neighbor in location.neighbors() {
            if active.contains(&neighbor) {
                count += 1;
            } else {
                inactive_neighbors.insert(neighbor);
            }
        }
        if count == 2 || count == 3 {
            next_gen_living.insert(location);
        }
    }

    for location in inactive_neighbors {
        if location
            .neighbors()
            .filter(|ccl| active.contains(ccl))
            .count()
            == 3
        {
            next_gen_living.insert(location);
        }
    }
    active.clear();
    active.extend(next_gen_living);
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut active: HashSet<ConwayCubeLocation> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        let y = i16::try_from(y).unwrap();
        for (x, c) in line.chars().enumerate() {
            let x = i16::try_from(x).unwrap();
            if c == '#' {
                active.insert(ConwayCubeLocation { x, y, z: 0, w: 0 });
            }
        }
    }

    for _ in 0..6 {
        update(&mut active);
    }

    println!("{}", active.len());
}
