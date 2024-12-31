// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 13 Part 2

type Location = (usize, usize);
const FAVORITE_NUMBER: usize = include!("input");

fn is_open((x, y): Location) -> bool {
    ((x * x) + (3 * x) + (2 * x * y) + y + (y * y) + FAVORITE_NUMBER).count_ones() % 2 == 0
}

fn find_reachable() -> usize {
    use std::collections::HashMap;

    macro_rules! neighbors {
        ($loc: ident) => {{
            vec![
                $loc.0.checked_sub(1).map(|nx| (nx, $loc.1)),
                $loc.0.checked_add(1).map(|nx| (nx, $loc.1)),
                $loc.1.checked_sub(1).map(|ny| ($loc.0, ny)),
                $loc.1.checked_add(1).map(|ny| ($loc.0, ny)),
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
        let new_dist = dist + 1;
        if new_dist > 50 {
            continue 'dijkstra_loop;
        }
        for neighbor in neighbors!(loc) {
            if distances.get(&neighbor).is_none_or(|od| *od > new_dist) {
                let _old_val = distances.insert(neighbor, new_dist);
                queue.push((new_dist, neighbor));
            }
        }
        queue.sort_by(|a, b| b.cmp(a));
        queue.dedup();
    }

    distances.len()
}

fn main() {
    println!("{}", find_reachable());
}
