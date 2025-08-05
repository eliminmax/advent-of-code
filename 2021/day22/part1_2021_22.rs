// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 22 Part 1

#[derive(Debug, PartialEq, Clone)]
struct Step {
    state: bool,
    x_bounds: (i32, i32),
    y_bounds: (i32, i32),
    z_bounds: (i32, i32),
}

impl Step {
    fn panicky_parse(s: &str) -> Self {
        let (state, coords) = s.split_once(' ').unwrap();
        let (x, y, z) = coords
            .split_once(',')
            .and_then(|(x, yz)| yz.split_once(',').map(|(y, z)| (x, y, z)))
            .unwrap();
        let x_bounds = x
            .strip_prefix("x=")
            .and_then(|x| x.split_once(".."))
            .map(|(lo, hi)| (lo.parse().unwrap(), hi.parse().unwrap()))
            .unwrap();
        let y_bounds = y
            .strip_prefix("y=")
            .and_then(|y| y.split_once(".."))
            .map(|(lo, hi)| (lo.parse().unwrap(), hi.parse().unwrap()))
            .unwrap();
        let z_bounds = z
            .strip_prefix("z=")
            .and_then(|z| z.split_once(".."))
            .map(|(lo, hi)| (lo.parse().unwrap(), hi.parse().unwrap()))
            .unwrap();
        let state = match state {
            "on" => true,
            "off" => false,
            bad => panic!("bad state: {bad:?}"),
        };
        Self {
            state,
            x_bounds,
            y_bounds,
            z_bounds,
        }
    }
    fn apply(&self, x: i32, y: i32, z: i32) -> Option<bool> {
        if (self.x_bounds.0..=self.x_bounds.1).contains(&x)
            && (self.y_bounds.0..=self.y_bounds.1).contains(&y)
            && (self.z_bounds.0..=self.z_bounds.1).contains(&z)
        {
            Some(self.state)
        } else {
            None
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let steps: Vec<Step> = input.lines().map(Step::panicky_parse).rev().collect();
    let mut count: u32 = 0;

    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                if steps.iter().filter_map(|s| s.apply(x, y, z)).next().unwrap_or(false) {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}
