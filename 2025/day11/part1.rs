// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 11 Part 1

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy)]
enum MachineEntry<'a> {
    Resolved { route_count: u16 },
    Unresolved { connections: &'a str },
}

fn resolve<'a>(label: &'a str, map: &'a HashMap<&'a str, RefCell<MachineEntry>>) -> u16 {
    let entry = { *map[label].borrow() };
    match entry {
        MachineEntry::Resolved { route_count } => route_count,
        MachineEntry::Unresolved { connections } => {
            let mut route_count = 0;
            for c in connections.split_whitespace() {
                if c == "out" {
                    route_count += 1;
                } else {
                    route_count += resolve(c, map);
                }
            }
            *map[label].borrow_mut() = MachineEntry::Resolved { route_count };
            route_count
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

    println!("{}", resolve("you", &map));
}
