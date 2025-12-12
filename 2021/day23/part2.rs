// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 23 Part 2

mod dijkstra;
use dijkstra::targeted_dijkstra;

use core::array::from_fn as array_from_fn;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
#[repr(align(4))]
struct SideHall([Option<Amphipod>; 4]);

impl SideHall {
    fn only_has(&self, amphi: Amphipod) -> bool {
        self.0.iter().all(|o| o.is_none_or(|a| a == amphi))
    }

    fn first_occupied(&self) -> Option<(usize, Amphipod)> {
        self.0
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, c)| c.map(|amphi| (i, amphi)))
            .next()
    }

    fn last_empty(&self) -> Option<usize> {
        for (i, c) in self.0.iter().enumerate().rev() {
            if c.is_none() {
                debug_assert!(self.0[..i].iter().all(|e| e.is_none()), "{self:?}");
                debug_assert!(self.0[(i + 1)..].iter().all(|e| e.is_some()), "{self:?}");
                return Some(i);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Burrow {
    main_hall: [Option<Amphipod>; 7],
    side_halls: [SideHall; 4],
}

impl Burrow {
    fn with_swap(&self, main_hall_idx: usize, amphi_hall_idx: usize, depth: usize) -> Self {
        let Self {
            mut side_halls,
            mut main_hall,
        } = *self;
        debug_assert!(
            main_hall[main_hall_idx].is_some() ^ side_halls[amphi_hall_idx][depth].is_some(),
            "{:?}, {:?}, {self:?}::with_swap({main_hall_idx}, {amphi_hall_idx:?}, {depth})",
            main_hall[main_hall_idx],
            side_halls[amphi_hall_idx][depth]
        );
        std::mem::swap(
            &mut main_hall[main_hall_idx],
            &mut side_halls[amphi_hall_idx][depth],
        );
        Self {
            side_halls,
            main_hall,
        }
    }

    fn reachable_states(self) -> Vec<(Self, u64)> {
        const fn small_index(full_index: usize) -> Option<usize> {
            match full_index {
                0 | 1 => Some(full_index),
                2 | 4 | 6 | 8 => None,
                3 | 5 | 7 | 9 => Some(full_index.div_ceil(2)),
                10 => Some(6),
                _ => panic!("out of bounds"),
            }
        }

        let mut v = vec![];

        let occupied: [bool; 11] = array_from_fn(|i| {
            if let Some(si) = small_index(i) {
                self.main_hall[si].is_some()
            } else {
                false
            }
        });

        for (amphi_index, amphi) in self.main_hall.iter().copied().enumerate() {
            let Some(amphi) = amphi else { continue };
            let subhall_index = (amphi as usize + 1) * 2;

            let full_index = match amphi_index {
                0 | 1 => amphi_index,
                6 => 10,
                2..=5 => amphi_index * 2 - 1,
                _ => panic!("out of bounds"),
            };

            let mut range = if full_index < subhall_index {
                (full_index + 1)..subhall_index
            } else {
                subhall_index..full_index
            };

            if range.any(|i| occupied[i]) {
                continue;
            }

            if self.side_halls[amphi as usize].only_has(amphi)
                && let Some(depth) = self.side_halls[amphi as usize].last_empty()
            {
                let distance = (full_index.abs_diff(subhall_index) + depth + 1) as u64;
                let cost = distance * amphi.move_cost();
                v.push((self.with_swap(amphi_index, amphi as usize, depth), cost));
            }
        }

        for (index, side_hall) in self.side_halls.iter().enumerate() {
            let Some((depth, amphi)) = side_hall.first_occupied() else {
                continue;
            };

            if amphi as usize == index && side_hall.only_has(amphi) {
                continue;
            }

            let x_pos = (index + 1) * 2;

            'left_moves: for full_dest in (0..x_pos).rev() {
                if occupied[full_dest] {
                    break 'left_moves;
                }
                // FIXME: on second iteration with sample input, tries to swap a None with a None.
                // State looks completely valid when that happens. Figure out why.
                if let Some(small_dest) = small_index(full_dest) {
                    v.push((
                        self.with_swap(small_dest, index, depth),
                        ((x_pos - full_dest + depth + 1) as u64) * amphi.move_cost(),
                    ));
                }
            }

            #[allow(
                clippy::needless_range_loop,
                reason = "better fits with intent and more concise"
            )]
            'right_moves: for full_dest in (x_pos + 1)..11 {
                if occupied[full_dest] {
                    break 'right_moves;
                }
                if let Some(small_dest) = small_index(full_dest) {
                    v.push((
                        self.with_swap(small_dest, index, depth),
                        ((full_dest - x_pos + depth + 1) as u64) * amphi.move_cost(),
                    ))
                }
            }
        }

        v
    }

    const TARGET_STATE: Self = {
        Self {
            main_hall: [None; 7],
            side_halls: [
                SideHall([Some(Amphipod::Amber); 4]),
                SideHall([Some(Amphipod::Bronze); 4]),
                SideHall([Some(Amphipod::Copper); 4]),
                SideHall([Some(Amphipod::Desert); 4]),
            ],
        }
    };

    fn panicky_parse(s: &str) -> Self {
        let (heads, tails) = s
            .strip_suffix('\n') // remove trailing newline
            .unwrap_or(s) // continue with original string if no trailing newline is present
            .strip_prefix("#############\n#...........#\n###") // remove beginning
            .and_then(|s| s.strip_suffix("#\n  #########")) // remove ending
            .and_then(|s| s.split_once("###\n  #")) // split lines
            .unwrap();
        debug_assert_eq!(heads.len(), 7, "{heads:?}");
        debug_assert_eq!(tails.len(), 7, "{tails:?}");

        let mut heads = heads
            .chars()
            .filter(|c| *c != '#')
            .map(|c| Amphipod::try_from(c).unwrap());

        let mut tails = tails
            .chars()
            .filter(|c| *c != '#')
            .map(|c| Amphipod::try_from(c).unwrap());

        let side_halls = array_from_fn(|i| {
            use Amphipod::{Amber as A, Bronze as B, Copper as C, Desert as D};

            SideHall([
                Some(heads.next().unwrap()),
                Some([D, C, B, A][i]),
                Some([D, B, A, C][i]),
                Some(tails.next().unwrap()),
            ])
        });

        assert!(heads.next().is_none(), "extra characters in head row");
        assert!(tails.next().is_none(), "extra characters in tail row");
        Self {
            main_hall: [None; 7],
            side_halls,
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let max_cost = targeted_dijkstra(
        Burrow::panicky_parse(&input),
        Burrow::TARGET_STATE,
        Burrow::reachable_states,
    );
    println!("{max_cost}");
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Amphipod {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

impl Amphipod {
    const fn move_cost(self) -> u64 {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

impl TryFrom<char> for Amphipod {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'A' => Ok(Self::Amber),
            'B' => Ok(Self::Bronze),
            'C' => Ok(Self::Copper),
            'D' => Ok(Self::Desert),
            _ => Err(c),
        }
    }
}

impl std::ops::Index<usize> for SideHall {
    type Output = Option<Amphipod>;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl std::ops::IndexMut<usize> for SideHall {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}
