// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 15 Part 2

mod dijkstra;

use dijkstra::dijkstra;


static GRID: [[u8; 500]; 500] = const {
    let template: [[u8; 100]; 100] = {
        #[cfg(aoc_direct)]
        let input = include_bytes!("input");
        #[cfg(not(aoc_direct))]
        let input = include_bytes!("../input");
        let mut grid = [[0xff; 100]; 100];
        let mut row = 0;
        while row < 100 {
            let row_offset = row * 101;
            let mut col = 0;
            while col < 100 {
                grid[row][col] = input[row_offset + col] - b'0';
                col += 1;
            }
            row += 1;
        }

        grid
    };

    let mut grid = [[0xff; 500]; 500];
    let mut row = 0;
    // populate the first 100 rows fully
    while row < 100 {
        let mut col = 0;
        // for the first 100 columns, just copy from the template.
        while col < 100 {
            grid[row][col] = template[row][col];
            col += 1;
        }
        // for the next 400 columns, define each one in relation to its predecessor
        while col < 500 {
            let num = grid[row][col - 100];
            grid[row][col] = if num >= 9 { 1 } else { num + 1 };
            col += 1;
        }

        row += 1;
    }
    // define the remaining rows in relation to their predecessors
    while row < 500 {
        let mut col = 0;
        while col < 500 {
            let num = grid[row - 100][col];
            grid[row][col] = if num >= 9 { 1 } else { num + 1 };
            col += 1;
        }
        row += 1;
    }

    grid
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position {
    row: u16,
    col: u16,
}

impl Position {
    const fn risk_level(self) -> u32 {
        GRID[self.row as usize][self.col as usize] as u32
    }
    const fn scored(self) -> (Self, u32) {
        (self, self.risk_level())
    }

    fn neighbors(self) -> impl IntoIterator<Item = (Self, u32)> {
        macro_rules! neighbor_check {
            {$cond: expr => $op: tt $axis: ident} => {{
                $cond.then(|| Self { $axis: self.$axis $op 1, ..self })
            }}
        }
        [
            neighbor_check! { (self.row > 0) => - row},
            neighbor_check! { (self.row < 499) => + row},
            neighbor_check! { (self.col > 0) => - col},
            neighbor_check! { (self.col < 499) => + col},
        ]
        .into_iter()
        .flatten()
        .map(Self::scored)
    }
}

fn main() {
    let answer =
        dijkstra(Position::default(), Position::neighbors)[&Position { row: 499, col: 499 }];
    println!("{answer}");
}
