// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 20 Part 1

use std::num::ParseIntError;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Quant3D {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
struct Particle {
    p: Quant3D,
    v: Quant3D,
    a: Quant3D,
}

impl Particle {
    fn distance_from_origin(&self) -> u32 {
        self.p.x.unsigned_abs() + self.p.y.unsigned_abs() + self.p.z.unsigned_abs()
    }

    fn update(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }
}

fn order_of(particles: &[Particle]) -> Vec<usize> {
    let mut order: Vec<(usize, &Particle)> = particles.iter().enumerate().collect();
    order.sort_by_key(|(_i, p)| p.distance_from_origin());
    order.into_iter().map(|(i, _p)| i).collect()
}

fn main() {
    let mut particles: Vec<Particle> = include_str!("input")
        .lines()
        .map(|line| line.parse().expect("Failed to parse line as particle"))
        .collect();
    let mut order = order_of(&particles[..]);
    loop {
        let prev_order = order.clone();
        particles.iter_mut().for_each(|p| p.update());
        order = order_of(&particles[..]);
        if order == prev_order {
            println!("{}", order[0]);
            break;
        }
    }
}

impl std::ops::AddAssign for Quant3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

macro_rules! unparsable {
    ($s: ident) => {{
        ParticleParseError::UnparsableFmt {
            _unparsed: Box::from($s),
        }
    }};
}

impl std::str::FromStr for Particle {
    type Err = ParticleParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::convert::TryInto;
        let components: [&str; 3] = s
            .split(", ")
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| unparsable!(s))?;
        macro_rules! component {
            ($label: ident, $i: literal) => {{
                match components[$i].split_once('=') {
                    Some((stringify!($label), v)) => v.parse(),
                    _ => Err(unparsable!(s)),
                }
            }};
        }

        Ok(Particle {
            p: component!(p, 0)?,
            v: component!(v, 1)?,
            a: component!(a, 2)?,
        })
    }
}

impl std::str::FromStr for Quant3D {
    type Err = ParticleParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s
            .strip_prefix('<')
            .ok_or(unparsable!(s))?
            .strip_suffix('>')
            .ok_or(unparsable!(s))?
            .split(',')
            .map(|c| c.parse::<i32>())
            .collect::<Result<Vec<_>, ParseIntError>>()?
            .into_iter();

        let ret = Quant3D {
            x: components.next().ok_or(unparsable!(s))?,
            y: components.next().ok_or(unparsable!(s))?,
            z: components.next().ok_or(unparsable!(s))?,
        };
        if components.next().is_none() {
            Ok(ret)
        } else {
            Err(unparsable!(s))
        }
    }
}

#[derive(Debug)]
enum ParticleParseError {
    IntParse { _err: ParseIntError },
    UnparsableFmt { _unparsed: Box<str> },
}

impl From<ParseIntError> for ParticleParseError {
    fn from(e: ParseIntError) -> Self {
        ParticleParseError::IntParse { _err: e }
    }
}
