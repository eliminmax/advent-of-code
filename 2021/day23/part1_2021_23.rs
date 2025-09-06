// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 23 Part 1

mod dijkstra;
use dijkstra::targeted_dijkstra;

use core::array::from_fn as array_from_fn;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
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

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
#[repr(align(8))]
struct Burrow {
    sub_halls: [[Option<Amphipod>; 2]; 4],
    main_hall: [Option<Amphipod>; 7],
}

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
enum Move {
    LeaveHead { src: u8, dst: u8 },
    LeaveTail { src: u8, dst: u8 },
    EnterHead { src: u8, dst: u8 },
    EnterTail { src: u8, dst: u8 },
}

impl Burrow {
    fn with_move(&self, m: Move) -> Self {
        use std::mem::swap as mem_swap;
        let Self {
            mut sub_halls,
            mut main_hall,
        } = *self;
        match m {
            Move::LeaveHead { src: a, dst: b } | Move::EnterHead { src: b, dst: a } => {
                mem_swap(
                    &mut sub_halls[usize::from(a)][0],
                    &mut main_hall[usize::from(b)],
                );
            }
            Move::LeaveTail { src: a, dst: b } | Move::EnterTail { src: b, dst: a } => {
                mem_swap(
                    &mut sub_halls[usize::from(a)][1],
                    &mut main_hall[usize::from(b)],
                );
            }
        }
        Self {
            sub_halls,
            main_hall,
        }
    }

    fn valid_moves(self) -> Vec<(Self, u64)> {
        const fn small_index(full_index: usize) -> Option<u8> {
            match full_index {
                0 | 1 => Some(full_index as u8),
                2 | 4 | 6 | 8 => None,
                3 => Some(2),
                5 => Some(3),
                7 => Some(4),
                9 => Some(5),
                10 => Some(6),
                _ => panic!("out of bounds"),
            }
        }

        let mut v = vec![];

        let occupied: [bool; 11] = array_from_fn(|i| {
            if let Some(si) = small_index(i) {
                self.main_hall[si as usize].is_some()
            } else {
                false
            }
        });

        /// a macro for converting values known to be in the range of valid x positions
        /// (i.e. `0..11`).
        macro_rules! index_convert {
            ($e: expr) => {{
                debug_assert!(matches!($e, 0..11));
                ($e).try_into().expect("index always in range 0..11")
            }};
            ($e: expr, $t: ty) => {{
                debug_assert!(matches!($e, 0..11));
                <$t>::try_from($e).expect("index always in range 0..11")
            }};
        }

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

            match self.sub_halls[amphi as usize] {
                [None, Some(other)] if other == amphi => {
                    let distance = index_convert!(full_index.abs_diff(subhall_index), u64) + 1;
                    let cost = distance * amphi.move_cost();
                    v.push((
                        self.with_move(Move::EnterHead {
                            src: index_convert!(amphi_index),
                            dst: amphi as u8,
                        }),
                        cost,
                    ))
                }

                [None, None] => {
                    let distance = index_convert!(full_index.abs_diff(subhall_index), u64) + 2;
                    let cost = distance * amphi.move_cost();
                    v.push((
                        self.with_move(Move::EnterTail {
                            src: index_convert!(amphi_index),
                            dst: (amphi as u8),
                        }),
                        cost,
                    ))
                }

                _ => (),
            }
        }

        for (i, sub_hall) in self.sub_halls.iter().enumerate() {
            let x_pos = (i + 1) * 2;
            match sub_hall {
                [Some(_), None] => unreachable!(),
                [None, None] => continue,
                [head, Some(tail)]
                    if *tail as usize == i && head.is_none_or(|h| h as usize == i) =>
                {
                    continue
                }
                [Some(head_amphi), Some(..)] => {
                    'left_moves: for full_dest in (0..x_pos).rev() {
                        if occupied[full_dest] {
                            break 'left_moves;
                        }
                        if let Some(dst) = small_index(full_dest) {
                            v.push((
                                self.with_move(Move::LeaveHead { src: i as u8, dst }),
                                index_convert!(x_pos - full_dest + 1, u64) * head_amphi.move_cost(),
                            ))
                        }
                    }
                    #[allow(clippy::needless_range_loop, reason = "better fits with intent and more concise")]
                    'right_moves: for full_dest in (x_pos + 1)..11 {
                        if occupied[full_dest] {
                            break 'right_moves;
                        }
                        if let Some(dst) = small_index(full_dest) {
                            v.push((
                                self.with_move(Move::LeaveHead { src: i as u8, dst }),
                                index_convert!(full_dest - x_pos + 1, u64) * head_amphi.move_cost(),
                            ))
                        }
                    }
                }
                [None, Some(tail_amphi)] => {
                    'left_moves: for full_dest in (0..x_pos).rev() {
                        if occupied[full_dest] {
                            break 'left_moves;
                        }
                        if let Some(dst) = small_index(full_dest) {
                            v.push((
                                self.with_move(Move::LeaveTail { src: i as u8, dst }),
                                index_convert!(x_pos - full_dest + 2, u64) * tail_amphi.move_cost(),
                            ))
                        }
                    }
                    #[allow(clippy::needless_range_loop, reason = "better fits with intent and more concise")]
                    'right_moves: for full_dest in (x_pos + 1)..11 {
                        if occupied[full_dest] {
                            break 'right_moves;
                        }
                        if let Some(dst) = small_index(full_dest) {
                            v.push((
                                self.with_move(Move::LeaveTail { src: i as u8, dst }),
                                index_convert!(full_dest - x_pos + 2, u64) * tail_amphi.move_cost(),
                            ))
                        }
                    }
                }
            }
        }

        v
    }

    const TARGET_STATE: Self = {
        Self {
            main_hall: [None; 7],
            sub_halls: [
                [Some(Amphipod::Amber); 2],
                [Some(Amphipod::Bronze); 2],
                [Some(Amphipod::Copper); 2],
                [Some(Amphipod::Desert); 2],
            ],
        }
    };

    fn panicky_parse(s: &str) -> Self {
        let (heads, tails) = s
            .strip_suffix('\n') // remove trailing newline
            .unwrap_or(s) // (continue with original string if no trailing newline is present)
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
        let sub_halls =
            array_from_fn(|_| [Some(heads.next().unwrap()), Some(tails.next().unwrap())]);
        assert!(heads.next().is_none(), "extra characters in head row");
        assert!(tails.next().is_none(), "extra characters in tail row");
        Self {
            main_hall: [None; 7],
            sub_halls,
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
        Burrow::valid_moves,
    );
    println!("{max_cost}");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn neighbors() {
        use std::collections::HashMap;
        // default, except the first amber amphipod is in the leftmost hallway position
        let bur = Burrow::TARGET_STATE.with_move(Move::LeaveHead { src: 0, dst: 0 });

        let neighbors: HashMap<_, _> = bur.valid_moves().into_iter().collect();

        assert_eq!(neighbors[&Burrow::TARGET_STATE], 3);
    }
}
