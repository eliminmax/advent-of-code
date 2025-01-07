// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 9 Part 1

use std::collections::{HashMap, HashSet};
use std::env::args;
use std::fs::read_to_string;
use std::iter::FromIterator;

fn heap_permutations<T: Copy>(vals: &mut [T]) -> Vec<Vec<T>> {
    // generic version of my implementation of Heap's Algorithm from 2019 day 07
    if vals.len() == 1 {
        return vec![vals.to_owned()];
    }
    let mut permutations: Vec<Vec<T>> = Vec::new();
    let mut new_perms: Vec<Vec<T>>;
    let last = vals.len() - 1;
    for i in 0..=last {
        new_perms = heap_permutations(&mut vals[..last]);
        new_perms.iter_mut().for_each(|v| v.push(vals[last]));
        permutations.append(&mut new_perms);
        if last % 2 == 0 {
            vals.swap(0, last);
        } else {
            vals.swap(i, last);
        }
    }
    permutations
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut distances: HashMap<(&str, &str), u16> = HashMap::new();
    let mut cities: HashSet<&str> = HashSet::new();
    for line in input.lines() {
        let words: Vec<&str> = line.split_whitespace().collect();
        if let [src, "to", dst, "=", distance] = words[..] {
            let distance = distance
                .parse::<u16>()
                .expect("Failed to parse distance as u16");
            cities.insert(src);
            cities.insert(dst);
            distances.insert((src, dst), distance);
            distances.insert((dst, src), distance);
        } else {
            panic!("Line {} could not be parsed", line);
        }
    }
    let mut cities = Vec::from_iter(cities);
    let mut min_dist = u16::MAX;
    for permutation in heap_permutations(cities.as_mut_slice()) {
        let travel_lengths: Vec<u16> = permutation
            .windows(2)
            .map(|w| distances[&(w[0], w[1])])
            .collect();
        min_dist = min_dist.min(travel_lengths.iter().sum());
    }

    println!("{min_dist}");
}
