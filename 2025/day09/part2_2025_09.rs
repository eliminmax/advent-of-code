// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2025 Day 09 Part 2

// using u32s would result in overflow when calculating the maximum rectangle size
type Point = (u64, u64);

#[derive(Debug, PartialEq, Clone, Copy)]
struct Segment {
    x0: u64,
    x1: u64,
    y0: u64,
    y1: u64,
}

macro_rules! sorted {
    [$a: expr, $b: expr] => {{ if $a > $b { [$b, $a] } else { [$a, $b] } }}
}
impl Segment {
    const fn new(a: Point, b: Point) -> Self {
        let (x0, x1) = if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) };
        let (y0, y1) = if a.1 < b.1 { (a.1, b.1) } else { (b.1, a.1) };
        assert!(a.0 == b.0 || a.1 == b.1);
        Self { x0, x1, y0, y1 }
    }

    fn intersects_rect(&self, &(ax, ay): &Point, &(bx, by): &Point) -> bool {
        let [x0, x1] = sorted![ax, bx];
        let [y0, y1] = sorted![ay, by];
        if self.x0 == self.x1 {
            x0 < self.x0
                && self.x0 < x1
                && ((self.y0 <= y0 && y0 < self.y1) || (self.y0 < y1 && y1 <= self.y1))
        } else {
            debug_assert!(self.y0 == self.y1);
            y0 < self.y0
                && self.y0 < y1
                && ((self.x0 <= x0 && x0 < self.x1) || (self.x0 < x1 && x1 <= self.x1))
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let red_tiles: Vec<Point> = input
        .lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();

    let mut bounds = ((u64::MAX, u64::MAX), (0, 0));
    for &(x, y) in red_tiles.iter() {
        bounds.0 .0 = bounds.0 .0.min(x);
        bounds.1 .0 = bounds.1 .0.max(x);
        bounds.0 .1 = bounds.0 .1.min(y);
        bounds.1 .1 = bounds.1 .1.max(y);
    }
    let mut segments = Vec::with_capacity(red_tiles.len());
    for w in red_tiles.windows(2) {
        segments.push(Segment::new(w[0], w[1]));
    }
    segments.push(Segment::new(*red_tiles.last().unwrap(), red_tiles[0]));

    let mut max_size = 0;
    for (i, a) in red_tiles.iter().enumerate() {
        for b in red_tiles[i + 1..].iter() {
            let [x0, x1] = sorted![a.0, b.0];
            let [y0, y1] = sorted![a.1, b.1];
            if segments.iter().any(|s| s.intersects_rect(a, b)) {
                continue;
            }

            let size = ((x1 - x0) + 1) * ((y1 - y0) + 1);
            if size > max_size {
                max_size = size;
            }
        }
    }
    println!("{max_size}");
}
