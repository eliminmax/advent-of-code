// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 08 Part 1

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct JunctionBox {
    x: u32,
    y: u32,
    z: u32,
}

impl JunctionBox {
    fn distance(&self, other: &Self) -> u64 {
        macro_rules! axis_dist {
            ($axis: ident) => {{
                let n = self.$axis.abs_diff(other.$axis) as u64;
                n * n
            }};
        }
        axis_dist!(x) + axis_dist!(y) + axis_dist!(z)
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let junctions: Vec<_> = input
        .lines()
        .map(|l| {
            let (x, yz) = l.split_once(',').unwrap();
            let (y, z) = yz.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let z = z.parse().unwrap();
            JunctionBox { x, y, z }
        })
        .collect();
    // might temporarily have 1 extra element, but no more
    let mut pairs: Vec<(u64, JunctionBox, JunctionBox)> = Vec::with_capacity(1001);
    for (i, j0) in junctions.iter().enumerate() {
        for j1 in junctions[i + 1..].iter() {
            let d = j0.distance(j1);
            if pairs.len() < 1000 || pairs.last().unwrap().0 > d {
                let insert_point = pairs.partition_point(|&(pd, _j0, _j1)| pd < d);
                pairs.insert(insert_point, (d, *j0, *j1));
            }
            if pairs.len() > 1000 {
                pairs.drain(1000..);
            }
        }
    }
    let mut circuits: Vec<HashSet<JunctionBox>> = Vec::with_capacity(1000);
    'outer: for (_, j0, j1) in pairs {
        let mut indices: [Option<usize>; 2] = [None, None];
        'inner: for (i, c) in circuits.iter().enumerate() {
            if c.contains(&j0) {
                indices[0] = Some(i);
                if indices[1].is_some() {
                    break 'inner;
                }
            }
            if c.contains(&j1) {
                indices[1] = Some(i);
                if indices[0].is_some() {
                    if indices[0] == indices[1] {
                        continue 'outer;
                    }
                    break 'inner;
                }
            }
        }
        match indices {
            [None, None] => circuits.push(HashSet::from([j0, j1])),
            [Some(i), None] => {
                circuits[i].insert(j1);
            }
            [None, Some(i)] => {
                circuits[i].insert(j0);
            }
            [Some(a), Some(b)] => {
                // to reduce the need to shift elements, sort out the elements here.
                let mut sorter = [a, b];
                sorter.sort();
                let [i0, i1] = sorter;
                let c = circuits.remove(i1);
                circuits[i0].extend(c);
            }
        }
    }
    let mut circuit_lens: Vec<usize> = circuits.into_iter().map(|hs| hs.len()).collect();
    circuit_lens.sort();
    let [a, b, c] = core::array::from_fn(|_| circuit_lens.pop().unwrap());
    println!("{}", a * b * c);
}
