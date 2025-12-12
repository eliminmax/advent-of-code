// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 14 Part 1

#[derive(Debug, PartialEq, Clone, Copy)]
struct DockInt(u64);

impl DockInt {
    const MAX: Self = Self(0xfffffffff);
    fn apply_mask(&mut self, Mask { keep, zero, one }: Mask) {
        debug_assert_eq!(keep ^ zero ^ one, Self::MAX.0);
        self.0 &= (!zero) & Self::MAX.0;
        self.0 |= one;
        debug_assert_eq!(self.0 & !Self::MAX.0, 0);
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Mask {
    keep: u64,
    zero: u64,
    one: u64,
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut mask = Mask {
        keep: 0xfffffffff,
        zero: 0,
        one: 0,
    };
    let mut dock_mem: Vec<DockInt> = [DockInt(0)].repeat(65536);
    for line in input.lines() {
        if let Ok(new_mask) = line.parse() {
            mask = new_mask;
        } else {
            let (mem_addr, val) = line.trim().split_once("] = ").unwrap();
            let mut val = DockInt(val.parse().unwrap());
            let mem_addr: usize = mem_addr.strip_prefix("mem[").unwrap().parse().unwrap();
            val.apply_mask(mask);
            dock_mem[mem_addr] = val;
        }
    }

    let total: u64 = dock_mem.into_iter().map(|DockInt(i)| i).sum();
    println!("{total}");
}

impl std::str::FromStr for Mask {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed_bytes = s.trim().strip_prefix("mask = ").ok_or(s)?.as_bytes();
        let mut keep = 0;
        let mut zero = 0;
        let mut one = 0;

        for (bit, char_index) in (0..36).rev().enumerate() {
            match trimmed_bytes[char_index] {
                b'0' => zero |= 1 << bit,
                b'1' => one |= 1 << bit,
                b'X' => keep |= 1 << bit,
                _ => return Err(s.to_string()),
            }
        }

        Ok(Self { keep, zero, one })
    }
}
