// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2017 Day 21 Part 2
// I expected to need to do some kind of memoization or something for part 2, but I first simply
// compiled with `rustc -C opt-level=3 -C target-cpu=native`, and got a binary that gave me the
// right answer in 45 ms.
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
struct Image {
    pixels: Vec<bool>,
    width: usize,
}

type Region<const N: usize> = [[bool; N]; N];

#[derive(Debug, Clone)]
struct ReplacementRules {
    rules_2x2: HashMap<Region<2>, Region<3>>,
    rules_3x3: HashMap<Region<3>, Region<4>>,
}

impl Image {
    fn new() -> Self {
        Image {
            pixels: vec![
                false, true, false, // row 0, ".#."
                false, false, true, // row 1, "..#"
                true, true, true, //-- row 2, "###"
            ],
            width: 3,
        }
    }

    fn transform(&mut self, rules: &ReplacementRules) {
        // the code to split and regroup by rows is seriously gnarly.
        assert_eq!(self.width * self.width, self.pixels.len());
        let mut new_pixels: Vec<bool> = Vec::new();
        let rows: Vec<&[bool]> = self.pixels.chunks_exact(self.width).collect();
        macro_rules! chunks_array {
            ($size: literal) => {
                rows.chunks_exact($size)
                    .map(|c| c.try_into().unwrap_or_else(|_| unreachable!()))
                    .collect::<Vec<[&[bool]; $size]>>()
            };
        }
        macro_rules! extend_new_pixels {
            ($nr: ident, $i: ident) => {{
                $nr.iter()
                    .for_each(|r| new_pixels.extend_from_slice(&r[$i]));
            }};
        }
        if self.width % 2 == 0 {
            chunks_array!(2).into_iter().for_each(|rows: [&[bool]; 2]| {
                let mut new_regions: Vec<Region<3>> = Vec::new();
                rows[0]
                    .chunks_exact(2)
                    .zip(rows[1].chunks_exact(2))
                    .for_each(|(a, b)| {
                        new_regions.push(rules.rules_2x2[&[[a[0], a[1]], [b[0], b[1]]]]);
                    });
                (0..3).for_each(|i| extend_new_pixels!(new_regions, i));
            });
            self.width = self.width * 3 / 2;
        } else {
            assert_eq!(self.width % 3, 0);
            chunks_array!(3).into_iter().for_each(|rows: [&[bool]; 3]| {
                let mut new_regions: Vec<Region<4>> = Vec::new();
                rows[0]
                    .chunks_exact(3)
                    .zip(rows[1].chunks_exact(3))
                    .zip(rows[2].chunks_exact(3))
                    .for_each(|((a, b), c)| {
                        let key: Region<3> =
                            [[a[0], a[1], a[2]], [b[0], b[1], b[2]], [c[0], c[1], c[2]]];
                        new_regions.push(rules.rules_3x3[&key]);
                    });
                (0..4).for_each(|i| extend_new_pixels!(new_regions, i));
            });
            self.width = self.width * 4 / 3;
        }
        self.pixels = new_pixels;
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let rules: ReplacementRules = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!")
        .parse()
        .expect("Failed to parse rules from input");
    let mut image = Image::new();
    (0..18).for_each(|_| image.transform(&rules));
    println!("{}", image.pixels.iter().filter(|&b| *b).count());
}

#[derive(Debug)]
struct RuleParseError;

fn try_parse_rule<const N: usize, const M: usize>(
    key: &str,
    val: &str,
) -> Result<(Region<N>, Region<M>), RuleParseError> {
    const {
        assert!(N + 1 == M);
    }
    let key: Region<N> = key
        .split('/')
        .map(|s| s.chars().map(|c| c != '.').collect::<Vec<_>>().try_into())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| RuleParseError)?
        .try_into()
        .map_err(|_| RuleParseError)?;
    let val: Region<M> = val
        .split('/')
        .map(|s| s.chars().map(|c| c != '.').collect::<Vec<_>>().try_into())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| RuleParseError)?
        .try_into()
        .map_err(|_| RuleParseError)?;
    Ok((key, val))
}

fn rotate_key<const N: usize>(region: &mut Region<N>) {
    let rotated: Region<N> =
        core::array::from_fn(|i| core::array::from_fn(|j| region[N - j - 1][i]));
    *region = rotated;
}

fn flip_key<const N: usize>(region: Region<N>) -> Region<N> {
    let flipped: Region<N> =
        core::array::from_fn(|i| core::array::from_fn(|j| region[i][N - j - 1]));
    flipped
}

impl std::str::FromStr for ReplacementRules {
    type Err = RuleParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules_2x2: HashMap<Region<2>, Region<3>> = HashMap::new();
        let mut rules_3x3: HashMap<Region<3>, Region<4>> = HashMap::new();
        for line in s.lines() {
            let (key, val) = line.split_once(" => ").ok_or(RuleParseError)?;
            macro_rules! parse_into {
                ($target: ident) => {{
                    let (mut key, val) = try_parse_rule(key, val)?;
                    for _ in 0..4 {
                        let _ = $target.insert(key, val);
                        let _ = $target.insert(flip_key(key), val);
                        rotate_key(&mut key);
                    }
                }};
            }
            match (key.len(), val.len()) {
                (5, 11) => parse_into!(rules_2x2),
                (11, 19) => parse_into!(rules_3x3),
                _ => return Err(RuleParseError),
            }
        }
        Ok(ReplacementRules {
            rules_2x2,
            rules_3x3,
        })
    }
}
