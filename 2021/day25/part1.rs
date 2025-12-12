// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 25 Part 1

use std::collections::HashSet;

struct CucumberGrid {
    east_facing: HashSet<(u8, u8)>,
    south_facing: HashSet<(u8, u8)>,
    w: u8,
    h: u8,
}

impl CucumberGrid {
    fn contains(&self, pos: (u8, u8)) -> bool {
        self.east_facing.contains(&pos) || self.south_facing.contains(&pos)
    }
    fn east_mov(&mut self) -> bool {
        let mut ret = false;
        let mut next_east = HashSet::with_capacity(self.east_facing.len());
        for (x, y) in self.east_facing.iter().cloned() {
            let next_pos = ((x + 1) % self.w, y);
            if self.contains(next_pos) {
                next_east.insert((x, y));
            } else {
                ret = true;
                next_east.insert(next_pos);
            }
        }
        self.east_facing = next_east;
        ret
    }

    fn south_mov(&mut self) -> bool {
        let mut ret = false;
        let mut next_south = HashSet::with_capacity(self.south_facing.len());
        for (x, y) in self.south_facing.iter().cloned() {
            let next_pos = (x, (y + 1) % self.h);
            if self.contains(next_pos) {
                next_south.insert((x, y));
            } else {
                ret = true;
                next_south.insert(next_pos);
            }
        }
        self.south_facing = next_south;
        ret
    }

    fn moves_until_end(mut self) -> usize {
        for i in 1.. {
            if !(self.east_mov() | self.south_mov()) {
                return i;
            }
        }
        panic!("More than usize::MAX rounds");
    }

    /// a lazy, not-at-all robust alternative to a proper str::FromStr impl that panics at the
    /// first sign of a problem parsing
    fn panicky_parse(s: &str) -> Self {
        let rows: Vec<&str> = s.lines().collect();
        assert!(
            rows[1..].iter().all(|r| r.len() == rows[0].len()),
            "mismatched widths"
        );
        let w = u8::try_from(rows[0].len()).unwrap();
        let h = u8::try_from(rows.len()).unwrap();
        let mut south_facing = HashSet::new();
        let mut east_facing = HashSet::new();
        for (y, r) in rows.iter().enumerate() {
            let y = y as u8;
            for (x, c) in r.chars().enumerate() {
                let x = x as u8;
                match c {
                    'v' => {
                        let _ = south_facing.insert((x, y));
                    }
                    '>' => {
                        let _ = east_facing.insert((x, y));
                    }
                    '.' => (),
                    _ => panic!("invalid character: {c:?}"),
                }
            }
        }

        Self {
            south_facing,
            east_facing,
            w,
            h,
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let grid = CucumberGrid::panicky_parse(&input);
    println!("{}", grid.moves_until_end());
}
