// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let lines: Vec<&str> = input.lines().collect();
    for block in lines.chunks(18) {
        let p0: i64 = block[4].strip_prefix("div z ").unwrap().parse().unwrap();
        let p1: i64 = block[5].strip_prefix("add x ").unwrap().parse().unwrap();
        let p2: i64 = block[15].strip_prefix("add y ").unwrap().parse().unwrap();
        println!("{p0}\t{p1}\t{p2}");
    }
}
