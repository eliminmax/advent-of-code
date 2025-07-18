// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 09 Part 2

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let nums: Vec<u64> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut invalid: Option<u64> = None;
    'main_loop: for window in nums.windows(26) {
        let candidates: Vec<u64> = window[..25]
            .iter()
            .cloned()
            .filter(|i| *i < window[25])
            .collect();
        for i in 0..candidates.len() {
            for j in (i + 1)..candidates.len() {
                if candidates[i] + candidates[j] == window[25] {
                    continue 'main_loop;
                }
            }
        }
        invalid = Some(window[25]);
        break 'main_loop;
    }

    let invalid = invalid.unwrap();
    'main_loop: for i in 0..nums.len() {
        let mut sum = 0;
        for j in i..nums.len() {
            sum += nums[j];
            if sum > invalid {
                continue 'main_loop;
            }
            if sum == invalid {
                println!(
                    "{}",
                    nums[i..=j].iter().min().unwrap() + nums[i..=j].iter().max().unwrap()
                );
                break 'main_loop;
            }
        }
    }
}
