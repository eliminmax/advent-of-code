// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 13 Part 1

type Location = (usize, usize);
const FAVORITE_NUMBER: usize = include!("input");
const TARGET_POSITION: Location = (31, 39);

fn is_open((x, y): Location) -> bool {
    ((x * x) + (3 * x) + (2 * x * y) + y + (y * y) + FAVORITE_NUMBER).count_ones() % 2 == 0
}

fn find_fewest_steps() -> u16 {
    // don't search if at least this many spaces past the target X or Y coordinate
    const PADDING: usize = 8;
    use std::collections::HashMap;

    macro_rules! neighbors {
        ($loc: ident) => {{
            vec![
                $loc.0.checked_sub(1).map(|nx| (nx, $loc.1)),
                $loc.0
                    .checked_add(1)
                    .map(|nx| {
                        if nx < TARGET_POSITION.0 + PADDING {
                            Some((nx, $loc.1))
                        } else {
                            None
                        }
                    })
                    .flatten(),
                $loc.1.checked_sub(1).map(|ny| ($loc.0, ny)),
                $loc.1
                    .checked_add(1)
                    .map(|ny| {
                        if ny < TARGET_POSITION.1 + PADDING {
                            Some(($loc.0, ny))
                        } else {
                            None
                        }
                    })
                    .flatten(),
            ]
            .into_iter()
            .filter_map(|i| {
                if i.is_some_and(|inner| is_open(inner)) {
                    i
                } else {
                    None
                }
            })
        }};
    }

    let mut distances: HashMap<Location, u16> = HashMap::new();
    let _ = distances.insert((1, 1), 0);
    let mut queue: Vec<(u16, Location)> = vec![(0, (1, 1))];
    'dijkstra_loop: while let Some((dist, loc)) = queue.pop() {
        if let Some(prev_dist) = distances.get(&loc) {
            if *prev_dist < dist {
                continue 'dijkstra_loop;
            }
        }
        for neighbor in neighbors!(loc) {
            let new_dist = dist + 1;
            if distances.get(&neighbor).is_none_or(|od| *od > new_dist) {
                let _old_val = distances.insert(neighbor, new_dist);
                queue.push((new_dist, neighbor));
            }
        }
        queue.sort_by(|a, b| b.cmp(a));
        queue.dedup();
    }

    *distances
        .get(&TARGET_POSITION)
        .unwrap_or_else(|| panic!("{:?}", distances))
}

fn main() {
    println!("{}", find_fewest_steps());
}
