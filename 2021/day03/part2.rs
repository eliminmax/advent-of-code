// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 3 Part 2

use std::env::args;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let vals: Vec<u16> = input
        .lines()
        .map(|s| u16::from_str_radix(s, 2).expect("Failed to parse as binary integer string"))
        .collect();
    let bits: u32 = vals
        .iter()
        .map(|i| 16 - i.leading_zeros())
        .max()
        .expect("vals empty");
    let cutoff: usize = vals.len() / 2;
    let leading_mask = 1 << (bits - 1);
    let oxy_bit = if vals.iter().filter(|i| (*i & leading_mask) != 0).count() >= cutoff {
        leading_mask
    } else {
        0
    };
    let (mut oxy_candidates, mut co2_candidates): (Vec<u16>, Vec<u16>) =
        vals.into_iter().partition(|n| n & leading_mask == oxy_bit);

    let mut oxy: Option<u16> = None;
    'oxy_loop: for bit in (0..(bits - 1)).rev() {
        let mask = 1 << bit;
        let cutoff = (oxy_candidates.len() as f64 / 2.0).ceil() as usize;
        if oxy_candidates.iter().filter(|i| (*i & mask) != 0).count() >= cutoff {
            oxy_candidates.retain(|i| (*i & mask) != 0);
        } else {
            oxy_candidates.retain(|i| (*i & mask) == 0);
        };
        if oxy_candidates.len() <= 1 {
            oxy = oxy_candidates.pop();
            break 'oxy_loop;
        }
    }
    let oxy: u16 = oxy.expect("Failed to find oxy value");

    let mut co2: Option<u16> = None;
    'co2_loop: for bit in (0..(bits - 1)).rev() {
        let mask = 1 << bit;
        let cutoff = (co2_candidates.len() as f64 / 2.0).ceil() as usize;
        if co2_candidates.iter().filter(|i| (*i & mask) != 0).count() >= cutoff {
            co2_candidates.retain(|i| (*i & mask) == 0);
        } else {
            co2_candidates.retain(|i| (*i & mask) != 0);
        }
        if co2_candidates.len() <= 1 {
            co2 = co2_candidates.pop();
            break 'co2_loop;
        }
    }
    let co2: u16 = co2.expect("Failed to find co2 value");
    println!("{}", co2 as u32 * oxy as u32);
}
