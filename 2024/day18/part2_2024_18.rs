// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 18 Part 2

use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, PartialEq, Copy, Clone)]
enum MemSpace {
    Corrupt,
    Safe(Option<u16>),
}

type MemGrid = [[MemSpace; 71]; 71];

fn dijkstra_score(grid: &mut MemGrid) {
    macro_rules! neighbors {
        ($x: ident, $y: ident) => {{
            vec![
                $x.checked_sub(1).map(|nx| (nx, $y)),
                $x.checked_add(1).map(|nx| (nx, $y)),
                $y.checked_sub(1).map(|ny| ($x, ny)),
                $y.checked_add(1).map(|ny| ($x, ny)),
            ]
            .into_iter()
            .filter_map(|i| i)
        }};
    }
    let mut queue: Vec<(u16, (usize, usize))> = vec![(0, (0, 0))];
    'dijkstra_loop: while !queue.is_empty() {
        queue.sort_by(|a, b| b.cmp(a));
        let (dist, (x, y)) = queue.pop().expect("failed to pop from non-empty queue");
        if let MemSpace::Safe(Some(i)) = grid[y][x] {
            if i < dist {
                continue 'dijkstra_loop;
            }
        }

        'neighbors: for (nx, ny) in neighbors!(x, y) {
            let new_dist = dist + 1;
            match grid.get(ny).and_then(|row| row.get(nx)) {
                Some(MemSpace::Safe(Some(i))) if new_dist >= *i => continue 'neighbors,
                None | Some(MemSpace::Corrupt) => continue 'neighbors,
                _ => (),
            }
            grid[ny][nx] = MemSpace::Safe(Some(new_dist));
            queue.push((new_dist, (nx, ny)));
        }
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut grid: MemGrid = [[MemSpace::Safe(None); 71]; 71];
    'main_loop: for line in input.lines() {
        let (x, y) = line
            .trim()
            .split_once(",")
            .expect("Failed to split line on comma");
        let x = x
            .parse::<usize>()
            .expect("Failed to parse x position as number");
        let y = y
            .parse::<usize>()
            .expect("Failed to parse y position as number");
        grid[y][x] = MemSpace::Corrupt;
        let mut test_grid = grid;
        dijkstra_score(&mut test_grid);
        match test_grid[70][70] {
            MemSpace::Safe(Some(_)) => continue 'main_loop,
            _ => {
                println!("{x},{y}");
                break 'main_loop;
            }
        }
    }
}
