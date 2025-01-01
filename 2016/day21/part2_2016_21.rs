// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 21 Part 2
use std::collections::{HashMap, VecDeque};

#[derive(Debug, PartialEq, Clone)]
enum ScrambleOp {
    SwapPos(usize, usize),
    SwapLetters(char, char),
    RotLeft(usize),
    RotRight(usize),
    RotFromLet(char),
    Reverse { start: usize, end: usize },
    Move { from: usize, to: usize },
}

#[derive(Debug)]
enum ScrambleOpParseError {
    IndexParse,
    LetterSize,
    BadRange,
    UnrecognizedOp,
}
impl From<std::num::ParseIntError> for ScrambleOpParseError {
    fn from(_e: std::num::ParseIntError) -> Self {
        ScrambleOpParseError::IndexParse
    }
}

fn letter_from(s: &str) -> Result<char, ScrambleOpParseError> {
    let s: Vec<_> = s.chars().collect();
    if s.len() == 1 {
        Ok(s[0])
    } else {
        Err(ScrambleOpParseError::LetterSize)
    }
}

impl std::str::FromStr for ScrambleOp {
    type Err = ScrambleOpParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<_> = s.split_whitespace().collect();
        match &words[..] {
            ["swap", "position", x, "with", "position", y] => {
                Ok(Self::SwapPos(x.parse()?, y.parse()?))
            }
            ["swap", "letter", x, "with", "letter", y] => {
                Ok(Self::SwapLetters(letter_from(x)?, letter_from(y)?))
            }
            ["rotate", "right", x, "steps"] => Ok(Self::RotRight(x.parse()?)),
            ["rotate", "left", x, "steps"] => Ok(Self::RotLeft(x.parse()?)),
            ["rotate", "right", "1", "step"] => Ok(Self::RotRight(1)),
            ["rotate", "left", "1", "step"] => Ok(Self::RotLeft(1)),
            ["rotate", "based", "on", _, _, _, x] => Ok(Self::RotFromLet(letter_from(x)?)),
            ["reverse", "positions", x, "through", y] => {
                let start: usize = x.parse()?;
                let end: usize = y.parse()?;
                if end < start {
                    Err(ScrambleOpParseError::BadRange)
                } else {
                    Ok(Self::Reverse { start, end })
                }
            }
            ["move", "position", from, "to", "position", to] => Ok(Self::Move {
                from: from.parse()?,
                to: to.parse()?,
            }),
            _ => Err(ScrambleOpParseError::UnrecognizedOp),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DescrambleError {
    MissingLetter,
    OutOfBounds,
    AmbiguousOriginal,
    ImpossibleLocation,
}

fn position_of<I: Iterator<Item = char>>(
    target: char,
    iterable: I,
) -> Result<usize, DescrambleError> {
    for (i, c) in iterable.enumerate() {
        if c == target {
            return Ok(i);
        }
    }
    Err(DescrambleError::MissingLetter)
}

type RotLookupTable = HashMap<usize, Result<usize, DescrambleError>>;

/// Generate a RotLookupTable to use to look up the shift length needed to get from the original to
/// the shifted position
fn letter_rot_lookup_table(sz: usize) -> RotLookupTable {
    use std::iter::FromIterator;
    use DescrambleError::{AmbiguousOriginal, ImpossibleLocation};
    let mut generator = VecDeque::from_iter(std::iter::repeat_n('>', sz));
    let mut table = RotLookupTable::new();
    generator[0] = 'a';
    for i in 0..sz {
        // part 1 ScrambleOp::RotFromLet(c) code adapted here
        let mut shift_by = i + 1;
        if shift_by > 4 {
            shift_by += 1;
        }
        shift_by %= generator.len();
        let mut v = generator.clone();
        v.rotate_right(shift_by);
        let pos = position_of('a', v.iter().cloned()).expect("'a' known to be present");
        // if this is the first time the location has been seen, insert the current starting index,
        // otherwise, replace the value with Err(AmbiguousOriginal)
        table.entry(pos).and_modify(|e| *e = Err(AmbiguousOriginal)).or_insert(Ok(shift_by));
        generator.rotate_right(1);
    }
    for i in 0..sz {
        // set any unset entries to ImpossibleLocations, as they have not come up for any of them
        table.entry(i).or_insert(Err(ImpossibleLocation));
    }
    table
}

fn descramble(mut rules: Vec<ScrambleOp>, s: &str) -> Result<String, DescrambleError> {
    // the mirrored version of part 1's scramble - most operations are switched - rotations are in
    // the opposite of the original direction, and parameter order is swapped whether or not it
    // matters.
    //
    // The exceptions are ScrambleOp::Reverse, which is unchanged, as reversing it again undoes the
    // previous step, and ScrambleOp::RotFromLet, for which a lookup table is used to determine the
    // shift amount that would have resulted in this string from the original, which necessitated
    // either a hard-coded lookup table, or code to generate one. I went with the latter, which is
    // in the letter_rot_lookup_table function, because I couldn't come up with a good name for it.

    use ScrambleOp as Op;
    let mut letters: VecDeque<char> = s.chars().collect();
    let rot_lookup_table = letter_rot_lookup_table(letters.len());
    macro_rules! pos {
        ($c: ident) => {
            position_of($c, letters.iter().cloned())?
        };
    }
    rules.reverse();
    for op in rules.into_iter() {
        match op {
            Op::SwapPos(x, y) => letters.swap(y, x),
            Op::SwapLetters(a, b) => {
                let x = pos!(a);
                let y = pos!(b);
                letters.swap(y, x);
            }
            Op::RotLeft(n) => letters.rotate_right(n),
            Op::RotRight(n) => letters.rotate_left(n),
            Op::RotFromLet(c) => {
                let i: usize = rot_lookup_table[&pos!(c)]?;
                letters.rotate_left(i);
            }
            Op::Reverse { start, end } => {
                let mut reverser: Vec<char> = letters.range(start..=end).cloned().collect();
                reverser.reverse();
                let mut reverser = reverser.into_iter();
                letters
                    .range_mut(start..=end)
                    .for_each(|c| *c = reverser.next().unwrap_or_else(|| unreachable!()));
            }
            Op::Move { from, to } => {
                let l = letters.remove(to).ok_or(DescrambleError::OutOfBounds)?;
                letters.insert(from, l);
            }
        }
    }
    Ok(letters.into_iter().collect())
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let scrambler: Vec<ScrambleOp> = input
        .lines()
        .map(|line| line.parse().expect("Failed to parse scramble op"))
        .collect();
    println!(
        "{}",
        descramble(scrambler, "fbgdceah").expect("Failed to descramble string")
    );
}
