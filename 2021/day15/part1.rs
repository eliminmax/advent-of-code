// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 15 Part 1

mod dijkstra;

use dijkstra::dijkstra;

static GRID: [[u16; 100]; 100] = const {
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
            grid[row][col] = (input[row_offset + col] - b'0') as u16;
            col += 1;
        }
        row += 1;
    }

    grid
};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Position {
    row: u8,
    col: u8,
}

impl Position {
    const fn scored(self) -> (Self, u16) {
        (self, GRID[self.row as usize][self.col as usize])
    }

    fn neighbors(self) -> impl IntoIterator<Item = (Self, u16)> {
        macro_rules! neighbor_check {
            {$cond: expr => $op: tt $axis: ident} => {{
                $cond.then(|| Self { $axis: self.$axis $op 1, ..self })
            }}
        }
        [
            neighbor_check! { (self.row > 0) => - row},
            neighbor_check! { (self.row < 99) => + row},
            neighbor_check! { (self.col > 0) => - col},
            neighbor_check! { (self.col < 99) => + col},
        ]
        .into_iter()
        .flatten()
        .map(Self::scored)
    }
}

fn main() {
    let answer = dijkstra(Position::default(), Position::neighbors)[&Position { row: 99, col: 99 }];
    println!("{answer}");
}
