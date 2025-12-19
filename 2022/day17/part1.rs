// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 17 Part 1

use std::iter::Cycle;
use std::ops;
macro_rules! rock {
    {$r0: literal $r1: literal $r2: literal $r3: literal} => {{
        rock!(validate($r0));
        rock!(validate($r1));
        rock!(validate($r2));
        rock!(validate($r3));
        Rock(($r0 << 25) | ($r1 << 17) | ($r2 << 9) | ($r3 << 1))
    }};
    // used within the main macro version to validate that each literal is in the expected form
    (validate($l: literal)) => {
        const{
            let l: &[u8] = stringify!($l).as_bytes();
            assert!(l.len() == 6, concat!("bad sized rock row: ", $l));
            assert!(l[0] == b'0' && l[1] == b'b', concat!("bad_prefix: ", $l));
        }
    };
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Rock(u32);

impl Rock {
    const CYCLE: [Self; 5] = [
        rock! {
            0b0000
            0b0000
            0b0000
            0b1111
        },
        rock! {
            0b0000
            0b0100
            0b1110
            0b0100
        },
        rock! {
            0b0000
            0b0010
            0b0010
            0b1110
        },
        rock! {
            0b1000
            0b1000
            0b1000
            0b1000
        },
        rock! {
            0b0000
            0b0000
            0b1100
            0b1100
        },
    ];

    // Return an array of rows, ordered from lowest to highest
    const fn into_rows(self) -> [Row; 4] {
        let rows = self.0.to_le_bytes();
        [Row(rows[0]), Row(rows[1]), Row(rows[2]), Row(rows[3])]
    }
}

impl ops::Add<Direction> for Rock {
    fn add(self, rhs: Direction) -> Rock {
        match rhs {
            Direction::Left => {
                if self.0 & 0x40404040 != 0 {
                    return self;
                }
                let bytes = self.0.to_be_bytes();
                Self(u32::from_be_bytes(core::array::from_fn(|i| bytes[i] << 1)))
            }
            Direction::Right => {
                if self.0 & 0x01010101 != 0 {
                    return self;
                }
                let bytes = self.0.to_be_bytes();
                Self(u32::from_be_bytes(core::array::from_fn(|i| bytes[i] >> 1)))
            }
        }
    }

    type Output = Rock;
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Row(u8);

impl Row {
    fn intersects(self, Row(other): Self) -> bool {
        self.0 & other != 0
    }

    fn merge(self, other: Self) -> Self {
        assert_eq!(self.0 & other.0, 0, "{:b}, {:b}", self.0, other.0);
        Row(self.0 | other.0)
    }
}

#[derive(Debug, Default)]
struct Tower(Vec<Row>);

impl Tower {
    fn add_rock<I: Iterator<Item = Direction> + Clone>(
        &mut self,
        mut rock: Rock,
        winds: &mut Cycle<I>,
    ) {
        let mut height = self.0.len() + 3;

        loop {
            // Move from the wind
            let direction = winds.next().unwrap();
            let new_rock = rock + direction;
            let rows = new_rock.into_rows();
            let mut shiftable = true;
            'inner: for (y_off, row) in rows.into_iter().enumerate() {
                if let Some(r) = self.0.get(height + y_off).cloned()
                    && r.intersects(row)
                {
                    shiftable = false;
                    break 'inner;
                }
            }

            if shiftable {
                rock = new_rock;
            }

            // move down
            shiftable = true;
            if height == 0 {
                shiftable = false;
            } else {
                'inner: for (y_off, &row) in rock.into_rows().iter().enumerate() {
                    if self
                        .0
                        .get(y_off + height - 1)
                        .is_some_and(|r| r.intersects(row))
                    {
                        shiftable = false;
                        break 'inner;
                    }
                }
            }
            if shiftable {
                height -= 1;
            } else {
                if self.0.len() <= height + 3 {
                    self.0.extend([Row(0)].repeat(height + 4 - self.0.len()));
                }
                for (y_off, r) in rock.into_rows().into_iter().enumerate() {
                    self.0[y_off + height] = self.0[y_off + height].merge(r);
                }
                break;
            }
        }

        while self.0.last().copied() == Some(Row(0)) {
            self.0.pop();
        }
    }

    fn height(&self) -> usize {
        self.0.len()
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut wind_cycle = input
        .trim()
        .bytes()
        .map(|b| match b {
            b'>' => Direction::Right,
            b'<' => Direction::Left,
            b => panic!("invalid wind direction: {}", b.escape_ascii()),
        })
        .collect::<Vec<_>>()
        .into_iter()
        .cycle();
    let mut tower = Tower::default();
    for rock in Rock::CYCLE.into_iter().cycle().take(2022) {
        tower.add_rock(rock, &mut wind_cycle);
    }
    println!("{}", tower.height());
}
