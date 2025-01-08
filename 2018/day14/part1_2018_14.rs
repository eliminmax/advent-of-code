// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 14 Part 1

const ELF_TARGET: usize = include!("input");
const NEED_TO_MAKE: usize = ELF_TARGET + 10;

fn main() {
    let mut scores: Vec<u8> = Vec::with_capacity(NEED_TO_MAKE + 1);
    scores.extend_from_slice(&[3, 7]);
    let mut elf_alice = 0;
    let mut elf_bob = 1;
    while scores.len() < NEED_TO_MAKE {
        let new_score = scores[elf_alice] + scores[elf_bob];
        match new_score {
            0..=9 => scores.push(new_score),
            10..=99 => {
                scores.push(new_score / 10);
                scores.push(new_score % 10);
            }
            100.. => panic!("cake too good!"),
        }
        elf_alice += usize::from(scores[elf_alice]) + 1;
        elf_alice %= scores.len();
        elf_bob += usize::from(scores[elf_bob]) + 1;
        elf_bob %= scores.len();
    }
    let resolve_string: String = scores[ELF_TARGET..(ELF_TARGET + 10)]
        .iter()
        .map(|b| {
            char::from_digit((*b).into(), 10)
                .unwrap_or_else(|| unreachable!("scores are at most 9"))
        })
        .collect();
    println!("{resolve_string}");
}
