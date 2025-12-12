// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 12 Part 1

#[derive(Debug, Default, Clone)]
struct Moon {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
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
        let x: i64 = parse_param!("x");
        let y: i64 = parse_param!("y");
        let z: i64 = parse_param!("z");

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
    fn energy(&self) -> u64 {
        (self.x.unsigned_abs() + self.y.unsigned_abs() + self.z.unsigned_abs())
            * (self.dx.unsigned_abs() + self.dy.unsigned_abs() + self.dz.unsigned_abs())
    }
}

fn time_step(moons: &mut [Moon; 4]) {
    macro_rules! update_velocity {
        ($a: literal, $b: literal) => {
            moons[$a].dx += (moons[$b].x.cmp(&moons[$a].x) as i64);
            moons[$a].dy += (moons[$b].y.cmp(&moons[$a].y) as i64);
            moons[$a].dz += (moons[$b].z.cmp(&moons[$a].z) as i64);
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

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let moons: Vec<Moon> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut moons: [Moon; 4] = moons.try_into().unwrap();

    for _ in 0..1000 {
        time_step(&mut moons);
    }

    println!(
        "{}",
        moons.into_iter().map(|moon| moon.energy()).sum::<u64>()
    );
}
