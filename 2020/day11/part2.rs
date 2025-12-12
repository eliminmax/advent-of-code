// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 11 Part 2

use std::collections::HashMap;

type SeatMap = HashMap<(usize, usize), (Seat, Vec<(usize, usize)>)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
enum Seat {
    #[default]
    Empty,
    Occupied,
}

fn set_next_locations(seat_map: &mut SeatMap) {
    let mut bounds = [usize::MAX, usize::MAX, 0, 0];
    for (r, c) in seat_map.keys().copied() {
        bounds[0] = bounds[0].min(r);
        bounds[1] = bounds[1].min(c);
        bounds[2] = bounds[2].max(r);
        bounds[3] = bounds[3].max(c);
    }
    let [min_r, min_c, max_r, max_c] = bounds;

    let keys: Vec<_> = seat_map.keys().cloned().collect();
    for (row, col) in keys.iter().cloned() {
        macro_rules! directional_iter {
            (- r) => {{ (min_r..row).rev() }};
            (- c) => {{ (min_c..col).rev() }};
            (+ r) => {{ (row + 1)..=max_r }};
            (+ c) => {{ (col + 1)..=max_c }};
            (@ r) => {{ std::iter::repeat(row) }};
            (@ c) => {{ std::iter::repeat(col) }};
            ($r: tt r, $c: tt c) => {{ directional_iter!($r r).zip(directional_iter!($c c)) }};
        }

        macro_rules! find_loc {
            ($directional_iter: expr) => {
                for loc in $directional_iter {
                    if seat_map.contains_key(&loc) {
                        seat_map.entry((row, col)).and_modify(|e| e.1.push(loc));
                        break;
                    }
                }
            };
        }

        find_loc!(directional_iter!(-r, -c));
        find_loc!(directional_iter!(-r, @c));
        find_loc!(directional_iter!(-r, +c));
        find_loc!(directional_iter!(@r, -c));
        find_loc!(directional_iter!(@r, +c));
        find_loc!(directional_iter!(+r, -c));
        find_loc!(directional_iter!(+r, @c));
        find_loc!(directional_iter!(+r, +c));
    }
}

fn get_updates(seat_map: &SeatMap) -> Vec<((usize, usize), Seat)> {
    let (occupied, unoccupied): (Vec<_>, Vec<_>) = seat_map
        .keys()
        .copied()
        .partition(|loc| seat_map[loc].0 == Seat::Occupied);

    let mut next_gen: Vec<_> = occupied
        .into_iter()
        .filter_map(|loc| {
            if seat_map[&loc]
                .1
                .iter()
                .filter(|l| matches!(seat_map.get(l), Some((Seat::Occupied, _))))
                .count()
                >= 5
            {
                Some((loc, Seat::Empty))
            } else {
                None
            }
        })
        .collect();

    next_gen.extend(unoccupied.into_iter().filter_map(|loc| {
        if seat_map[&loc]
            .1
            .iter()
            .filter(|l| matches!(seat_map.get(l), Some((Seat::Occupied, _))))
            .count()
            == 0
        {
            Some((loc, Seat::Occupied))
        } else {
            None
        }
    }));
    next_gen
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let mut seat_map: SeatMap = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| {
                match c {
                    // add one to simplify querying neighbors, as checking offsets won't underflow
                    // if starting from (1, 1) instead of (0, 0)
                    '#' => Some(((row + 1, col + 1), (Seat::Empty, Vec::with_capacity(8)))),
                    'L' => Some(((row + 1, col + 1), (Seat::Occupied, Vec::with_capacity(8)))),
                    _ => None,
                }
            })
        })
        .collect();

    set_next_locations(&mut seat_map);

    while let next_gen = get_updates(&seat_map)
        && !next_gen.is_empty()
    {
        for (loc, seat) in next_gen {
            seat_map.entry(loc).and_modify(|e| e.0 = seat);
        }
    }
    println!(
        "{}",
        seat_map
            .into_values()
            .filter(|v| v.0 == Seat::Occupied)
            .count()
    );
}
