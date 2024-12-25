// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 13 Part 1

// Looking at the size of the input, there are 7 people, meaning there are 7! (i.e. 5040)
// possibilities to look at - a small enough number to brute force

use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
use std::ops::{Deref, DerefMut};

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

#[derive(Debug)]
struct PrefTableParseError;

#[derive(Debug)]
#[repr(transparent)]
/// A thin wrapper around the inner HashMap which adds methods for parsing prefs from strings and
/// scoring permutations. Transparently derefs into its inner HashMap. Based on WireKit<'a> from
/// my 2024 day 24 part 1 solution
struct PrefTable<'a>(HashMap<(&'a str, &'a str), i16>);

impl<'a> Deref for PrefTable<'a> {
    type Target = HashMap<(&'a str, &'a str), i16>;
    fn deref(&self) -> &HashMap<(&'a str, &'a str), i16> {
        &self.0
    }
}

impl DerefMut for PrefTable<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PrefTable<'_> {
    fn bound_to<'a>(_binding: &'a str) -> PrefTable<'a> {
        PrefTable::<'a>(HashMap::new())
    }
}

impl<'a> PrefTable<'a> {
    fn get_names(&self) -> Vec<&'a str> {
        let mut names: Vec<&'a str> = self.keys().map(|(a, _)| *a).collect();
        names.sort();
        names.dedup();
        names
    }

    fn load_pref(&mut self, pref_str: &'a str) -> Result<(), PrefTableParseError> {
        let words: Vec<&'a str> = pref_str.split_whitespace().collect();
        if let [name0, _, effect, amount, _, _, _, _, _, _, name1] = &words[..] {
            let name1 = name1.strip_suffix(".").ok_or(PrefTableParseError)?;
            let mut amount = amount.parse::<i16>().map_err(|_| PrefTableParseError)?;
            if *effect == "lose" {
                amount = -amount;
            }
            let _ = &self.insert((name0, name1), amount);
            Ok(())
        } else {
            Err(PrefTableParseError)
        }
    }

    fn score(&self, mut order: Vec<&'a str>) -> i16 {
        // start in one direction
        let forwards_score = self[&(*order.last().expect("empty order provided"), order[0])]
            + order
                .windows(2)
                .map(|pairing| &self[&(pairing[0], pairing[1])])
                .sum::<i16>();
        // now reverse, and do the same thing again
        order.reverse();
        self[&(*order.last().unwrap_or_else(|| unreachable!()), order[0])]
            + order
                .windows(2)
                .map(|pairing| &self[&(pairing[0], pairing[1])])
                .sum::<i16>()
            + forwards_score
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut prefs: PrefTable = PrefTable::bound_to(&input);
    input.lines().for_each(|line| {
        prefs.load_pref(line).expect("Failed to parse line");
    });
    let mut names = prefs.get_names();
    let orders = heap_permutations(&mut names[..]);
    println!(
        "{}",
        orders
            .into_iter()
            .map(|order| prefs.score(order))
            .max()
            .expect("No preference")
    );
}
