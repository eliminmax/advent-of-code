// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 21 Part 2

use std::collections::{BTreeMap, HashMap, HashSet};

#[cfg(aoc_direct)]
const INPUT: &str = include_str!("input");
#[cfg(not(aoc_direct))]
const INPUT: &str = include_str!("../input");

/// A HashSet of static string slices derived from INPUT
type StaticStrSet = HashSet<&'static str>;

fn try_resolving(
    allergen: &'static str,
    suspects: &mut StaticStrSet,
    lists: &[(StaticStrSet, StaticStrSet)],
    unresolved: &mut StaticStrSet,
) -> Option<&'static str> {
    suspects.retain(|ingredient| unresolved.contains(ingredient));
    for (ingredients, allergens) in lists {
        if allergens.contains(allergen) {
            suspects.retain(|s| ingredients.contains(s));
        }
    }
    if suspects.len() == 1 {
        let resolved_to = suspects
            .iter()
            .copied()
            .next()
            .expect("will have element if len == 0");
        unresolved.remove(resolved_to);
        Some(resolved_to)
    } else {
        assert!(
            !suspects.is_empty(),
            "No possible candidates left for {allergen}"
        );
        None
    }
}

fn main() {
    let ingredients: StaticStrSet = INPUT
        .lines()
        .flat_map(|l| {
            l.split_once(" (contains")
                .map(|(ings, _)| ings)
                .unwrap()
                .split_ascii_whitespace()
        })
        .collect();

    let lists_with_allergens: Vec<(StaticStrSet, StaticStrSet)> = INPUT
        .lines()
        .map(|line| {
            line.split_once(" (contains ")
                .map(|(ingrs, alergs)| {
                    let alergs = alergs.strip_suffix(')').expect("Missing closing paren");
                    (
                        ingrs.split_ascii_whitespace().collect(),
                        alergs.split(", ").collect(),
                    )
                })
                .unwrap()
        })
        .collect();

    let mut unresolved_allergens: HashMap<&'static str, StaticStrSet> = INPUT
        .lines()
        .filter_map(|line| {
            line.split_once(" (contains ")
                .and_then(|(_, end)| end.strip_suffix(')'))
        })
        .flat_map(|l| l.split(", ").map(|s| (s, ingredients.clone())))
        .collect();
    let mut unresolved_ingredients = ingredients.clone();

    let mut allergens: BTreeMap<&'static str, &'static str> = BTreeMap::new();

    loop {
        let mut newly_resolved = StaticStrSet::with_capacity(unresolved_allergens.len());
        for (allergen, suspects) in unresolved_allergens.iter_mut() {
            if let Some(culprit) = try_resolving(
                allergen,
                suspects,
                &lists_with_allergens,
                &mut unresolved_ingredients,
            ) {
                allergens.insert(allergen, culprit);
                newly_resolved.insert(allergen);
            }
        }

        if newly_resolved.is_empty() {
            break;
        }

        unresolved_allergens.retain(|k, _| !newly_resolved.contains(k));
        if unresolved_allergens.is_empty() {
            break;
        }
    }

    let ordered_allergens: Vec<&'static str> = allergens.into_values().collect();
    println!("{}", ordered_allergens.join(","));
}
