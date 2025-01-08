// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 11 Part 2

const SERIAL_NO: u32 = include!("input");

fn compute_power(x: u32, y: u32) -> i32 {
    use std::convert::TryInto;
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += SERIAL_NO;
    power *= rack_id;
    power %= 1000;
    power /= 100;
    let power: i32 = power
        .try_into()
        .unwrap_or_else(|_| unreachable!("power is in the range 0..=9 at this point"));
    power - 5
}

fn main() {
    let mut highest_power = i32::MIN;
    let mut leading_corner: Option<(u32, u32, u32)> = None;
    let corners = (1u32..300).flat_map(|y| (1u32..300).map(move |x| (x, y)));
    for (x_off, y_off) in corners {
        for size in 0..(300 - x_off.max(y_off)) {
            let mut power: i32 = 0;
            for y in y_off..(y_off + size) {
                for x in x_off..(x_off + size) {
                    power += compute_power(x, y);
                }
            }
            if power > highest_power {
                highest_power = power;
                leading_corner = Some((x_off, y_off, size));
            }
        }
    }
    let leading_corner =
        leading_corner.unwrap_or_else(|| unreachable!("There must be at least one area"));
    println!(
        "{},{},{}",
        leading_corner.0, leading_corner.1, leading_corner.2
    );
}
