// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 16 Part 2

//! This solution relies on the following chain of logic, which I worked through and double-checked
//! in Python
//!
//! 1. When calculating the Nth digit (using 0-based indexing), its pattern has N leading 0s,
//!    followed by (N+1) 1s, (N+1) 0s, (N+1) -1s, and so on.
//!
//! 2. Following from 1, the first non-0 multiplier when calculating Nth digit will be the
//!    multiplier for N itself, and will be a 1.
//!   * This is because there are N 0s, followed by (N+1) 1s, and (N+1) is always a positive number
//!
//! 3. For a sequence of M digits, any digit index N that's at least M/2 will have a sequence of N
//!    0s, followed by (M-N) 1s.
//!   * this is because the lowest value that satisfies that rule will be M/2 itself, and,
//!     following from 1, the sequence will be M/2 0s followed by (M/2)+1 1s, and because of the
//!     rules of integer division, (M/2)*2 will be either M or M - 1
//!
//! 4. Following from 2 + 3, the final digit of a sequence, regardless of length, will always be left
//!    unchanged after each round
//!
//! 5. Following from 2, 3, and 4, for a sequence of M digits, any digit index N in the range
//!    (M/2)..M can be calculated by taking the previous value of digit N and adding every digit
//!    in the range N+1..M in the current round
//!
//! 6. the input is 650 digits long, and for my input, the first 7 digits, read as a number, are
//!    more than 650*10_000/2 (which is 3_250_000), so there's no need to implement logic for
//!    indexes less than M/2 for an M-digit number - as long as the second half of the digits are
//!    correct. Furthermore, dropping any digits before
//!
//!
//! I can't take full credit for this line of reasoning, as I saw several replies to a handful of
//! Reddit threads asking for help on this problem, and while I only lightly skimmed them, I
//! gleaned enough to decide to look for patterns towards the end of the sequence, and to know that
//! the answer will be in the second half.

fn more_flawed_freq_transmission(digits: &mut [i8]) {
    let mut total: i32 = 0;
    for digit in digits.iter_mut().rev() {
        total += i32::from(*digit);
        *digit = i8::try_from(total % 10).unwrap();
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut digits: Vec<i8> = input
        .trim()
        .bytes()
        .map(|b| (b as char).to_digit(10).unwrap().try_into().unwrap())
        .collect();
    digits = digits.repeat(10_000);

    let offset: usize = input[..(7.min(input.trim().len()))].parse().unwrap();
    assert!(
        offset >= digits.len() / 2,
        "using algorithm that's only accurate for the second half of the digits"
    );

    // as explained above, digits before the offset will have no effect.
    drop(digits.drain(..offset));

    for _ in 0..100 {
        more_flawed_freq_transmission(&mut digits);
    }

    println!(
        "{}",
        digits[..8]
            .iter()
            .map(
                |i| char::from_digit(u32::try_from(*i).expect("number in range 0..=9"), 10)
                    .expect("Valid digit")
            )
            .collect::<String>()
    );
}

#[cfg(test)]
mod equiv_demo {
    use super::more_flawed_freq_transmission;
    // the part 1 implementation
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

    #[test]
    fn digits_123456() {
        let mut digits: [[i8; 6]; 2] = [[1, 2, 3, 4, 5, 6]; 2];
        for _ in 0..100 {
            flawed_freq_transmission(&mut digits[0]);
            more_flawed_freq_transmission(&mut digits[1]);
            assert_eq!(digits[0][3..], digits[1][3..]);
        }
    }
    #[test]
    fn digits_123456654321() {
        let mut digits: [[i8; 12]; 2] = [[1, 2, 3, 4, 5, 6, 6, 5, 4, 3, 2, 1]; 2];
        for _ in 0..100 {
            flawed_freq_transmission(&mut digits[0]);
            more_flawed_freq_transmission(&mut digits[1]);
            assert_eq!(digits[0][6..], digits[1][6..]);
        }
    }
}
