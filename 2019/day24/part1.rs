// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 24 Part 1

/// u32 used as bit-fields that indicate whether the next cell is updated or not. The highest 7
/// bits are ignored.
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Field(u32);

#[inline]
const fn pack_index(row: u8, col: u8) -> u8 {
    row * 5 + col
}
#[inline]
const fn unpack_index(index: u8) -> (u8, u8) {
    (index / 5, index % 5)
}
impl Field {
    #[inline]
    const fn neighbors(bit_index: u8) -> [Option<u8>; 5] {
        match unpack_index(bit_index) {
            (0, 0) => [
                Some(pack_index(0, 1)),
                Some(pack_index(1, 0)),
                None,
                None,
                None,
            ],
            (4, 4) => [
                Some(pack_index(3, 4)),
                Some(pack_index(4, 3)),
                None,
                None,
                None,
            ],
            (0, 4) => [
                Some(pack_index(0, 3)),
                Some(pack_index(1, 4)),
                None,
                None,
                None,
            ],
            (4, 0) => [
                Some(pack_index(4, 1)),
                Some(pack_index(3, 0)),
                None,
                None,
                None,
            ],
            (0, c) => [
                Some(pack_index(1, c)),
                Some(pack_index(0, c - 1)),
                Some(pack_index(0, c + 1)),
                None,
                None,
            ],
            (4, c) => [
                Some(pack_index(3, c)),
                Some(pack_index(4, c - 1)),
                Some(pack_index(4, c + 1)),
                None,
                None,
            ],
            (r, 0) => [
                Some(pack_index(r, 1)),
                Some(pack_index(r - 1, 0)),
                Some(pack_index(r + 1, 0)),
                None,
                None,
            ],
            (r, 4) => [
                Some(pack_index(r, 3)),
                Some(pack_index(r - 1, 4)),
                Some(pack_index(r + 1, 4)),
                None,
                None,
            ],
            (r, c) => [
                Some(pack_index(r - 1, c)),
                Some(pack_index(r + 1, c)),
                Some(pack_index(r, c - 1)),
                Some(pack_index(r, c + 1)),
                None,
            ],
        }
    }

    const fn next_gen(self) -> Self {
        let mut next = 0;
        let mut bit_index = 0;
        while bit_index < 25 {
            let mut living_neighbors = 0;
            let neighbors = Self::neighbors(bit_index);
            let mut i = 0;
            while let Some(neighbor) = neighbors[i] {
                if self.0 & (1 << neighbor) != 0 {
                    living_neighbors += 1;
                }
                i += 1;
            }
            if self.0 & (1 << bit_index) != 0 {
                if living_neighbors == 1 {
                    next |= 1 << bit_index;
                }
            } else if living_neighbors == 1 || living_neighbors == 2 {
                next |= 1 << bit_index;
            }
            bit_index += 1;
        }

        Self(next)
    }

    const fn score(self) -> u32 {
        self.0
    }
}

fn main() {
    use std::collections::HashSet;
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut field_data = 0;
    for (i, has_bug) in input
        .bytes()
        .filter_map(|b| match b {
            b'#' => Some(true),
            b'.' => Some(false),
            _ => None,
        })
        .enumerate()
    {
        if has_bug {
            field_data |= 1 << i;
        }
    }
    let mut field = Field(field_data);
    let mut seen_fields = HashSet::new();
    while seen_fields.insert(field) {
        field = field.next_gen();
    }
    println!("{}", field.score());
}

impl std::fmt::Debug for Field {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "Field(0x{:x}){{", self.0)?;
        for row in 0..5 {
            write!(fmt, "\t")?;
            for col in 0..5 {
                if self.0 & (1 << pack_index(row, col)) == 0 {
                    write!(fmt, ".")?;
                } else {
                    write!(fmt, "#")?;
                }
            }
            writeln!(fmt)?;
        }
        writeln!(fmt, "}}")
    }
}
