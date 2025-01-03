// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 14 Part 2
type Grid = [bool; 128 * 128];

#[derive(Debug, Copy, Clone)]
struct Location(usize, usize);

impl Location {
    fn unpack(index: usize) -> Self {
        Location(index / 128, index % 128)
    }

    fn pack(&self) -> usize {
        (self.0 * 128) + self.1
    }

    fn neighbors(&self) -> impl Iterator<Item = Self> {
        vec![
            self.0.checked_sub(1).map(|nr| (nr, self.1)),
            Some((self.0 + 1, self.1)),
            self.1.checked_sub(1).map(|nc| (self.0, nc)),
            Some((self.0, self.1 + 1)),
        ]
        .into_iter()
        .filter_map(|i| match i {
            Some((r, c)) if r.max(c) < 128 => Some(Location(r, c)),
            _ => None
        })
    }
}

impl std::ops::Index<Location> for Grid {
    type Output = bool;
    fn index(&self, loc: Location) -> &bool {
        &self[loc.pack()]
    }
}

impl std::ops::IndexMut<Location> for Grid {
    fn index_mut(&mut self, loc: Location) -> &mut bool {
        &mut self[loc.pack()]
    }
}

fn count_regions(mut grid: Grid) -> u16 {
    use std::collections::VecDeque;
    let mut total = 0;
    while let Some(i) = grid.iter().position(|&b| b) {
        let loc = Location::unpack(i);
        let mut queue: VecDeque<Location> = VecDeque::from([(loc)]);
        grid[loc] = false;
        total += 1;
        while let Some(loc) = queue.pop_front() {
            for neighbor in loc.neighbors() {
                if grid[neighbor] {
                    grid[neighbor] = false;
                    queue.push_back(neighbor);
                }
            }
        }
    }
    assert!(!grid.iter().any(|&b| b));
    total
}

fn main() {
    const INPUT: &str = include_str!("input").trim_ascii_end();
    let mut grid = [false; 128 * 128];
    for i in 0..128 {
        let packed_row = u128::from_be_bytes(knot_hash(format!("{INPUT}-{i}").as_bytes()));
        for bit_index in 0..128 {
            grid[Location(i, bit_index)] = packed_row & (1 << (127 - bit_index)) != 0;
        }
    }
    println!("{}", count_regions(grid));
}

fn knot_hash(lengths: &[u8]) -> [u8; 16] {
    use std::collections::VecDeque;
    use std::iter::FromIterator;
    let mut knot_string = VecDeque::from_iter(0u8..=255);
    let mut current_pos: usize = 0;
    let mut skip_size: usize = 0;
    const EXTENSION: [u8; 5] = [17, 31, 73, 47, 23];

    // create the "sparse hash"
    for _ in 0..64 {
        for length in lengths.iter().cloned().chain(EXTENSION) {
            let length: usize = length.into();
            // collecting then calling into_iter to avoid keeping a reference to knot_string
            let mut reversed = knot_string
                .range(..length)
                .cloned()
                .rev()
                .collect::<Vec<_>>()
                .into_iter();
            knot_string
                .range_mut(..length)
                .for_each(|v| *v = reversed.next().unwrap_or_else(|| unreachable!()));
            current_pos += skip_size + length;
            knot_string.rotate_left((skip_size + length) % knot_string.len());
            skip_size += 1;
        }
    }
    knot_string.rotate_right(current_pos % knot_string.len());
    // create and return the "dense hash"
    let knot_string: Vec<_> = knot_string.into();
    let mut chunks = knot_string.chunks_exact(16);
    core::array::from_fn(|_| {
        chunks
            .next()
            .unwrap_or_else(|| unreachable!())
            .iter()
            .fold(0, |acc, x| acc ^ x)
    })
}
