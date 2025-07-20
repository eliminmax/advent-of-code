// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 24 Part 2

use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
#[repr(i8)]
#[allow(dead_code, reason = "\"dead\" variants constructed via transmute")]
/// The coordinate system used for a specific layer.
///
/// In the following diagram, `><` means a portal outwards, and `<>` means a portal inwards
///
/// ```text
///    >< >< >< >< ><
/// >< AA AB AC AD AE ><
/// >< BA BB BC BD BE ><
/// >< CA CB <> CD CE ><
/// >< DA DB DC DD DE ><
/// >< EA EB EC ED EE ><
///    >< >< >< >< ><
/// ```
enum StandardCoord {
    AA = 0,
    AB = 1,
    AC = 2,
    AD = 3,
    AE = 4,

    BA = 5,
    BB = 6,
    BC = 7,
    BD = 8,
    BE = 9,

    CA = 10,
    CB = 11,
    // CC is the next grid layer entry
    CD = 13,
    CE = 14,

    DA = 15,
    DB = 16,
    DC = 17,
    DD = 18,
    DE = 19,

    EA = 20,
    EB = 21,
    EC = 22,
    ED = 23,
    EE = 24,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum DirectionalNeighborhood {
    Same(StandardCoord),
    Out(StandardCoord),
    In([StandardCoord; 5]),
}

impl StandardCoord {
    const NORTHERN_EDGE: [Self; 5] = [Self::AA, Self::AB, Self::AC, Self::AD, Self::AE];
    const WESTERN_EDGE: [Self; 5] = [Self::AA, Self::BA, Self::CA, Self::DA, Self::EA];
    const SOUTHERN_EDGE: [Self; 5] = [Self::EA, Self::EB, Self::EC, Self::ED, Self::EE];
    const EASTERN_EDGE: [Self; 5] = [Self::AE, Self::BE, Self::CE, Self::DE, Self::EE];

    const fn neighbors(self) -> [DirectionalNeighborhood; 4] {
        use std::hint::unreachable_unchecked;
        use std::mem::transmute;
        use DirectionalNeighborhood as DN;
        let n = match self as i8 {
            i8::MIN..=-1 | 12 | 25..=i8::MAX => unsafe { unreachable_unchecked() },
            0..=4 => DN::Out(Self::BC),
            17 /* Self::DC */ => DN::In(Self::SOUTHERN_EDGE),
            i => DN::Same(
                // SAFETY: all possible remaining variants are 5 more than other valid variants
                unsafe {transmute::<i8, Self>(i - 5)}
            ),
        };
        let s = match self as i8 {
            i8::MIN..=-1 | 12 | 25..=i8::MAX => unsafe { unreachable_unchecked() },
            20..=24 => DN::Out(Self::DC),
            7 /* Self::BC */ => DN::In(Self::NORTHERN_EDGE),
            i => DN::Same(
                // SAFETY: all possible remaining variants are 5 less than other valid variants
                unsafe {transmute::<i8, Self>(i + 5)}
            ),
        };
        let w = match self as i8 {
            i8::MIN..=-1 | 12 | 25..=i8::MAX => unsafe { unreachable_unchecked() },
            13 /* Self::CD */ => DN::In(Self::EASTERN_EDGE),
            0 | 5 | 10 | 15 | 20 => DN::Out(Self::CB),
            i => DN::Same (
                // SAFETY: all possible remaining variants are 1 less than other valid variants
                unsafe {transmute::<i8, Self>(i - 1)}
            ),
        };
        let e = match self as i8 {
            i8::MIN..=-1 | 12 | 25..=i8::MAX => unsafe { unreachable_unchecked() },
            11 /* Self::CB */ => DN::In(Self::WESTERN_EDGE),
            4 | 9 | 14 | 19 | 24 => DN::Out(Self::CD),
            i => DN::Same (
                // SAFETY: all possible remaining variants are 1 less than other valid variants
                unsafe {transmute::<i8, Self>(i + 1)}
            ),
        };

        [n, e, s, w]
    }

    const fn from_pair(row: i8, col: i8) -> Option<Self> {
        use std::mem::transmute;
        let joined = row * 5 + col;
        match joined {
            0..=11 | 13..=24 => Some(
                // SAFETY: match ensures that only valid discriminants will be transmuted
                unsafe { transmute::<i8, Self>(joined) },
            ),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Position {
    location: StandardCoord,
    depth: i16,
}

impl Position {
    fn neighbors(&self) -> impl Iterator<Item = Self> {
        let mut ret = Vec::new();
        for neighbor in self.location.neighbors() {
            match neighbor {
                DirectionalNeighborhood::Same(location) => ret.push(Self { location, ..*self }),
                DirectionalNeighborhood::Out(location) => ret.push(Self {
                    location,
                    depth: self.depth - 1,
                }),
                DirectionalNeighborhood::In(locations) => {
                    ret.extend(locations.into_iter().map(|location| Self {
                        location,
                        depth: self.depth + 1,
                    }))
                }
            }
        }
        ret.into_iter()
    }
}

fn main() {
    use std::env::args;
    use std::fs::read;
    let input = read(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut living_bugs = BTreeSet::new();
    for r in 0..5 {
        for c in 0..5 {
            // skip over the newlines by multiplying by 6
            if input[(r * 6 + c) as usize] == b'#' {
                living_bugs.insert(Position {
                    depth: 0,
                    location: StandardCoord::from_pair(r, c).unwrap(),
                });
            }
        }
    }

    for _ in 0..200 {
        let mut next_gen = BTreeSet::new();
        // ugly approach with repeated work, but with only 200 generations, it's fine
        for position in living_bugs
            .iter()
            .flat_map(Position::neighbors)
            .chain(living_bugs.iter().cloned())
        {
            let neighbor_count = position
                .neighbors()
                .filter(|n| living_bugs.contains(n))
                .count();
            if neighbor_count == 1 || (neighbor_count == 2 && !living_bugs.contains(&position)) {
                next_gen.insert(position);
            }
        }
        living_bugs = next_gen;
    }
    println!("{}", living_bugs.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn neighbors() {
        let pos = Position {
            location: StandardCoord::AA,
            depth: 0,
        };
        let neighbors: BTreeSet<_> = pos.neighbors().collect();
        macro_rules! pos {
            ($depth: literal, $loc:ident) => {
                Position {
                    location: StandardCoord::$loc,
                    depth: $depth,
                }
            };
        }
        let expected = BTreeSet::from([pos!(0, AB), pos!(0, BA), pos!(-1, BC), pos!(-1, CB)]);
        assert_eq!(neighbors, expected);
    }
}
