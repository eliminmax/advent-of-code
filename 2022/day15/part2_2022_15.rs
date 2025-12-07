// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 15 Part 2

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Span {
    start: i64,
    end: i64,
}

impl Span {
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

fn parse_input_line(line: &str) -> Sensor {
    fn parse_coords(s: &str) -> Position {
        let (x, y) = s.split_once(", ").unwrap();
        let x = x.strip_prefix("x=").unwrap().parse().unwrap();
        let y = y.strip_prefix("y=").unwrap().parse().unwrap();
        Position { x, y }
    }
    let (sensor, beacon) = line
        .strip_prefix("Sensor at ")
        .unwrap()
        .split_once(": closest beacon is at ")
        .unwrap();
    let pos = parse_coords(sensor);
    let radius = pos.dist(&parse_coords(beacon));
    Sensor { pos, radius }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    const fn dist(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Sensor {
    pos: Position,
    radius: i64,
}

impl Sensor {
    fn in_range(&self, pos: Position) -> bool {
        self.pos.dist(&pos) <= self.radius
    }

    fn span_at(&self, height: i64) -> Option<Span> {
        let remaining_distance = self.radius - (self.pos.y - height).abs();
        if remaining_distance < 1 {
            None
        } else {
            let start = (self.pos.x - remaining_distance).max(0);
            let end = (self.pos.x + remaining_distance).min(4000000);
            if start > 4000000 || end < 0 {
                None
            } else {
                Some(Span { start, end })
            }
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let sensors: Vec<Sensor> = input
        .lines()
        .map(parse_input_line)
        .filter(|s| {
            let x = s.pos.x.clamp(0, 4000000);
            let y = s.pos.y.clamp(0, 4000000);
            s.in_range(Position { x, y })
        })
        .collect();
    for y in 0..=4000000 {
        let spans = normalize_spans(sensors.iter().flat_map(|s| s.span_at(y)).collect());
        if *spans != [Span {start: 0, end: 4000000 }][..] {
            assert_eq!(spans.len(), 2);
            assert_eq!(spans[0].end + 2, spans[1].start);
            println!("{}", (spans[0].end + 1) * 4000000 + y);
            break;
        }
    }

}
