// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 06 Part 2

#[derive(Debug, PartialEq, Clone, Copy)]
struct Span {
    start: usize,
    end: usize,
}

impl Span {
    const fn as_range(&self) -> std::ops::Range<usize> {
        self.start..self.end
    }
    const fn len(&self) -> usize {
        self.end - self.start
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut rows: Vec<&str> = input.lines().collect();
    let mut start = 0;
    let mut spans = Vec::new();
    let op_row = rows.pop().unwrap();
    let num_rows = rows;

    while start < num_rows[0].len() {
        let mut end = start + 1;
        while end < num_rows[0].len() && num_rows.iter().any(|r| r.as_bytes()[end] != b' ') {
            end += 1;
        }
        let range = Span { start, end };
        start = end + 1;
        spans.push(range);
    }

    let mut total = 0;

    for span in spans {
        let mut num_strings: Vec<String> = vec![String::new(); span.len()];
        for row in num_rows.iter() {
            for (i, c) in row[span.as_range()].chars().enumerate() {
                assert!(c.is_ascii(), "Non-ASCII character");
                num_strings[i].push(c);
            }
        }
        let op_str = &op_row[span.as_range()];
        assert!(op_str[1..].trim_end_matches([' ', '\n']).is_empty());
        let nums = num_strings
            .into_iter()
            .map(|ns| ns.trim().parse::<u64>().unwrap());

        total += match op_str.as_bytes()[0] {
            b'+' => nums.sum::<u64>(),
            b'*' => nums.product(),
            b => panic!("invalid opration: {}", b.escape_ascii()),
        };
    }

    println!("{total}");
}
