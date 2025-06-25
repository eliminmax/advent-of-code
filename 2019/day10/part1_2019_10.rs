// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 10 Part 1

use std::collections::HashSet;

fn gcd(a: i16, b: i16) -> i16 {
    let mut a = a.unsigned_abs();
    let mut b = b.unsigned_abs();

    while a != 0 {
        (a, b) = (b % a, a);
    }

    i16::try_from(b).expect("value <= starting parameters can be cast back to i16")
}

fn simplified_val(mut x: i16, mut y: i16) -> (i16, i16) {
    loop {
        let factor = gcd(x, y);
        if factor == 1 {
            break;
        }
        x /= factor;
        y /= factor;
    }
    (x, y)
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).as_deref().unwrap_or("input")).unwrap();
    let mut astroids = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.bytes().enumerate() {
            if cell == b'#' {
                astroids.insert((i16::try_from(x).unwrap(), i16::try_from(y).unwrap()));
            }
        }
    }

    let max_reached = astroids
        .iter()
        .map(|(x, y)| {
            astroids
                .iter()
                .cloned()
                .filter(|(ax, ay)| ((ax, ay) != (x, y)))
                .map(|(ax, ay)| simplified_val(x - ax, y - ay))
                .collect::<HashSet<_>>()
                .len()
        })
        .max()
        .unwrap();
    println!("{max_reached}");
}
