// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 14 Part 2

fn main() {
    // need to parse input as digits, so doing this instead
    let elf_target: Vec<u8> = include_bytes!("input")
        .trim_ascii_end()
        .iter()
        .cloned()
        .map(|b| b - b'0')
        .collect();

    let mut scores: Vec<u8> = Vec::from([3, 7]);
    let mut elf_alice = 0;
    let mut elf_bob = 1;

    while !scores.ends_with(&elf_target[..]) {
        let new_score = scores[elf_alice] + scores[elf_bob];
        match new_score {
            0..=9 => scores.push(new_score),
            10..=99 => {
                scores.push(new_score / 10);
                if scores.ends_with(&elf_target[..]) {
                    break;
                }
                scores.push(new_score % 10);
            }
            100.. => panic!("cake too good!"),
        }
        elf_alice += usize::from(scores[elf_alice]) + 1;
        elf_alice %= scores.len();
        elf_bob += usize::from(scores[elf_bob]) + 1;
        elf_bob %= scores.len();
    }
    println!("{}", scores.len() - elf_target.len());
}
