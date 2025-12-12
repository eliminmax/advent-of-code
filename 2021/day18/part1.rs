// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 18 Part 1

// This is hell to model the data of, and I switched repeatedly between different approaches and
// languages while mulling over how to begin, given the way it goes between different layers. It
// may seem like Rust is a bad choice for this, I'd argue that it makes screwing up when going
// between different levels less likely to go unnoticed.

#[derive(PartialEq, Clone)]
enum SnailfishNum {
    Single(u8),
    Pair {
        left: Box<SnailfishNum>,
        right: Box<SnailfishNum>,
    },
}

/// enum representing the status of the explosion processing
#[derive(Debug, PartialEq, Clone, Copy)]
enum ExplosionOutcome {
    /// no explosion has occurred
    NoExplosion,
    /// an explosion has occurred and been resolved
    Resolved,
    /// the child node has exploded, and cleanup is needed.
    ChildExploding { left: u8, right: u8 },
    /// a descendent node has exploded, and the left side still needs to be resolved
    ExplodingLeft { left: u8 },
    /// a descendent node has exploded, and the right side still needs to be resolved
    ExplodingRight { right: u8 },
}

impl SnailfishNum {
    /// Add the right value that resulted from an explosion to the left
    fn add_from_left_explosion(&mut self, right: u8) {
        match self {
            SnailfishNum::Single(i) => *i += right,
            SnailfishNum::Pair { left, .. } => left.add_from_left_explosion(right),
        }
    }

    /// Add the left value that resulted from an explosion to the right
    fn add_from_right_explosion(&mut self, left: u8) {
        match self {
            SnailfishNum::Single(i) => *i += left,
            SnailfishNum::Pair { right, .. } => right.add_from_right_explosion(left),
        }
    }

    /// try to handle an explosion within a child node.
    /// Returns true if an explosion occurred, and false if no explodable pairs were found.
    fn try_explode(&mut self) -> bool {
        self.explode_inner(0) != ExplosionOutcome::NoExplosion
    }

    fn explode_inner(&mut self, nest_level: u8) -> ExplosionOutcome {
        match self {
            SnailfishNum::Single(_) => ExplosionOutcome::NoExplosion,
            SnailfishNum::Pair { left, right } if nest_level >= 4 => {
                let (&Self::Single(left), &Self::Single(right)) = (left.as_ref(), right.as_ref())
                else {
                    panic!("nested too deep")
                };
                *self = Self::Single(0);
                ExplosionOutcome::ChildExploding { left, right }
            }
            SnailfishNum::Pair { left, right } => {
                match left.explode_inner(nest_level + 1) {
                    ExplosionOutcome::ChildExploding {
                        left: lval,
                        right: rval,
                    } => {
                        right.add_from_left_explosion(rval);
                        return ExplosionOutcome::ExplodingLeft { left: lval };
                    }
                    ExplosionOutcome::ExplodingRight { right: rval } => {
                        right.add_from_left_explosion(rval);
                        return ExplosionOutcome::Resolved;
                    }
                    ExplosionOutcome::NoExplosion => (),
                    other => return other,
                }

                match right.explode_inner(nest_level + 1) {
                    ExplosionOutcome::ChildExploding {
                        left: lval,
                        right: rval,
                    } => {
                        left.add_from_right_explosion(lval);
                        ExplosionOutcome::ExplodingRight { right: rval }
                    }
                    ExplosionOutcome::ExplodingLeft { left: lval } => {
                        left.add_from_right_explosion(lval);
                        ExplosionOutcome::Resolved
                    }
                    other => other,
                }
            }
        }
    }

    fn try_split(&mut self) -> bool {
        match self {
            Self::Single(i) if *i >= 10 => {
                *self = Self::Pair {
                    left: Box::new(Self::Single(*i / 2)),
                    right: Box::new(Self::Single(i.div_ceil(2))),
                };
                true
            }
            Self::Single(_) => false,
            SnailfishNum::Pair { left, right } => left.try_split() || right.try_split(),
        }
    }

    fn reduce(&mut self) {
        // double-checked that rust supports short-circuiting, so if try_explode succeeds,
        // try_split won't run, ensuring the correct order is preserved
        while self.try_explode() || self.try_split() {}
    }

    fn magnitude(&self) -> u32 {
        match self {
            SnailfishNum::Single(n) => (*n).into(),
            SnailfishNum::Pair { left, right } => left.magnitude() * 3 + right.magnitude() * 2,
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let nums = input.lines().map(str::parse).map(Result::unwrap);
    let sum: SnailfishNum = add_snailfish_nums(nums).unwrap();
    println!("{}", sum.magnitude());
}

impl std::fmt::Display for SnailfishNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(n) => write!(f, "{n}"),
            SnailfishNum::Pair { left, right } => write!(f, "[{left},{right}]"),
        }
    }
}
impl std::fmt::Debug for SnailfishNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl std::ops::Add for SnailfishNum {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut added = Self::Pair {
            left: Box::new(self),
            right: Box::new(rhs),
        };
        added.reduce();
        added
    }
}

fn add_snailfish_nums(into_iter: impl IntoIterator<Item = SnailfishNum>) -> Option<SnailfishNum> {
    let mut iter = into_iter.into_iter();
    let start = iter.next()?;
    Some(iter.fold(start, |acc, x| acc + x))
}

// hide away the parsing logic here, to keep the focus on the actual logic of the solution
mod parse {

    use super::SnailfishNum;

    #[derive(Debug)]
    pub(super) enum SnailfishNumParseError {
        InvalidStructure,
        InvalidCharacter(#[allow(unused)] char),
        LeftoverChars(#[allow(unused)] String),
        EmptyInput,
    }

    impl SnailfishNum {
        fn parse_from_iter(
            chars: &mut impl Iterator<Item = char>,
        ) -> Result<Self, SnailfishNumParseError> {
            match chars.next().ok_or(SnailfishNumParseError::EmptyInput)? {
                '[' => {
                    let left = Box::new(Self::parse_from_iter(chars)?);

                    if chars.next() != Some(',') {
                        return Err(SnailfishNumParseError::InvalidStructure);
                    }

                    let right = Box::new(Self::parse_from_iter(chars)?);
                    if chars.next() != Some(']') {
                        return Err(SnailfishNumParseError::InvalidStructure);
                    }
                    Ok(SnailfishNum::Pair { left, right })
                }

                c => {
                    if let Some(n32) = c.to_digit(10)
                        && let Ok(n) = n32.try_into()
                    {
                        Ok(SnailfishNum::Single(n))
                    } else {
                        Err(SnailfishNumParseError::InvalidCharacter(c))
                    }
                }
            }
        }
    }

    impl std::str::FromStr for SnailfishNum {
        type Err = SnailfishNumParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut chars = s.chars();
            let parsed = Self::parse_from_iter(&mut chars)?;

            let leftover: String = chars.collect();
            if leftover.is_empty() {
                Ok(parsed)
            } else {
                Err(Self::Err::LeftoverChars(leftover))
            }
        }
    }
}
