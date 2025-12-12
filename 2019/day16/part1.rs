// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 16 Part 1

fn flawed_freq_transmission(digits: &mut [i8]) {
    let pattern: [i16; 4] = [0, 1, 0, -1];
    let old_digits: Vec<i8> = digits.to_owned();
    for (round, digit) in digits.iter_mut().enumerate() {
        let mut nums = pattern
            .into_iter()
            .flat_map(|i| [i].repeat(round + 1))
            .cycle()
            .skip(1);
        *digit = (old_digits
            .iter()
            .cloned()
            .fold(0, |acc, i| (acc + (i16::from(i) * nums.next().unwrap())))
            .abs()
            % 10) as i8;
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut digits: Vec<i8> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect();

    for _ in 0..100 {
        flawed_freq_transmission(&mut digits);
    }
    println!(
        "{}",
        digits[0..8]
            .iter()
            .map(|i| (i.unsigned_abs() + b'0') as char)
            .collect::<String>()
    );
}
