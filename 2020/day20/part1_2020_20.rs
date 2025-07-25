// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 20 Part 1
use std::collections::{HashMap, VecDeque};

#[derive(PartialEq, Clone, Copy)]
struct Tile {
    id: u16,
    pixels: [[bool; 10]; 10],
}

impl Tile {
    fn lazy_panicky_parse(s: &str) -> Self {
        let mut lines = s.lines();
        let id: u16 = lines
            .next()
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .strip_prefix("Tile ")
            .unwrap()
            .parse()
            .unwrap();
        let pixels: [[bool; 10]; 10] = core::array::from_fn(|_| {
            let row = lines.next().unwrap().as_bytes();
            core::array::from_fn(|i| row[i] == b'#')
        });
        Self { id, pixels }
    }

    #[inline(always)]
    const fn top_edge(&self) -> [bool; 10] {
        self.pixels[0]
    }

    #[inline(always)]
    const fn bottom_edge(&self) -> [bool; 10] {
        self.pixels[9]
    }

    fn left_edge(&self) -> [bool; 10] {
        core::array::from_fn(|i| self.pixels[i][0])
    }

    fn right_edge(&self) -> [bool; 10] {
        core::array::from_fn(|i| self.pixels[i][9])
    }

    fn rotate(&mut self) {
        for y in 0..10 {
            for x in y..10 {
                let prev_yx = self.pixels[y][x];
                self.pixels[y][x] = self.pixels[x][y];
                self.pixels[x][y] = prev_yx;
            }
        }
        self.flip();
    }

    fn flip(&mut self) {
        for y in 0..10 {
            self.pixels[y].reverse();
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let mut queue: VecDeque<Tile> = input
        .trim() // trim to avoid trying to parse the empty space at the end as a tile
        .split("\n\n")
        .map(Tile::lazy_panicky_parse)
        .collect();

    let mut grid: HashMap<(i8, i8), Tile> = HashMap::with_capacity(queue.len());
    grid.insert((0, 0), queue.pop_front().unwrap());

    'outer: while let Some(mut tile) = queue.pop_front() {
        let mut pos: Option<(i8, i8)> = None;
        macro_rules! locator_loop {
            {} => {
                'locator: for _ in 0..4 {
                    tile.rotate();
                    for ((x, y), neighbor) in grid.iter() {
                        macro_rules! check_match {
                            ($self_edge: ident, $other_edge: ident, $pos_expr: expr) => {
                                if tile.$self_edge() == neighbor.$other_edge() {
                                    pos = Some($pos_expr);
                                    break 'locator;
                                }
                            };
                        }
                        check_match!(top_edge, bottom_edge, (*x, y + 1));
                        check_match!(bottom_edge, top_edge, (*x, y - 1));
                        check_match!(right_edge, left_edge, (x - 1, *y));
                        check_match!(left_edge, right_edge, (x + 1, *y));
                    }
                }
            };
        }
        locator_loop! {}
        if pos.is_none() {
            tile.flip();
            locator_loop! {}
        }

        let Some((x, y)) = pos else {
            queue.push_back(tile);
            continue 'outer;
        };

        assert!(!grid.contains_key(&(x, y)), "duplicate positions");

        macro_rules! check_pos {
            (neighbor:($x: expr, $y: expr), tile.$tile_edge: ident(), neighbor.$other_edge: ident()) => {
                assert!(grid
                    .get(&($x, $y))
                    .is_none_or(|t| t.$other_edge() == tile.$tile_edge()));
            };
        }

        check_pos!(neighbor: (x - 1, y), tile.left_edge(), neighbor.right_edge());
        check_pos!(neighbor: (x + 1, y), tile.right_edge(), neighbor.left_edge());
        check_pos!(neighbor: (x, y - 1), tile.top_edge(), neighbor.bottom_edge());
        check_pos!(neighbor: (x, y + 1), tile.bottom_edge(), neighbor.top_edge());
        grid.insert((x, y), tile);
    }
    let mut min_x = i8::MAX;
    let mut min_y = i8::MAX;
    let mut max_x = i8::MIN;
    let mut max_y = i8::MIN;

    for &(x, y) in grid.keys() {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            assert!(grid.contains_key(&(x, y)), "discontinuity at ({x}, {y})");
        }
    }

    let top_left = u64::from(grid[&(min_x, min_y)].id);
    let bottom_left = u64::from(grid[&(min_x, max_y)].id);
    let top_right = u64::from(grid[&(max_x, min_y)].id);
    let bottom_right = u64::from(grid[&(max_x, max_y)].id);
    println!("{}", top_left * bottom_left * top_right * bottom_right);

}

impl std::fmt::Display for Tile {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(fmt, "Tile {}:", self.id)?;
        for row in self.pixels {
            for pixel in row {
                write!(fmt, "{}", if pixel { '#' } else { '.' })?;
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}
