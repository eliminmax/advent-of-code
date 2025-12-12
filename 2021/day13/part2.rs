// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 13 Part 2

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    /// function to display the point in a terminal that supports ANSI terminal control escape
    /// sequences, using `origin` as the character at the top left corner
    fn render_ansi(&self, origin: Self) {
        print!(
            concat!(
                "\x1b[7m",     // invert foreground and background
                "\x1b[{};{}H", // move cursor to specified {row};{col}
                " ",           // print a literal space with inverted fg and bg
                "\x1b[27m",    // undo invert
            ),
            self.y - origin.y,
            self.x - origin.x,
        );
    }
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

    for fold in folds {
        points.iter_mut().for_each(|i| *i = fold.flip(*i));
        points.sort();
        points.dedup();
    }

    let mut origin = Point {
        x: i32::MAX,
        y: i32::MAX,
    };

    let mut max_row = i32::MIN;
    for &Point { x, y } in points.iter() {
        origin.x = origin.x.min(x);
        origin.y = origin.y.min(y);
        max_row = max_row.max(y);
    }

    // adjustment to compensate for 1-based indexing of character locations in PTY output
    origin.x -= 1;
    origin.y -= 1;
    max_row += 1;

    // If both stdout and stderr are the same pty, this will be cleared, but if either is
    // redirected, then it will be available for inspection in debug builds.
    if cfg!(debug_assertions) {
        eprintln!("origin = {origin:?};");
        eprintln!("max_row = {max_row:?};");
    }

    print!(concat!(
        // move the cursor to 0, 0
        "\x1b[H", // clear from cursor to end of screen
        "\x1b[J"
    ));
    for point in points {
        point.render_ansi(origin);
    }

    // finally, move the cursor to the line after max_row
    println!("\x1b[{max_row}H");
}
