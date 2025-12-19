// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 17 Part 2

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

impl<I> ops::Index<I> for Tower
where
    Vec<Row>: ops::Index<I>,
{
    type Output = <Vec<Row> as ops::Index<I>>::Output;

    fn index(&self, i: I) -> &Self::Output {
        self.0.index(i)
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let wind_cycle_inner = input
        .trim()
        .bytes()
        .map(|b| match b {
            b'>' => Direction::Right,
            b'<' => Direction::Left,
            b => panic!("invalid wind direction: {}", b.escape_ascii()),
        })
        .collect::<Vec<_>>();
    let mut tower = Tower::default();
    let cycle_unit_len = lcm(Rock::CYCLE.len(), wind_cycle_inner.len());
    let mut wind_cycle = wind_cycle_inner.iter().copied().cycle();

    // find a cycle
    let mut heights = vec![];
    let mut offsets = vec![];
    let mut prev_indices = vec![];
    let (start, end) = 'outer: loop {
        for rock in Rock::CYCLE.into_iter().cycle().take(cycle_unit_len) {
            tower.add_rock(rock, &mut wind_cycle);
        }
        let offset = tower.height() - heights.last().copied().unwrap_or_default();
        prev_indices.clear();
        prev_indices.extend(
            offsets
                .iter()
                .enumerate()
                .filter_map(|(i, &o)| if o == offset { Some(i) } else { None }),
        );
        for i in prev_indices.iter().copied() {
            let mp = i.midpoint(offsets.len());
            let mph = heights[mp];
            if prev_indices.contains(&mp) && tower[heights[i]..mph] == tower[mph..] {
                break 'outer (mp, offsets.len());
            }
        }

        heights.push(tower.height());
        offsets.push(offset);
    };

    let mut rock_count = 1_000_000_000_000 - (start * cycle_unit_len);
    let cycle_len = (end - start) * cycle_unit_len;
    let full_cycle_count = rock_count / cycle_len;
    rock_count %= cycle_len;

    let base_height =
        heights[start - 1] + full_cycle_count * offsets[start..end].iter().sum::<usize>();

    // with the cycle information available, reset to the point right before the cycle begins, to
    // figure out the height added after the last full cycle.
    let mut tower = Tower::default();
    let mut wind_cycle = wind_cycle_inner.into_iter().cycle();
    let mut rock_cycle = Rock::CYCLE.into_iter().cycle();
    for _ in 0..(start * cycle_unit_len) {
        tower.add_rock(rock_cycle.next().unwrap(), &mut wind_cycle);
    }

    let pre_cycle_height = tower.height();
    for rock in rock_cycle.take(rock_count) {
        tower.add_rock(rock, &mut wind_cycle);
    }
    println!("{}", base_height + tower.height() - pre_cycle_height);
}

// Function adapted from solution to 2022 day 11 part 2
/// Calculate the lowest common denominator of 2 `usize`s
const fn lcm(a: usize, b: usize) -> usize {
    let gcd = {
        let (mut a, mut b) = (a, b);
        while a != 0 {
            (a, b) = (b % a, a);
        }
        b
    };
    a * b / gcd
}
