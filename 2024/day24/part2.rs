// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 24 Part 2

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum WireKitError {
    UnknownWire,
    Unparsable,
    Redefinition,
    UndefinedValue,
    IncorrectValue,
    DependencyLoop,
    MissingStartWire,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum WireOp {
    And,
    Xor,
    Or,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Wire {
    Literal(bool),
    FromWires(&'static str, &'static str, WireOp),
}

#[derive(Debug, PartialEq, Clone)]
/// A thin wrapper around the inner HashMap which adds methods for parsing wires from strings and
/// resolving wire values. Transparently derefs into its inner HashMap.
struct WireKit(HashMap<&'static str, Wire>);

impl std::ops::Deref for WireKit {
    type Target = HashMap<&'static str, Wire>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for WireKit {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WireKit {
    fn new() -> WireKit {
        WireKit(HashMap::<&'static str, Wire>::new())
    }
    fn insert_wire(&mut self, wire_id: &'static str, wire: Wire) -> Result<(), WireKitError> {
        if self.insert(wire_id, wire).is_none() {
            Ok(())
        } else {
            Err(WireKitError::Redefinition)
        }
    }

    /// Load a literal wire of the form `x00: 1`
    fn literal_wire(&mut self, wire_str: &'static str) -> Result<(), WireKitError> {
        let words: Vec<&str> = wire_str.split_whitespace().collect();
        if let [wire, val] = &words[..] {
            let wire = wire.strip_suffix(":").ok_or(WireKitError::Unparsable)?;
            match *val {
                "0" => self.insert_wire(wire, Wire::Literal(false)),
                "1" => self.insert_wire(wire, Wire::Literal(true)),
                _ => Err(WireKitError::Unparsable),
            }
        } else {
            Err(WireKitError::Unparsable)
        }
    }

    /// Load a wire defined by other wires in the form `x00 AND y00 -> z00`
    fn wire_from_wires(&mut self, wire_str: &'static str) -> Result<(), WireKitError> {
        let words: Vec<&str> = wire_str.split_whitespace().collect();
        if let [wire_a, op, wire_b, "->", dest_wire] = &words[..] {
            let op = match *op {
                "AND" => Ok(WireOp::And),
                "XOR" => Ok(WireOp::Xor),
                "OR" => Ok(WireOp::Or),
                _ => Err(WireKitError::Unparsable),
            }?;
            let mut ordered: [&'static str; 2] = [wire_a, wire_b];
            ordered.sort();
            self.insert_wire(dest_wire, Wire::FromWires(ordered[0], ordered[1], op))
        } else {
            Err(WireKitError::Unparsable)
        }
    }

    /// try to resolve wire value
    /// * If already resolved (i.e. `matches!(&self[wire_id], Wire::Literal(_))`, returns `Ok(true)`
    /// * Otherwise:
    ///     * if wire_a and wire_b are both resolved, sets `&self[wire_id]` to the resolved signal,
    ///       then returns `Ok(true)`
    ///     * if not enough information is available, it returns `Ok(false)`
    ///     * if wire_a or wire_b are not found within self, returns
    ///       `Err(WireKitError::UnknownWire)`
    fn try_resolve(&mut self, wire_id: &'static str) -> Result<bool, WireKitError> {
        match &self[wire_id] {
            Wire::Literal(_) => Ok(true),
            Wire::FromWires(wire_a, wire_b, op) => match (self.get(wire_a), self.get(wire_b)) {
                (Some(Wire::Literal(a)), Some(Wire::Literal(b))) => {
                    let signal = match op {
                        WireOp::And => *a && *b,
                        WireOp::Xor => *a != *b,
                        WireOp::Or => *a || *b,
                    };
                    let _ = self.insert(wire_id, Wire::Literal(signal));
                    Ok(true)
                }
                (Some(_), Some(_)) => Ok(false),
                _ => Err(WireKitError::UnknownWire),
            },
        }
    }

    /// resolve all wires
    fn resolve_value(&self) -> Result<(), WireKitError> {
        let mut clone = self.clone();
        let mut to_resolve: VecDeque<&str> = clone.keys().cloned().collect();
        let mut unresolved_count: usize = 0;
        while let Some(wire_id) = to_resolve.pop_front() {
            if clone.contains_key(wire_id) {
                if !clone.try_resolve(wire_id)? {
                    to_resolve.push_back(wire_id);
                    unresolved_count += 1;
                    if unresolved_count == to_resolve.len() {
                        return Err(WireKitError::DependencyLoop);
                    }
                } else {
                    unresolved_count = 0;
                }
            } else {
                return Err(WireKitError::UnknownWire);
            }
        }
        let expected = clone.value_of_set('x')? + clone.value_of_set('y')?;
        let actual = clone.value_of_set('z')?;
        if actual == expected {
            eprintln!("0x{actual:x} == 0x{expected:x}");
            Ok(())
        } else {
            Err(WireKitError::IncorrectValue)
        }
    }

    fn value_of_set(&self, set_key: char) -> Result<u64, WireKitError> {
        let mut set_wires: Vec<&str> = self
            .keys()
            .filter(|&k| k.starts_with(set_key))
            .cloned()
            .collect();
        set_wires.sort();
        let mut result = 0u64;
        for (shift, wire) in set_wires.into_iter().enumerate() {
            if let Wire::Literal(bit) = self[wire] {
                result |= (bit as u64) << shift;
            } else {
                return Err(WireKitError::UndefinedValue);
            }
        }
        Ok(result)
    }

    fn clone_with_swaps(&self, swaps: &[(&'static str, &'static str)]) -> Self {
        let mut clone: WireKit = self.clone();
        for (a, b) in swaps.iter() {
            let _ = clone.insert(*a, self[b]);
            let _ = clone.insert(*b, self[a]);
        }
        clone
    }

    /// Per u/LxsterGames on the subreddit, the connections between circuits going into z wires
    /// other than the last one should be XORs, and connections going into any other wires except
    /// if coming from an x and y wire pair should be ANDs or ORs, but not XORs, and 3 of the 4
    /// pairs that need swapping can be found with that information. I did not read beyond that
    /// to see how to figure out how to pair up those found or find the final pair, but it was a
    /// critical insight without which I'd probably be stuck.
    ///
    /// see https://www.reddit.com/r/adventofcode/comments/1hla5ql/
    fn flag_abnormal(&self) -> Vec<&'static str> {
        let final_wire = self.keys().max().expect("No keys");
        let mut abnormal: Vec<&'static str> = Vec::new();
        self.iter().for_each(|(k, v)| match *v {
            Wire::Literal(_) => (),
            Wire::FromWires(a, b, WireOp::Xor) => {
                if !k.starts_with("z") {
                    let mut ordered: [&str; 2] = [a, b];
                    ordered.sort();
                    if !(ordered[0].starts_with("x") && ordered[1].starts_with("y")) {
                        abnormal.push(k);
                    }
                }
            }
            Wire::FromWires(..) => {
                if k.starts_with("z") && k != final_wire {
                    abnormal.push(k);
                }
            }
        });
        abnormal.sort();
        abnormal
    }

    /// tries to set x and y wires according to provided values
    fn set_inputs(&mut self, mut x: u64, mut y: u64) -> Result<(), WireKitError> {
        macro_rules! mask {
            ($takefrom: ident, $numstr: ident) => {{
                let mask_by = $numstr.chars().skip(1).collect::<String>();
                let mask_by = mask_by
                    .parse::<u64>()
                    .map_err(|_| WireKitError::Unparsable)?;
                let mask = 1u64 << mask_by;
                let bit = ($takefrom & mask) >> mask_by;
                $takefrom &= !mask;
                bit == 1
            }};
        }
        for (k, v) in self.iter_mut() {
            if k.starts_with('x') {
                *v = Wire::Literal(mask!(x, k));
            } else if k.starts_with('y') {
                *v = Wire::Literal(mask!(y, k));
            }
        }
        if x == 0 && y == 0 {
            Ok(())
        } else {
            Err(WireKitError::MissingStartWire)
        }
    }

    fn test_with_value(&self, x: u64, y: u64) -> Result<(), WireKitError> {
        let mut clone = self.clone();
        clone.set_inputs(x, y)?;
        clone.resolve_value()
    }
}

fn find_swaps(wires: &WireKit) -> Vec<Vec<(&'static str, &'static str)>> {
    use std::iter::zip;
    use std::sync::mpsc;
    use std::thread;

    let mut matching_candidates: Vec<Vec<(&'static str, &'static str)>> = Vec::new();
    let threads = thread::available_parallelism()
        .expect("Failed to determine appropriate number of threads")
        .get();
    let worker_threads = threads - 1;
    if worker_threads == 0 {
        panic!("Not enough parallelism");
    }
    eprintln!("Working with {worker_threads} threads.");

    let abnormal = wires.flag_abnormal();
    let candidates: Vec<&str> = wires
        .keys()
        .filter_map(|k| {
            if matches!(&wires[k], Wire::FromWires(..)) && !abnormal.contains(k) {
                Some(*k)
            } else {
                None
            }
        })
        .to_owned()
        .collect();

    // it seems short enough to hard-code all orderings here, given that it's only 6 of them.
    const ORDERINGS: [[usize; 3]; 6] = [
        [5, 4, 3],
        [5, 3, 4],
        [4, 5, 3],
        [4, 3, 5],
        [3, 5, 4],
        [3, 4, 5],
    ];
    for swap_indices in ORDERINGS.iter() {
        let swaps: Vec<(&str, &str)> = zip(
            abnormal.iter().cloned(),
            swap_indices.iter().map(|&i| abnormal[i]),
        )
        .collect();
        for a in candidates.iter() {
            for b_group in candidates.chunks(worker_threads) {
                let (tx, rx) = mpsc::channel();
                for b in b_group.iter() {
                    let mut swaps = swaps.clone();
                    let tx = tx.clone();
                    swaps.push((a, b));
                    let test_wires = wires.clone_with_swaps(&swaps[..]);
                    thread::spawn(move || {
                        if test_wires.resolve_value().is_ok() {
                            tx.send(swaps).expect("Failed to send winning swap list");
                        }
                    });
                }
                drop(tx);
                if let Ok(mut swaps) = rx.recv() {
                    // this mess gets the pairs into the right order
                    swaps.iter_mut().for_each(|(a, b)| {
                        let mut ordered = [*a, *b];
                        ordered.sort();
                        *a = ordered[0];
                        *b = ordered[1];
                    });
                    swaps.sort();
                    matching_candidates.push(swaps);
                }
            }
        }
    }
    matching_candidates
}

fn test_candidate(wires: &WireKit, swaps: &[(&'static str, &'static str)]) -> bool {
    const TEST_PATTERNS: [u64; 8] = [
        0xa_aa_aa_aa_aa_aa, // each nibble set to 0b1010
        0x5_55_55_55_55_55, // each nibble set to 0b0101
        0x9_99_99_99_99_99, // each nibble set to 0b1001
        0x6_66_66_66_66_66, // each nibble set to 0b0110
        0x8_88_88_88_88_88, // each nibble set to 0b1000
        0x4_44_44_44_44_44, // each nibble set to 0b0100
        0x2_22_22_22_22_22, // each nibble set to 0b0010
        0x1_11_11_11_11_11, // each nibble set to 0b0001
    ];
    let test_set = wires.clone_with_swaps(swaps);
    for x in TEST_PATTERNS.iter() {
        for y in TEST_PATTERNS.iter() {
            if test_set.test_with_value(*x, *y).is_err() {
                return false;
            }
        }
    }
    true
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let input = input.leak();

    let mut wires: WireKit = WireKit::new();
    let (lits, wfws) = input
        .split_once("\n\n")
        .expect("Failed to split input on blank delimiter line");
    for line in lits.lines() {
        wires
            .literal_wire(line)
            .expect("Failed to insert literal wire");
    }
    for line in wfws.lines() {
        wires
            .wire_from_wires(line)
            .expect("Failed to insert wire-from-wires");
    }
    let mut candidates = find_swaps(&wires);
    candidates.sort();
    candidates.dedup();
    eprintln!("filtered down to the following candidates: {candidates:?}");
    candidates.retain(|swaps| test_candidate(&wires, &swaps[..]));
    eprintln!("further filtered down to the following candidates: {candidates:?}");
    assert_eq!(candidates.len(), 1);
    let mut winner: Vec<String> = candidates
        .pop()
        .expect("already established that it's non-empty")
        .into_iter()
        .flat_map(|(a, b)| [a.to_string(), b.to_string()])
        .collect();
    winner.sort();
    println!("{}", winner.join(","));
}
