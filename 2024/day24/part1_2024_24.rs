// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 24 Part 1

use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug)]
enum WireKitError {
    UnknownWire,
    Unparsable,
    Redefinition,
    DependencyLoop,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum WireOp {
    And,
    Xor,
    Or,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Wire<'a> {
    Literal(bool),
    FromWires(&'a str, &'a str, WireOp),
}

#[derive(Debug, PartialEq)]
/// A thin wrapper around the inner HashMap which adds methods for parsing wires from strings and
/// resolving wire values. Transparently derefs into its inner HashMap.
struct WireKit<'a>(HashMap<&'a str, Wire<'a>>);

impl<'a> std::ops::Deref for WireKit<'a> {
    type Target = HashMap<&'a str, Wire<'a>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for WireKit<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl WireKit<'_> {
    fn bound_to<'a>(_binding: &'a str) -> WireKit<'a> {
        WireKit::<'a>(HashMap::<&'a str, Wire<'a>>::new())
    }
}

impl<'a> WireKit<'a> {
    fn insert_wire(&mut self, wire_id: &'a str, wire: Wire<'a>) -> Result<(), WireKitError> {
        if self.insert(wire_id, wire).is_none() {
            Ok(())
        } else {
            Err(WireKitError::Redefinition)
        }
    }

    /// Load a literal wire of the form `x00: 1`
    fn literal_wire(&mut self, wire_str: &'a str) -> Result<(), WireKitError> {
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
    fn wire_from_wires(&mut self, wire_str: &'a str) -> Result<(), WireKitError> {
        let words: Vec<&str> = wire_str.split_whitespace().collect();
        if let [wire_a, op, wire_b, "->", dest_wire] = &words[..] {
            let op = match *op {
                "AND" => Ok(WireOp::And),
                "XOR" => Ok(WireOp::Xor),
                "OR" => Ok(WireOp::Or),
                _ => Err(WireKitError::Unparsable),
            }?;
            self.insert_wire(dest_wire, Wire::FromWires(wire_a, wire_b, op))
        } else {
            Err(WireKitError::Unparsable)
        }
    }

    /// try to resolve wire value and return it.
    /// * If already resolved (i.e. `matches!(&self[wire_id], Wire::Literal(_))`, returns `Ok(true)`
    /// * Otherwise:
    ///     * if wire_a and wire_b are both resolved, sets `&self[wire_id]` to the resolved signal,
    ///       then returns `Ok(true)`
    ///     * if not enough information is available, it returns `Ok(false)`
    ///     * if wire_a or wire_b are not found within self, returns
    ///       `Err(WireKitError::UnknownWire)`
    fn try_resolve(&mut self, wire_id: &'a str) -> Result<bool, WireKitError> {
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

    /// resolve all wires, replacing any Wire::FromWires with a Wire::Literal
    fn resolve_all(&mut self) -> Result<(), WireKitError> {
        use std::collections::VecDeque;
        let mut to_resolve: VecDeque<&str> = self.keys().cloned().collect();
        let mut unresolved_count: usize = 0;
        while let Some(wire_id) = to_resolve.pop_front() {
            if self.contains_key(wire_id) {
                if !self.try_resolve(wire_id)? {
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
        Ok(())
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut wires: WireKit = WireKit::bound_to(&input);
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
    wires.resolve_all().expect("Failed to resolve wire values");
    let mut zwires: Vec<&str> = wires
        .keys()
        .filter(|&k| k.starts_with('z'))
        .cloned()
        .collect();
    zwires.sort();
    let mut result: u64 = 0;
    for (shift, wire) in zwires.into_iter().enumerate() {
        if let Wire::Literal(bit) = wires[wire] {
            result |= (bit as u64) << shift;
        } else {
            unreachable!("All wires have already been resolved to Literals by this point");
        }
    }
    println!("{result}");
}
