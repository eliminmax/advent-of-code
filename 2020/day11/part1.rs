// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 11 Part 1

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
enum Seat {
    #[default]
    Empty,
    Occupied,
}

const fn adjacent_locations((row, col): (usize, usize)) -> [(usize, usize); 8] {
    [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
}

fn get_updates(seat_map: &HashMap<(usize, usize), Seat>) -> Vec<((usize, usize), Seat)> {
    let (occupied, unoccupied): (Vec<_>, Vec<_>) = seat_map
        .keys()
        .copied()
        .partition(|loc| seat_map[loc] == Seat::Occupied);

    let mut next_gen: Vec<_> = occupied
        .into_iter()
        .filter_map(|loc| {
            if adjacent_locations(loc)
                .into_iter()
                .filter(|l| seat_map.get(l).copied() == Some(Seat::Occupied))
                .count()
                >= 4
            {
                Some((loc, Seat::Empty))
            } else {
                None
            }
        })
        .collect();

    next_gen.extend(unoccupied.into_iter().filter_map(|loc| {
        if adjacent_locations(loc)
            .into_iter()
            .filter(|l| seat_map.get(l).copied() == Some(Seat::Occupied))
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

    let mut seat_map: HashMap<(usize, usize), Seat> = input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| {
                match c {
                    // add one to simplify querying neighbors, as checking offsets won't underflow
                    // if starting from (1, 1) instead of (0, 0)
                    '#' => Some(((row + 1, col + 1), Seat::Empty)),
                    'L' => Some(((row + 1, col + 1), Seat::Occupied)),
                    _ => None,
                }
            })
        })
        .collect();
    while let next_gen = get_updates(&seat_map)
        && !next_gen.is_empty()
    {
        for (loc, seat) in next_gen {
            seat_map.entry(loc).and_modify(|e| *e = seat);
        }
    }
    println!(
        "{}",
        seat_map
            .into_values()
            .filter(|v| *v == Seat::Occupied)
            .count()
    );
}
