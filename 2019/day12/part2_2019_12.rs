// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 12 Part 2

use std::collections::HashMap;
use std::num::NonZeroU64;
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Moon {
    x: i16,
    y: i16,
    z: i16,
    dx: i16,
    dy: i16,
    dz: i16,
}

impl std::str::FromStr for Moon {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let mut parts = s
            .trim()
            .strip_suffix('>')
            .and_then(|p| p.strip_prefix('<'))
            .ok_or(s)?
            .split(", ");
        macro_rules! parse_param {
            ($param: literal) => {{
                parts
                    .next()
                    .and_then(|p| p.strip_prefix(concat!($param, "=")))
                    .ok_or(s)?
                    .parse()
                    .map_err(|e| format!(concat!("could not parse ", $param, ": {:?}"), e))?
            }};
        }
        let x: i16 = parse_param!("x");
        let y: i16 = parse_param!("y");
        let z: i16 = parse_param!("z");

        if let Some(leftover) = parts.next() {
            Err(format!(
                "Had leftover string data ({leftover:?}) at the end of {s:?}"
            ))
        } else {
            Ok(Self {
                x,
                y,
                z,
                ..Default::default()
            })
        }
    }
}

impl Moon {
    fn x_axis(&self) -> (i16, i16) {
        (self.x, self.dx)
    }
    fn y_axis(&self) -> (i16, i16) {
        (self.y, self.dy)
    }
    fn z_axis(&self) -> (i16, i16) {
        (self.z, self.dz)
    }
}

fn time_step(moons: &mut [Moon; 4]) {
    macro_rules! update_velocity {
        ($a: literal, $b: literal) => {
            moons[$a].dx += (moons[$b].x.cmp(&moons[$a].x) as i16);
            moons[$a].dy += (moons[$b].y.cmp(&moons[$a].y) as i16);
            moons[$a].dz += (moons[$b].z.cmp(&moons[$a].z) as i16);
        };
        ($a: literal, $b: literal, $($others: literal),+) => {
            update_velocity!($a, $b);
            update_velocity!($a, $($others),+);
        };
    }
    update_velocity!(0, 1, 2, 3);
    update_velocity!(1, 2, 3, 0);
    update_velocity!(2, 3, 0, 1);
    update_velocity!(3, 0, 1, 2);

    for moon in moons.iter_mut() {
        moon.x += moon.dx;
        moon.y += moon.dy;
        moon.z += moon.dz;
    }
}

type AxisLog = HashMap<[(i16, i16); 4], u64>;

fn main() {
    use core::array::from_fn as array_from;
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let moons: Vec<Moon> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut moons: [Moon; 4] = moons.try_into().unwrap();

    // because each axis is unaffected by other axes, find the span at which
    let mut x_log = AxisLog::new();
    let mut y_log = AxisLog::new();
    let mut z_log = AxisLog::new();
    let mut x_span: Option<(u64, NonZeroU64)> = None;
    let mut y_span: Option<(u64, NonZeroU64)> = None;
    let mut z_span: Option<(u64, NonZeroU64)> = None;
    for i in 0_u64.. {
        macro_rules! axis_check {
            ($axis_span: ident, $axis_log: ident, $axis_fn: ident) => {
                if $axis_span.is_none() {
                    if let Some(loop_start) =
                        $axis_log.insert(array_from(|i| moons[i].$axis_fn()), i)
                    {
                        $axis_span = Some((loop_start, NonZeroU64::new(i).unwrap()));
                    }
                }
            };
        }
        axis_check!(x_span, x_log, x_axis);
        axis_check!(y_span, y_log, y_axis);
        axis_check!(z_span, z_log, z_axis);

        if x_span.is_some() && y_span.is_some() && z_span.is_some() {
            break;
        }
        time_step(&mut moons);
    }

    let (Some((0, x_end)), Some((0, y_end)), Some((0, z_end))) = (x_span, y_span, z_span) else {
        unimplemented!("Solution assumes all 3 cycles start at t=0");
    };

    let x_end = x_end.get();
    let y_end = y_end.get();
    let z_end = z_end.get();
    println!("{}", lcm3(x_end, y_end, z_end));
    
}

/// Calculate the lowest common denominator of 2 `u64`s
fn lcm(a: u64, b: u64) -> u64 {
    let gcd = {
        let (mut a, mut b) = (a, b);
        // greatest common denominator algorithm adapted for `u64` from 2019 day 10
        while a != 0 {
            (a, b) = (b % a, a);
        }
        b
    };
    a * b / gcd
}

/// find least common multiple of 3 numbers
fn lcm3(a: u64, b: u64, c: u64) -> u64 {
    lcm(lcm(a, b), c)
}
