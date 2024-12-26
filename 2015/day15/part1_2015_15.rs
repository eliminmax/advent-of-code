// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 15 Part 1

use std::convert::TryInto;
use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct IngredientParseError;

#[derive(Debug, PartialEq)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    _calories: i32,
}

impl FromStr for Ingredient {
    type Err = IngredientParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_ingredient, properties) = s.split_once(": ").ok_or(IngredientParseError)?;
        let mut properties = properties.split_whitespace();

        macro_rules! parse_property {
            ($prop: ident) => {
                if properties.next() != Some(stringify!($prop)) {
                    return Err(IngredientParseError);
                }
                let mut $prop = properties.next().ok_or(IngredientParseError)?;
                $prop = $prop.strip_suffix(',').unwrap_or($prop);
                let $prop = $prop.parse::<i32>().map_err(|_| IngredientParseError)?;
            };
        }

        parse_property!(capacity);
        parse_property!(durability);
        parse_property!(flavor);
        parse_property!(texture);
        parse_property!(calories);
        if properties.next().is_none() {
            Ok(Ingredient {
                capacity,
                durability,
                flavor,
                texture,
                _calories: calories,
            })
        } else {
            Err(IngredientParseError)
        }
    }
}

fn score(ingredients: &[Ingredient; 4], amounts: [i32; 4]) -> i32 {
    macro_rules! tally_property {
        ($prop: ident) => {
            let mut $prop = 0i32;
            for i in 0..4 {
                $prop += ingredients[i].$prop * amounts[i];
            }
            let $prop = $prop.max(0);
        }
    }
    tally_property!(capacity);
    tally_property!(durability);
    tally_property!(flavor);
    tally_property!(texture);
    capacity * durability * flavor * texture
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let ingredients: [Ingredient; 4] = input
        .lines()
        .map(|l| Ingredient::from_str(l).expect("failed to parse ingredient"))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Expected 4 ingredients");

    let mut max_score = 0i32;
    // I hate this, but it works (I write before actually completing it)
    for i0 in 0..=100i32 {
        for i1 in 0..=(100 - i0) {
            for i2 in 0..=(100 - (i0 + i1)) {
                let amounts: [i32; 4] = [i0, i1, i2, 100 - (i0 + i1 + i2)];
                max_score = max_score.max(score(&ingredients, amounts));
            }
        }
    }
    println!("{max_score}");
}
