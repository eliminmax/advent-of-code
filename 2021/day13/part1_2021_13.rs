// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 13 Part 1

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Fold {
    X(i32),
    Y(i32),
}

impl Fold {
    const fn flip(self, p: Point) -> Point {
        const fn flip_num(val: i32, pivot_point: i32) -> i32 {
            if val > pivot_point {
                (pivot_point * 2) - val
            } else {
                val
            }
        }

        match self {
            Fold::X(pivot) => Point {
                x: flip_num(p.x, pivot),
                ..p
            },
            Fold::Y(pivot) => Point {
                y: flip_num(p.y, pivot),
                ..p
            },
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let (input_points, input_folds) = input.split_once("\n\n").unwrap();

    let mut points = Vec::new();

    for (x, y) in input_points
        .lines()
        .map(|line| line.split_once(',').unwrap())
    {
        let x = x.parse().unwrap();
        let y = y.parse().unwrap();
        points.push(Point { x, y });
    }

    let mut folds = Vec::new();

    for fold in input_folds.lines() {
        let (axis, coord) = fold
            .strip_prefix("fold along ")
            .unwrap()
            .split_once('=')
            .unwrap();
        match axis {
            "x" => folds.push(Fold::X(coord.parse().unwrap())),
            "y" => folds.push(Fold::Y(coord.parse().unwrap())),
            s => panic!("invalid axis: {s:?}"),
        }
    }

    points.iter_mut().for_each(|i| *i = folds[0].flip(*i));
    points.sort();
    points.dedup();
    println!("{}", points.len());
}
