// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 22 Part 2

// This would've been impossible for me if it weren't for a tutorial on the required modular
// arithmetic at https://codeforces.com/blog/entry/72593, specifically geard towards this problem

use std::num::ParseIntError;

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(transparent)]
struct ModularI128<const M: i128>(i128);

macro_rules! impl_op {
    ($op_trait: ident, $fn_name: ident, $op: tt) => {
        impl<const M: i128> std::ops::$op_trait for ModularI128<M> {
            type Output = Self;
            fn $fn_name(self, other: Self) -> Self {
                ModularI128((self.0 $op other.0).rem_euclid(M))
            }
        }
    }
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);

impl<const M: i128> ModularI128<M> {
    fn pow(mut self, mut n: i128) -> Self {
        let mut y = Self(1);
        while n > 0 {
            if n.rem_euclid(2) == 1 {
                y = y * self;
            }
            self = self * self;
            n = n.div_euclid(2);
        }
        y
    }
}

impl<const M: i128> std::ops::Div for ModularI128<M> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * other.pow(M - 2)
    }
}

#[derive(PartialEq, Clone, Copy)]
struct LinearCongruentEquation<const M: i128> {
    mul: i128,
    add: i128,
}

impl<const M: i128> LinearCongruentEquation<M> {
    fn as_inverted_fn(&self) -> impl FnOnce(ModularI128<M>) -> ModularI128<M> {
        move |i| (i - ModularI128(self.add)) / ModularI128(self.mul)
    }

    fn as_fn(&self) -> impl FnOnce(ModularI128<M>) -> ModularI128<M> {
        move |i| (i * ModularI128(self.mul)) + ModularI128(self.add)
    }

    fn repeat(mut self, mut count: i128) -> Self {
        let mut composed = Self::default();
        while count > 0 {
            if count % 2 == 1 {
                composed += self
            }
            count = (count / 2).rem_euclid(M);
            self += self;
        }

        composed
    }
}

impl<const M: i128> Default for LinearCongruentEquation<M> {
    fn default() -> Self {
        Self { mul: 1, add: 0 }
    }
}

impl<const M: i128> std::iter::FromIterator<Self> for LinearCongruentEquation<M> {
    fn from_iter<I: IntoIterator<Item = Self>>(iter: I) -> Self {
        iter.into_iter().fold(Self::default(), |acc, x| acc + x)
    }
}

impl<const M: i128> std::fmt::Debug for LinearCongruentEquation<M> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.add == 0 {
            write!(fmt, "LinearCongruentEquation<M>(|x| x * {})", self.mul)
        } else if self.add.is_negative() {
            write!(
                fmt,
                "LinearCongruentEquation<M>(|x| (x * {}) - {})",
                self.mul, -self.add
            )
        } else {
            write!(
                fmt,
                "LinearCongruentEquation<M>(|x| (x * {}) + {})",
                self.mul, self.add
            )
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;

    let shuffle_lcf: LinearCongruentEquation<119315717514047> =
        read_to_string(args().nth(1).as_deref().unwrap_or("input"))
            .expect("Failed to read file!")
            .lines()
            .map(|line| line.parse().unwrap())
            .collect();

    let repeat_lcf = shuffle_lcf.repeat(101741582076661);

    let answer = repeat_lcf.as_inverted_fn()(ModularI128(2020));
    assert_eq!(repeat_lcf.as_fn()(answer), ModularI128(2020));

    println!("{}", answer.0);
}

#[derive(Debug)]
enum StepParseError {
    IntParse(#[allow(dead_code)] ParseIntError),
    FormatError(#[allow(dead_code)] String),
}

impl From<ParseIntError> for StepParseError {
    fn from(e: ParseIntError) -> Self {
        Self::IntParse(e)
    }
}

impl<const M: i128> std::str::FromStr for LinearCongruentEquation<M> {
    type Err = StepParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();

        match &words[..] {
            ["cut", count] => Ok(Self {
                add: (-count.parse::<i128>()?) % M,
                mul: 1,
            }),
            ["deal", "into", "new", "stack"] => Ok(Self { add: -1, mul: -1 }),
            ["deal", "with", "increment", count] => Ok(Self {
                add: 0,
                mul: count.parse()?,
            }),
            _ => Err(StepParseError::FormatError(s.into())),
        }
    }
}

impl<const M: i128> std::ops::Add for LinearCongruentEquation<M> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let Self { mul: a, add: b } = self;
        let Self { mul: c, add: d } = other;
        // if f(x) = ax + b mod m, and g(x) = cx + d mod m, then g(f(x)) = c(ax + b) + d mod m
        // that is equivalent to acx + bc + d mod m
        let mul = (a * c) % M;
        let add = ((b * c) + d) % M;
        Self { mul, add }
    }
}

impl<const M: i128> std::ops::AddAssign for LinearCongruentEquation<M> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}
