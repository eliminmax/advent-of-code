// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 11 Part 2

fn neighbors(row: usize, col: usize) -> impl Iterator<Item = (usize, usize)> {
    macro_rules! axis_iter {
        ($var: ident) => {{
            $var.saturating_sub(1)..=(9.min($var + 1))
        }};
    }

    (axis_iter!(row).flat_map(move |r| axis_iter!(col).map(move |c| (r, c))))
        .filter(move |loc| *loc != (row, col))
}

// run through a step, returning true if all octopi flashed
fn step(octopi: &mut [[u8; 10]; 10]) -> bool {
    use std::collections::BinaryHeap;
    octopi
        .iter_mut()
        .for_each(|row| row.iter_mut().for_each(|o| *o += 1));

    let mut queue: BinaryHeap<(usize, usize)> = (0..10)
        .flat_map(move |r| (0..10).map(move |c| (r, c)))
        .filter(|&(r, c)| (octopi[r][c] > 9))
        .collect();

    let mut have_flashed = [[false; 10]; 10];
    while let Some((row, col)) = queue.pop() {
        if have_flashed[row][col] {
            continue;
        }
        have_flashed[row][col] = true;
        for (r, c) in neighbors(row, col) {
            octopi[r][c] += 1;
            if octopi[r][c] > 9 && !have_flashed[r][c] {
                queue.push((r, c));
            }
        }
    }

    for r in 0..10 {
        for c in 0..10 {
            if have_flashed[r][c] {
                octopi[r][c] = 0;
            } else {
                debug_assert!(octopi[r][c] <= 9, "octopi[{r}][{c}]={}", octopi[r][c]);
            }
        }
    }

    have_flashed.into_iter().flatten().all(|b| b)
}

fn main() {
    let mut octopi: [[u8; 10]; 10] = const {
        #[cfg(aoc_direct)]
        let nums = include_bytes!("input");
        #[cfg(not(aoc_direct))]
        let nums = include_bytes!("../input");

        let mut octopi = [[0xff; 10]; 10];
        let mut row = 0;
        while row < 10 {
            let row_offset = row * 11;
            let mut col = 0;
            while col < 10 {
                let num = nums[row_offset + col];
                octopi[row][col] = num - b'0';
                col += 1;
            }
            row += 1;
        }

        octopi
    };

    let mut counter: u32 = 1;
    while !step(&mut octopi) {
        counter += 1;
    }
    println!("{counter}");
}
