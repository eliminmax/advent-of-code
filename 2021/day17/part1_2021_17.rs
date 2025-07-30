// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 17 Part 1

// I don't know what math to do to do this "right", but the input bounds look small enough that a
// brute force solution works well enough

/// given the range of possible y bounds, returns the range of possible starting y velocities that
/// will result in y falling within the y range 
fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let (x, y) = input.trim().strip_prefix("target area: x=").unwrap().split_once(", y=").unwrap();
    let (x_min, x_max) = x.split_once("..").unwrap();
    let (y_min, y_max) = y.split_once("..").unwrap();

    let x_min: i32 = x_min.parse().unwrap();
    let y_min: i32 = y_min.parse().unwrap();
    let x_max: i32 = x_max.parse().unwrap();
    let y_max: i32 = y_max.parse().unwrap();

    let x_bounds = (x_min.min(-1), x_max.max(1));
    
    let mut best_y: i32 = 0;

    for start_dx in -512..=512 {
        for start_dy in -512..=512 {
            let (mut x, mut y) = (0, 0);
            let (mut dx, mut dy): (i32, i32) = (start_dx, start_dy);
            let mut highest_y = 0;
            loop {
                if x_min <= x && x <= x_max && y_min <= y && y <= y_max {
                    best_y = best_y.max(highest_y);
                    if dy <= 0 {
                        break;
                    }
                }
                x += dx;
                y += dy;
                highest_y = highest_y.max(y);
                dx -= dx.signum();
                dy -= 1;
                // break if out-of-bounds on the x axis
                if x < x_bounds.0 || x > x_bounds.1 {
                    break;
                }

                // break if below the target and descending
                if y < y_min && dy < 0 {
                    break;
                }
            }
        }
    }

    println!("{best_y}");
}
