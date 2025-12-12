// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 11 Part 2

use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::AddAssign;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct ResolvedRouteCount {
    /// number of routes that pass through both "fft" and "dac"
    full_routes: u64,
    /// number of routes that pass through "dac" but not "fft"
    dac_routes: u64,
    /// number of routes that pass through "fft" but not "dac"
    fft_routes: u64,
    /// "number of routes that reach "out" without passing through either "dac" or "fft"
    out_routes: u64,
}

impl AddAssign for ResolvedRouteCount {
    fn add_assign(&mut self, rhs: Self) {
        self.full_routes += rhs.full_routes;
        self.dac_routes += rhs.dac_routes;
        self.fft_routes += rhs.fft_routes;
        self.out_routes += rhs.out_routes;
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum MachineEntry<'a> {
    Resolved(ResolvedRouteCount),
    Unresolved { connections: &'a str },
}

fn resolve<'a>(
    label: &'a str,
    map: &'a HashMap<&'a str, RefCell<MachineEntry>>,
) -> ResolvedRouteCount {
    let entry = { *map[label].borrow() };
    match entry {
        MachineEntry::Resolved(rrc) => rrc,
        MachineEntry::Unresolved { connections } => {
            let mut rrc = ResolvedRouteCount::default();
            for c in connections.split_whitespace() {
                if c == "out" {
                    rrc.out_routes += 1;
                } else {
                    rrc += resolve(c, map);
                    if label == "dac" {
                        rrc.full_routes += rrc.fft_routes;
                        rrc.fft_routes = 0;
                        rrc.dac_routes += rrc.out_routes;
                        rrc.out_routes = 0;
                    } else if label == "fft" {
                        rrc.full_routes += rrc.dac_routes;
                        rrc.dac_routes = 0;
                        rrc.fft_routes += rrc.out_routes;
                        rrc.out_routes = 0;
                    }
                }
            }
            *map[label].borrow_mut() = MachineEntry::Resolved(rrc);
            rrc
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut map = HashMap::new();

    for line in input.lines() {
        let (label, connections) = line.split_once(": ").unwrap();
        map.insert(
            label,
            RefCell::new(MachineEntry::Unresolved { connections }),
        );
    }

    println!("{}", resolve("svr", &map).full_routes);
}
