// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 14 Part 1

use std::collections::{BTreeMap, HashMap};

#[derive(Debug, PartialEq)]
struct Recipe<'a> {
    out_amnt: u64,
    ingredients: HashMap<&'a str, u64>,
}

fn needed_ore<'a>(recipes: HashMap<&'a str, Recipe<'a>>) -> u64 {
    // using a BTreeMap to be able to pop items.
    let mut needed = BTreeMap::from([("FUEL", 1_u64)]);

    let mut ore_amount: u64 = 0;
    let mut leftovers: HashMap<&'a str, u64> = HashMap::new();

    while let Some((item_to_make, mut amount_to_make)) = needed.pop_last() {
        // can make infinite ore, but need to track how much is made.
        if item_to_make == "ORE" {
            ore_amount += amount_to_make;
            continue;
        }

        leftovers.entry(item_to_make).and_modify(|held_amnt| {
            let remaining = held_amnt.saturating_sub(amount_to_make);
            amount_to_make = amount_to_make.saturating_sub(*held_amnt);
            *held_amnt = remaining;
        });

        let out_amnt = recipes[item_to_make].out_amnt;
        let amount_made = amount_to_make.next_multiple_of(out_amnt);
        let extra = amount_made - amount_to_make;
        leftovers
            .entry(item_to_make)
            .and_modify(|i| *i += extra)
            .or_insert(extra);

        for (needed_item, needed_amount) in recipes[item_to_make].ingredients.iter() {
            needed
                .entry(needed_item)
                .and_modify(|i| *i += (amount_made / out_amnt) * needed_amount)
                .or_insert((amount_made / out_amnt) * needed_amount);
        }
    }
    ore_amount
}

fn parse_recipes<'a>(s: &'a str) -> HashMap<&'a str, Recipe<'a>> {
    let mut recipes: HashMap<&'a str, Recipe> = HashMap::new();

    for line in s.lines() {
        let (ingredients, output) = line.trim().split_once(" => ").unwrap();
        let (out_amnt, out_item): (u64, &'a str) = output
            .split_once(" ")
            .map(|(a, i)| (a.parse().unwrap(), i))
            .unwrap();

        let ingredients = ingredients
            .split(", ")
            .map(|s| {
                s.split_once(" ")
                    .map(|(a, i)| (i, a.parse().unwrap()))
                    .unwrap()
            })
            .collect();

        let prev_recipe = recipes.insert(
            out_item,
            Recipe {
                out_amnt,
                ingredients,
            },
        );
        assert!(
            prev_recipe.is_none(),
            "solution assumes a unique way to construct each item"
        );
    }
    recipes
}

fn main() {
    println!("{}", needed_ore(parse_recipes(include_str!("input"))));
}
