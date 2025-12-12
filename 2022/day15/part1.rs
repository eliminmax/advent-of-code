// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 15 Part 1

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Span {
    start: i64,
    end: i64,
}

impl Span {
    fn len(&self) -> u64 {
        self.start.abs_diff(self.end) + 1
    }
    fn contains(&self, n: i64) -> bool {
        n >= self.start && n <= self.end
    }
}

/// sort `ranges`, and merge overlapping ranges
fn normalize_spans(mut spans: VecDeque<Span>) -> Box<[Span]> {
    spans.make_contiguous().sort();
    let mut merged_ranges = Vec::with_capacity(spans.len());
    while let Some(mut range) = spans.pop_front() {
        while spans.front().is_some_and(|next| range.contains(next.start)) {
            let next = spans.pop_front().unwrap();
            range.end = range.end.max(next.end);
        }
        merged_ranges.push(range);
    }
    merged_ranges.into_boxed_slice()
}

type Coord = (i64, i64);
const fn manhattan_dist(a: Coord, b: Coord) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_input_line<const Y: i64>(line: &str) -> Option<Span> {
    fn parse_coords(s: &str) -> Coord {
        let (x, y) = s.split_once(", ").unwrap();
        let x = x.strip_prefix("x=").unwrap().parse().unwrap();
        let y = y.strip_prefix("y=").unwrap().parse().unwrap();
        (x, y)
    }
    let (sensor, beacon) = line
        .strip_prefix("Sensor at ")
        .unwrap()
        .split_once(": closest beacon is at ")
        .unwrap();
    let sensor = parse_coords(sensor);
    let beacon = parse_coords(beacon);
    let scan_radius = manhattan_dist(sensor, beacon);
    let distance_from_y = manhattan_dist(sensor, (sensor.0, Y));
    let remaining_distance = scan_radius - distance_from_y;
    if remaining_distance < 1 {
        None
    } else {
        let mut start = sensor.0 - remaining_distance;
        let mut end = sensor.0 + remaining_distance;
        if (start, Y) == beacon {
            start += 1;
        }
        if (end, Y) == beacon {
            end -= 1;
        }
        Some(Span { start, end })
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let spans = normalize_spans(
        input
            .lines()
            .flat_map(parse_input_line::<2000000>)
            .collect(),
    );
    println!("{}", spans.iter().map(Span::len).sum::<u64>());
}
