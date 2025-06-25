// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 10 Part 2

use std::collections::{HashMap, HashSet, VecDeque};

fn gcd(a: i16, b: i16) -> i16 {
    let mut a = a.unsigned_abs();
    let mut b = b.unsigned_abs();

    while a != 0 {
        (a, b) = (b % a, a);
    }

    i16::try_from(b).expect("value <= starting parameters can be cast back to i16")
}

fn simplified_angle(mut x: i16, mut y: i16) -> (i16, i16) {
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

#[derive(Debug)]
struct TargetOrder {
    angle_map: Vec<VecDeque<(i16, i16)>>,
    angle_index: usize,
}

impl From<HashMap<(i16, i16), Vec<(i16, i16)>>> for TargetOrder {
    fn from(map: HashMap<(i16, i16), Vec<(i16, i16)>>) -> Self {
        let mut angle_map: Vec<_> = map.into_iter().collect();

        angle_map.sort_by(|(a, _), (b, _)| f64::partial_cmp(&angle_val(a), &angle_val(b)).unwrap());

        Self {
            angle_map: angle_map
                .into_iter()
                .map(|(_, asteroids)| asteroids.into())
                .collect(),
            angle_index: 0,
        }
    }
}

impl Iterator for TargetOrder {
    type Item = (i16, i16);
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        // pop the first astroid reached at the current angle
        let ret = self.angle_map.get_mut(self.angle_index)?.pop_front();
        // Before returning increment the index, then if it's out-of-bounds, prune empty queues and
        // reset it to zero
        self.angle_index += 1;
        if self.angle_index == self.angle_map.len() {
            self.angle_index = 0;
            self.angle_map.retain(|vd| !vd.is_empty())
        }

        ret
    }
}

fn angle_val(&(x, y): &(i16, i16)) -> f64 {
    // Per Mathematics Stack Extchange, atan2(y, x) would get the angle of the line
    // segment from (x1, y1) to (x2, y2) (https://math.stackexchange.com/a/2587852)
    //
    // The problem there is that the angle is relative to the X axis, with increasing angles going
    // counter-clockwise. The solution there is to invert the X and Y axis.
    //
    // Rust provides `f64::atan2`, which is documented to be a wrapper for the libc `atan2`
    // function on Unix and Windows, and is explicitly documented to be non-deterministic and have
    // an arbitrary precision, which is not ideal, but good enough for a quick-and-dirty solution.
    (-f64::atan2(f64::from(x), f64::from(y)) + std::f64::consts::PI + std::f64::consts::TAU)
        .to_degrees()
        % 360_f64
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;

    // sanity check that angle_val is reasonably precise, even with the non-deterministic precision
    // of `f64::atan2`
    assert_eq!(angle_val(&(0, -1)) as i64, 0, "(0, -1)");
    assert_eq!(angle_val(&(1, 0)) as i64, 90, "(1, 0)");
    assert_eq!(angle_val(&(0, 1)) as i64, 180, "(0, 1)");
    assert_eq!(angle_val(&(-1, 0)) as i64, 270, "(-1, 0)");

    let input = read_to_string(args().nth(1).as_deref().unwrap_or("input")).unwrap();
    let mut astroids = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, cell) in line.bytes().enumerate() {
            if cell == b'#' {
                astroids.insert((i16::try_from(x).unwrap(), i16::try_from(y).unwrap()));
            }
        }
    }

    let (x, y) = *astroids
        .iter()
        .max_by_key(|(x, y)| {
            astroids
                .iter()
                .cloned()
                .filter(|(ax, ay)| ((ax, ay) != (x, y)))
                .map(|(ax, ay)| simplified_angle(ax - x, ay - y))
                .collect::<HashSet<_>>()
                .len()
        })
        .unwrap();
    astroids.remove(&(x, y));

    let mut grouped_by_angle: HashMap<(i16, i16), Vec<(i16, i16)>> = HashMap::new();
    for (ax, ay) in astroids.into_iter() {
        grouped_by_angle
            .entry(simplified_angle(ax - x, ay - y))
            .and_modify(|v| v.push((ax, ay)))
            .or_insert(vec![(ax, ay)]);
    }

    grouped_by_angle
        .values_mut()
        .for_each(|v| v.sort_by_key(|&(ax, ay)| (ax - x).unsigned_abs() + (ay - y).unsigned_abs()));

    println!(
        "{}",
        TargetOrder::from(grouped_by_angle)
            .nth(199)
            .map(|(x, y)| (x * 100) + y)
            .unwrap()
    );
}
