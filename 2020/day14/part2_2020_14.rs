// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 14 Part 2
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct DockAddr(u64);

impl DockAddr {
    fn with_mask(mut self, Mask { floating, ones, .. }: Mask) -> impl Iterator<Item = Self> {
        self.0 |= ones;
        self.0 &= !floating;
        let mut variants = vec![self];

        let floating_bits: Vec<u32> = (0..36).filter(|i| floating & 1 << i != 0).collect();

        for bit_index in floating_bits {
            let with_bit_set = variants
                .clone()
                .into_iter()
                .map(|Self(i)| Self(i | (1 << bit_index)));
            variants.extend(with_bit_set);
        }

        variants.into_iter()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Mask {
    floating: u64,
    keep: u64,
    ones: u64,
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut mask = Mask {
        floating: 0xfffffffff,
        keep: 0,
        ones: 0,
    };
    let mut dock_mem: HashMap<DockAddr, u32> = HashMap::new();
    for line in input.lines() {
        if let Ok(new_mask) = line.parse() {
            mask = new_mask;
        } else {
            let (mem_addr, val) = line.trim().split_once("] = ").unwrap();
            let val: u32 = val.parse().unwrap();
            let mem_addr = DockAddr(mem_addr.strip_prefix("mem[").unwrap().parse().unwrap());
            for addr in mem_addr.with_mask(mask) {
                dock_mem.insert(addr, val);
            }
        }
    }

    let total: u64 = dock_mem.into_values().map(u64::from).sum();
    println!("{total}");
}

impl std::str::FromStr for Mask {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trimmed_bytes = s.trim().strip_prefix("mask = ").ok_or(s)?.as_bytes();
        let mut floating = 0;
        let mut keep = 0;
        let mut ones = 0;

        for (bit, char_index) in (0..36).rev().enumerate() {
            match trimmed_bytes[char_index] {
                b'0' => keep |= 1 << bit,
                b'1' => ones |= 1 << bit,
                b'X' => floating |= 1 << bit,
                _ => return Err(s.to_string()),
            }
        }

        Ok(Self {
            floating,
            keep,
            ones,
        })
    }
}
