// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 22 Part 1
use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GridSpace {
    Open,
    Wall,
}

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
enum Facing {
    #[default]
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Facing {
    const fn turn_right(&mut self) {
        *self = match *self {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        }
    }
    const fn turn_left(&mut self) {
        *self = match *self {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let (map, directions) = input.split_once("\n\n").unwrap();
    let mut grid = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut start = None;
    for (y, line) in map.lines().enumerate() {
        let y = i32::try_from(y).unwrap() + 1;
        max_y = max_y.max(y);
        for (x, col) in line.trim_end_matches('\n').chars().enumerate() {
            let x = i32::try_from(x).unwrap() + 1;
            max_x = max_x.max(x);
            match col {
                ' ' => (),
                '#' => {
                    grid.insert((x, y), GridSpace::Wall);
                }
                '.' => {
                    grid.insert((x, y), GridSpace::Open);
                    if start.is_none() {
                        start = Some((x, y));
                    }
                }
                c => panic!("invalid space: {c:?}"),
            }
        }
    }

    let next_position = |facing: Facing, (mut x, mut y), count| {
        let grid = &grid;
        'l: for _ in 0..count {
            let mut next_x = x;
            let mut next_y = y;
            macro_rules! move_direction {
                {$dir_assign: tt $axis_var: ident $cmp: tt $wrap_from: expr => $wrap_to: expr} => {{
                    $axis_var $dir_assign 1;
                    while !grid.contains_key(&(next_x, next_y)) {
                        if $axis_var $cmp $wrap_from {
                            $axis_var = $wrap_to;
                        } else {
                            $axis_var $dir_assign 1;
                        }
                    }
                }}
            }
            match facing {
                Facing::Right => move_direction! { += next_x > max_x => 1 },
                Facing::Down => move_direction! { += next_y > max_y => 1 },
                Facing::Left => move_direction! { -= next_x <= 1 => max_y },
                Facing::Up => move_direction! { -= next_y <= 1 => max_y },
            };
            if grid[&(next_x, next_y)] == GridSpace::Open {
                (x, y) = (next_x, next_y);
            } else {
                break 'l;
            }
        }
        (x, y)
    };

    let mut pos = start.unwrap();
    let mut facing = Facing::default();
    for step in directions.trim().split_inclusive(['L', 'R']) {
        let count: i32 = step.trim_end_matches(['L', 'R']).parse().unwrap();
        pos = next_position(facing, pos, count);
        match step.chars().skip_while(|c| c.is_ascii_digit()).last() {
            Some('L') => facing.turn_left(),
            Some('R') => facing.turn_right(),
            Some(c) => panic!("invalid direction: {c:?}"),
            None => (),
        }
    }
    let (x, y) = pos;
    let password = (y * 1000) + (x * 4) + facing as i32;
    println!("{password}");
}
