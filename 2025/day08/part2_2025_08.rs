// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 08 Part 2

use std::collections::{HashSet, VecDeque};

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

fn indices_of(
    j0: &JunctionBox,
    j1: &JunctionBox,
    circuits: &[HashSet<JunctionBox>],
) -> (usize, Option<usize>) {
    let mut indices: [Option<usize>; 2] = [None, None];
    for (i, c) in circuits.iter().enumerate() {
        if c.contains(j0) {
            indices[0] = Some(i);
            if indices[1].is_some() {
                break;
            }
        }
        if c.contains(j1) {
            indices[1] = Some(i);
            if indices[0].is_some() {
                break;
            }
        }
    }
    let i0 = indices[0].unwrap();
    let i1 = indices[1].unwrap();
    if i0 == i1 {
        (i0, None)
    } else {
        // to reduce the need to shift elements, sort out the elements here.
        let mut is = [i0, i1];
        is.sort();
        (is[0], Some(is[1]))
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
    let mut pairs: VecDeque<(JunctionBox, JunctionBox)> = VecDeque::with_capacity(500000);
    for (i, j0) in junctions.iter().enumerate() {
        for j1 in junctions[i + 1..].iter() {
            pairs.push_back((*j0, *j1));
        }
    }
    pairs
        .make_contiguous()
        .sort_unstable_by_key(|(j0, j1)| j0.distance(j1));
    let mut circuits: Vec<HashSet<JunctionBox>> =
        junctions.into_iter().map(|j| HashSet::from([j])).collect();

    while circuits.len() > 2 {
        let (j0, j1) = pairs.pop_front().unwrap();
        if let (i0, Some(i1)) = indices_of(&j0, &j1, &circuits) {
            let c = circuits.remove(i1);
            circuits[i0].extend(c);
        }
    }
    assert_eq!(circuits.len(), 2);
    while indices_of(&pairs[0].0, &pairs[0].1, &circuits).1.is_none() {
        pairs.pop_front();
    }
    println!("{}", pairs[0].0.x * pairs[0].1.x);
}
