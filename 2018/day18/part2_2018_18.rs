// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 18 Part 2

// I knew it. I originally impl'd Eq and Hash for GridContents and LumberGrid in part 1 because I
// saw this coming, but changed my mind, and decided to wait for part 2.

const GRID_SIZE: usize = 50;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum GridContents {
    OpenGround,
    Trees,
    LumberYard,
}

impl GridContents {
    fn next_gen(&self, neighbors: Neighborhood) -> Self {
        let Neighborhood { trees, yards } = neighbors;
        macro_rules! processing_rule {
            ($condition: expr, $next_gen: ident) => {{
                if $condition {
                    GridContents::$next_gen
                } else {
                    *self
                }
            }};
        }
        match self {
            GridContents::OpenGround => processing_rule!(trees >= 3, Trees),
            GridContents::Trees => processing_rule!(yards >= 3, LumberYard),
            GridContents::LumberYard => processing_rule!(yards == 0 || trees == 0, OpenGround),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct LumberGrid([[GridContents; GRID_SIZE]; GRID_SIZE]);

#[derive(Debug, PartialEq)]
struct Neighborhood {
    trees: u8,
    yards: u8,
}

impl LumberGrid {
    fn moore_neighbors(&self, row: usize, col: usize) -> Neighborhood {
        let mut yards = 0u8;
        let mut trees = 0u8;
        (row.saturating_sub(1)..=(row + 1).min(GRID_SIZE - 1)).for_each(|r| {
            (col.saturating_sub(1)..=(col + 1).min(GRID_SIZE - 1))
                .filter(|&c| (r, c) != (row, col))
                .for_each(|c| {
                    match self.0[r][c] {
                        GridContents::OpenGround => (), // not needed for any rules
                        GridContents::Trees => trees += 1,
                        GridContents::LumberYard => yards += 1,
                    }
                })
        });
        Neighborhood { trees, yards }
    }

    fn step_generations(&mut self, generations: u32) {
        use std::collections::HashMap;
        let mut seen_states = HashMap::from([(self.0, 0u32)]);
        let mut generation = 0;
        let (cycle_start, cycle_size) = loop {
            generation += 1;
            let next_gen: [[GridContents; GRID_SIZE]; GRID_SIZE] = core::array::from_fn(|r| {
                core::array::from_fn(|c| self.0[r][c].next_gen(self.moore_neighbors(r, c)))
            });
            self.0 = next_gen;
            if generation == generations {
                return;
            }
            if let Some(cycle_start) = seen_states.insert(next_gen, generation) {
                break (cycle_start, generation - cycle_start);
            }
        };
        let remaining = (generations - cycle_start) % cycle_size;
        for _ in 0..remaining {
            let next_gen: [[GridContents; GRID_SIZE]; GRID_SIZE] = core::array::from_fn(|r| {
                core::array::from_fn(|c| self.0[r][c].next_gen(self.moore_neighbors(r, c)))
            });
            self.0 = next_gen;
        }
    }

    fn resource_value(&self) -> u32 {
        let mut trees: u32 = 0;
        let mut yards: u32 = 0;
        self.0.iter().flatten().for_each(|g| match g {
            GridContents::OpenGround => (), // not needed for any rules
            GridContents::Trees => trees += 1,
            GridContents::LumberYard => yards += 1,
        });
        trees * yards
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut grid: LumberGrid = input.parse().expect("Failed to parse input");
    grid.step_generations(1000000000);
    println!("{}", grid.resource_value());
}

#[derive(Debug)]
enum InvalidGrid {
    BadRowWidth(#[allow(unused)] usize),
    BadGridHeight(#[allow(unused)] usize),
    UnknownGridSpaceContent(#[allow(unused)] char),
}

impl std::fmt::Debug for GridContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GridContents::OpenGround => '.',
                GridContents::Trees => '|',
                GridContents::LumberYard => '#',
            }
        )
    }
}

impl std::fmt::Debug for LumberGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row_strings: Vec<String> = self
            .0
            .iter()
            .map(|row| {
                row.iter()
                    .map(|g| match g {
                        GridContents::OpenGround => '.',
                        GridContents::Trees => '|',
                        GridContents::LumberYard => '#',
                    })
                    .collect()
            })
            .collect();
        write!(f, "{}", row_strings.join("\n"))
    }
}

impl std::str::FromStr for LumberGrid {
    type Err = InvalidGrid;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::convert::TryFrom;
        type GridRow = [GridContents; GRID_SIZE];
        type Grid = [GridRow; GRID_SIZE];
        Ok(LumberGrid(
            Grid::try_from(
                s.lines()
                    .map(|l| {
                        GridRow::try_from(
                            l.chars()
                                .map(|c| match c {
                                    '.' => Ok(GridContents::OpenGround),
                                    '|' => Ok(GridContents::Trees),
                                    '#' => Ok(GridContents::LumberYard),
                                    e => Err(InvalidGrid::UnknownGridSpaceContent(e)),
                                })
                                .collect::<Result<Vec<_>, _>>()?,
                        )
                        .map_err(|e| InvalidGrid::BadRowWidth(e.len()))
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            )
            .map_err(|e| InvalidGrid::BadGridHeight(e.len()))?,
        ))
    }
}
