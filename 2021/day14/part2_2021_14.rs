// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 14 Part 2

//! So this is one of those days with a fairly basic part 1, and a fairly complex part 2, even
//! though the only difference is the number of iterations.
//!
//! My first approach was to up the round count to 40 with the assumption it would be too slow
//! and/or memory intensive - I added an eprintln of the current round number and starting size,
//! then ran `timeout 20 cargo r --release --quiet 2>&1 | ts -i '%H:%M:%.S' > log`, to get a log of
//! the first 20 seconds.
//!
//! Looking at `log`, after the first round beyound around round 13 approximately doubled in time
//! to calculate, and the number of elements looked to be always exactly one less than twice that
//! of the previous round, and it made it to round 28 before being timed out. At that point, the
//! length was 2550136833, which at 4 bytes per element, had a minimum size of 9.6 GiB.
//!
//! Some quick and dirty spreadsheet math found that if that the length were to hold, and the time
//! were to keep more-or-less doubling, then by the end, it would have 20890720927745 elements
//! (which would take around 77 TiB, plus any bookkeeping)
//!
//! In addition, while the compilation time did slow things down at first, and it took a few rounds
//! to get up to speed, after the first 12 or so rounds, the amount of time per round started to
//! approach the sum of the previous rounds' times, and if that continued to hold, it would take
//! around 16 hours to complete, in a fantasy world where I had enough RAM.
//!
//! For my input, all possible pairings of characters that could exist do show up, so the
//! calculated final length of 20890720927745 is going to be exactly right.

use std::collections::HashMap;

#[derive(Default, Clone)]
/// data about the number of elements in the polymer chain
struct PolymerInfo {
    /// the total number of each element that appears, not counting the tail
    data: HashMap<u8, u64>,
    /// the final element, which is not counted in `data`, to avoid double-counting `b` when
    /// combining PolymerInfo for (ab) with PolymerInfo for (bc)
    tail: u8,
}

type CacheIndex = ((u8, u8), u8);
type ChainCache = HashMap<CacheIndex, PolymerInfo>;

/// Check if `cache` contains an entry for `index`, and compute then insert one if it doesn't exist.
///
/// If `rounds` is greater than `0`, it needs cached values for `((a, rules[&(a, b)), rounds - 1)`
/// and `((rules[&(a, b)], b), rounds - 1)`, so it calls itself recursively to ensure they're
/// available.
fn compute_memoizing<'a>(
    ((a, b), rounds): CacheIndex,
    rules: &HashMap<(u8, u8), u8>,
    cache: &'a mut ChainCache,
) -> &'a PolymerInfo {
    if !cache.contains_key(&((a, b), rounds)) {

        let new_entry = {
            if rounds == 0 {
                PolymerInfo {
                    data: HashMap::from([(a, 1)]),
                    tail: b,
                }
            } else {
                let inserted = rules[&(a, b)];
                // clone to avoid holding a borrow, as that would create lifetime issues
                let start = compute_memoizing(((a, inserted), rounds - 1), rules, cache).clone();
                start.joined_with(compute_memoizing(((inserted, b), rounds - 1), rules, cache))
            }
        };

        cache.insert(((a, b), rounds), new_entry);
    }
    &cache[&((a, b), rounds)]
}

impl PolymerInfo {
    fn joined_with(&self, other: &Self) -> Self {
        let mut joined = other.clone();
        for (&elem, &c) in self.data.iter() {
            joined.data.entry(elem).and_modify(|e| *e += c).or_insert(c);
        }
        joined
    }

    /// Build the `PolymerInfo` for the polymer created by applying `rules` to `chain` the
    /// specified number of `rounds` - does not actually construct the polymer chain, due to the
    /// time and memory needed, as explained in the doc comment above.
    fn evaluate_chain(chain: &[u8], rules: &HashMap<(u8, u8), u8>, rounds: u8) -> Self {
        // The upper bound for the number of entries is 1 per rule per round, plus 1 per rule for
        // the base case of 0 rounds.
        let mut cache: ChainCache = HashMap::with_capacity(rules.len() * usize::from(rounds + 1));

        chain.windows(2).fold(Self::default(), |acc, w| {
            acc.joined_with(compute_memoizing(((w[0], w[1]), rounds), rules, &mut cache))
        })
    }

    /// score the PolymerInfo - returns the difference between the smallest and largest counts.
    fn score(mut self) -> u64 {
        // first, tail needs to actually be counted within the data
        self.data
            .entry(self.tail)
            .and_modify(|e| *e += 1)
            .or_insert(1);
        // now, figure out the maximum and minimum counts
        let mut min = u64::MAX;
        let mut max = u64::MIN;
        for count in self.data.into_values() {
            max = max.max(count);
            min = min.min(count);
        }
        max - min
    }
}
fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let (polymer, input) = input.split_once("\n\n").unwrap();
    let rules: HashMap<(u8, u8), u8> = (input.lines())
        .map(|line| {
            let &[a, b, b' ', b'-', b'>', b' ', out] = line.as_bytes() else {
                panic!("invalid line structure: {line}")
            };
            ((a, b), out)
        })
        .collect();

    println!(
        "{}",
        PolymerInfo::evaluate_chain(polymer.as_bytes(), &rules, 40).score()
    );
}
