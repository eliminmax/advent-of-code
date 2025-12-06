// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 06 Part 1

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut rows = input.lines().map(|r| r.split_ascii_whitespace());
    let mut problems: Vec<Vec<&str>> = rows.next().unwrap().map(|i| vec![i]).collect();
    for row in rows {
        for (i, item) in row.enumerate() {
            problems[i].push(item);
        }
    }

    println!("{}", {
        problems
            .into_iter()
            .map(|mut problem| {
                let op = problem.pop().unwrap();
                let nums = problem.into_iter().map(|i| i.parse::<u64>().expect("valid input"));
                match op {
                    "*" => nums.product::<u64>(),
                    "+" => nums.sum::<u64>(),
                    invalid => panic!("invalid operation: {invalid}"),
                }
            })
            .sum::<u64>()
    });
}
