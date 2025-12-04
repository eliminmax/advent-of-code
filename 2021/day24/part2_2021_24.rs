// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 24 Part 2


const fn digit_pair(offset: i8) -> (i8, i8) {
    assert!(matches!(offset, -8..=8));
    let (mut a, mut b) = if offset <= 0 {
        (9, 9 + offset)
    } else {
        (9 - offset, 9)
    };
    while a > 1 && b > 1 {
        a -=  1;
        b -=  1;
    }
    (a, b)
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let blocks: Vec<&str> = input.lines().collect();
    let mut stack: Vec<(usize, i8)> = Vec::with_capacity(14);
    let mut digits: [i8; 14] = [0; 14];
    for (i, block) in blocks.chunks(18).enumerate() {
        if block[4].trim() == "div z 1" {
            stack.push((i, block[15].trim().strip_prefix("add y ").unwrap().parse().unwrap()));
        } else {
            let (paired_index, off_a) = stack.pop().unwrap();
            let off_b: i8 = block[5].trim().strip_prefix("add x ").unwrap().parse().unwrap();
            let (a, b) = digit_pair(off_a + off_b);
            digits[paired_index] = a;
            digits[i] = b;
        }
    }

    if digits.contains(&0) { panic!("Digit left unset: {digits:?}"); }

    for digit in digits { print!("{digit}") };
    println!()
    
}

#[cfg(test)]
#[test]
fn proper_relations() {
    for offset in -8..=8 {
        let (a, b) = digit_pair(offset);
        assert_eq!(b - a, offset);
        assert!(matches!(a, 1..=9));
        assert!(matches!(b, 1..=9));
        if offset > 0 {
            assert!(a == 1)
        } else {
            assert!(b == 1)
        }
    }
}
