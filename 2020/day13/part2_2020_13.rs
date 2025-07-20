// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 13 Part 2

fn naive_mmi(mut a: u64, modulus: u64) -> Option<u64> {
    a %= modulus;

    (0..modulus).find(|i| (i * a) % modulus == 1)
}

fn crt(pairs: &[(u64, u64)]) -> Option<u64> {
    let prod: u64 = pairs.iter().cloned().map(|i| i.1).product();

    let mut sum = 0;
    for (rem, modulus) in pairs {
        let val = prod / modulus;
        sum += rem * naive_mmi(val, *modulus)? * val;
        sum %= prod;
    }
    Some(sum)
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut lines = input.lines();
    let _: u64 = lines.next().unwrap().parse().unwrap();

    let bus_times: Vec<(u64, u64)> = lines
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter_map(|(i, bus)| {
            bus.parse()
                .map(|bus| {
                    let i = u64::try_from(i).unwrap();
                    ((i.next_multiple_of(bus) - i) % bus, bus)
                })
                .ok()
        })
        .collect();

    assert!(lines.next().is_none());
    println!("{}", crt(&bus_times).unwrap());
}


