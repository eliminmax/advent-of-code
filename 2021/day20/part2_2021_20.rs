// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 20 Part 2

use std::collections::HashSet;

// The challenging part of this, for me, is that for my input, a pixel 3x3 empty area activates,
// while a pixel in a 3x3 fully active area deactivates, so to avoid needing infinite time and
// space, I need to track what the "default" pixel state is, and if

#[derive(PartialEq, Clone)]
struct EnhancementAlg([bool; 512]);

impl EnhancementAlg {
    fn panicky_parse(s: &str) -> Self {
        let s = s.as_bytes();
        Self(core::array::from_fn(|i| s[i] == b'#'))
    }
}

struct Picture {
    default_pix: bool,
    inverted: HashSet<(i32, i32)>,
}

impl Picture {
    fn apply_alg(&mut self, alg: &EnhancementAlg) {
        // if the default case comes alive, to avoid needing to check and store infinite positions,
        // instead flip both the default and inputs
        let mut next_gen: Vec<(i32, i32)> = Vec::with_capacity(self.inverted.len() * 2);
        let mut to_check = self.inverted.clone();
        let next_default = alg[[[self.default_pix; 3]; 3]];
        for &(r, c) in self.inverted.iter() {
            to_check.extend(
                ((r - 2)..=(r + 2)).flat_map(move |nr| ((c - 2)..=(c + 2)).map(move |nc| (nr, nc))),
            );
        }

        for (r, c) in to_check {
            macro_rules! bit {
                ($i: ident, -) => {{ $i - 1 }};
                ($i: ident, =) => {{ $i }};
                ($i: ident, +) => {{ $i + 1 }};
                ($r: tt $c: tt) => {{ self.inverted.contains(&(bit!(r, $r), bit!(c, $c))) ^ self.default_pix }};
            }

            let index_arr = [
                [bit!(- -), bit!(- =), bit!(- +)],
                [bit!(= -), bit!(= =), bit!(= +)],
                [bit!(+ -), bit!(+ =), bit!(+ +)],
            ];

            if alg[index_arr] != next_default {
                next_gen.push((r, c));
            }
        }
        self.default_pix = next_default;
        self.inverted.clear();
        self.inverted.extend(next_gen);
    }
    fn inverted_len(&self) -> usize {
        self.inverted.len()
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let (alg, pic) = input.split_once("\n\n").unwrap();
    let alg = EnhancementAlg::panicky_parse(alg);

    let mut r = 0;
    let mut c = 0;

    let mut inverted = HashSet::with_capacity(pic.len());
    for chr in pic.bytes() {
        match chr {
            b'\n' => {
                r += 1;
                c = 0;
            }
            b'.' => c += 1,
            b'#' => {
                inverted.insert((r, c));
                c += 1;
            }
            bad => panic!("invalid picture character: {}", bad.escape_ascii()),
        }
    }

    let mut pic = Picture {
        inverted,
        default_pix: false,
    };

    for _ in 0..50 {
        pic.apply_alg(&alg);
    }
    println!("{}", pic.inverted_len());
}

impl std::ops::Index<[[bool; 3]; 3]> for EnhancementAlg {
    type Output = bool;

    fn index(&self, index: [[bool; 3]; 3]) -> &Self::Output {
        let mut bit_index = 0;
        for row in index {
            for bit in row {
                bit_index <<= 1;
                bit_index |= usize::from(bit)
            }
        }
        self.0.index(bit_index)
    }
}

impl std::fmt::Debug for EnhancementAlg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .cloned()
                .map(|i| if i { '1' } else { '0' })
                .collect::<String>()
        )
    }
}
#[cfg(test)]
mod tests {
    use super::EnhancementAlg;

    #[test]
    fn alg_parse() {
        // test pattern generated algorithmically such that each index i is active is on if and
        // only if i is a power of 2.
        let test_pat = concat!(
            ".##.#...#.......#...............#...............................",
            "#...............................................................",
            "#...............................................................",
            "................................................................",
            "#...............................................................",
            "................................................................",
            "................................................................",
            "................................................................"
        );

        let test_alg = EnhancementAlg::panicky_parse(test_pat);
        for i in 0_usize..512 {
            macro_rules! bit_check {
                ($bit: literal) => {{ i & (1 << $bit) != 0 }};
            }
            let unpacked_index = [
                [bit_check!(8), bit_check!(7), bit_check!(6)],
                [bit_check!(5), bit_check!(4), bit_check!(3)],
                [bit_check!(2), bit_check!(1), bit_check!(0)],
            ];
            assert_eq!(
                test_alg[unpacked_index],
                i.is_power_of_two(),
                "{i}, {unpacked_index:?}, {test_alg:?}"
            );
        }
    }
}
