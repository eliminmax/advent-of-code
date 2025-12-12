// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 24 Part 2

use std::borrow::Cow;
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Damage<'a> {
    element: &'a str,
    amount: u32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Side {
    ImmuneSystem,
    Infection,
}

#[derive(Debug, Clone)]
struct ArmyGroup<'a> {
    id: usize,
    targeted: bool,
    target: Option<usize>,
    initiative: u32,
    units: u32,
    hit_points: u32,
    damage: Damage<'a>,
    side: Side,
    immunities: HashSet<&'a str>,
    weaknesses: HashSet<&'a str>,
}

impl<'a> ArmyGroup<'a> {
    fn parse(
        s: &'a str,
        counter: &mut std::ops::RangeFrom<usize>,
        side: Side,
    ) -> Result<Self, Cow<'a, str>> {
        let (unit_count, remaining) = s.split_once(" units each with ").ok_or(s)?;
        let units: u32 = unit_count.parse().map_err(|_| s)?;

        let (hp_count, mut remaining) = remaining.split_once(" hit points ").ok_or(s)?;
        let hit_points: u32 = hp_count.parse().map_err(|e| format!("{e:?} ({s})"))?;

        let mut immunities = HashSet::new();
        let mut weaknesses = HashSet::new();

        if remaining.starts_with('(') {
            let (affinities, new_remaining) = remaining.split_once(')').ok_or(s)?;
            remaining = new_remaining.strip_prefix(' ').ok_or(s)?;

            let affinities: Vec<&'a str> =
                affinities.strip_prefix('(').unwrap().split("; ").collect();
            for grouping in affinities {
                if grouping.starts_with("weak to ") {
                    weaknesses.extend(grouping.strip_prefix("weak to ").unwrap().split(", "));
                } else if grouping.starts_with("immune to ") {
                    immunities.extend(grouping.strip_prefix("immune to ").unwrap().split(", "));
                }
            }
        }

        let (damage, initiative) = remaining
            .strip_prefix("with an attack that does ")
            .ok_or(s)?
            .split_once(" damage at initiative ")
            .ok_or(s)?;

        let (amount, element) = damage.split_once(' ').ok_or(s)?;
        let amount: u32 = amount.parse().map_err(|e| format!("{e:?} ({s})"))?;
        let damage = Damage { element, amount };

        let initiative: u32 = initiative.parse().map_err(|e| format!("{e:?} ({s})"))?;

        Ok(Self {
            id: counter.next().unwrap(),
            target: None,
            targeted: false,
            side,
            initiative,
            units,
            hit_points,
            damage,
            immunities,
            weaknesses,
        })
    }
}

impl Damage<'_> {
    fn damage_against(self, enemy: &ArmyGroup<'_>) -> u32 {
        if enemy.weaknesses.contains(self.element) {
            self.amount * 2
        } else if enemy.immunities.contains(self.element) {
            0
        } else {
            self.amount
        }
    }
}

impl ArmyGroup<'_> {
    fn effective_power(&self) -> u32 {
        self.units * self.damage.amount
    }

    fn damage_against(&self, other: &ArmyGroup<'_>) -> u32 {
        self.damage.damage_against(other) * self.units
    }
}

type Battlefield<'a> = Vec<RefCell<ArmyGroup<'a>>>;

fn set_targets(army_groups: &mut Battlefield<'_>) {
    army_groups.sort_by_key(|ag| {
        let ag = ag.borrow();
        Reverse((ag.effective_power(), ag.initiative))
    });
    for ag in army_groups.iter() {
        let mut ag = ag.borrow_mut();
        ag.target = None;
        ag.targeted = false;
    }
    for ag in army_groups.iter() {
        let side: Side;
        {
            side = ag.borrow().side;
        }

        let target_id = army_groups
            .iter()
            .filter(|other| {
                let other = other.borrow();
                side != other.side && !other.targeted && (ag.borrow().damage_against(&other) > 0)
            })
            .max_by_key(|enemy| {
                let enemy = enemy.borrow();
                (
                    ag.borrow().damage_against(&enemy),
                    enemy.effective_power(),
                    enemy.initiative,
                )
            })
            .map(|target| target.borrow().id);

        ag.borrow_mut().target = target_id;
        if let Some(target_id) = target_id {
            army_groups
                .iter()
                .find(move |&ag| ag.borrow().id == target_id)
                .unwrap()
                .borrow_mut()
                .targeted = true;
        }
    }
}

fn run_attacks(army_groups: &mut Battlefield<'_>) {
    army_groups.sort_by_key(|ag| {
        let ag = ag.borrow();
        Reverse(ag.initiative)
    });

    for ag in army_groups.as_slice() {
        let ag = ag.borrow();

        let Some(target_id) = ag.target else {
            continue;
        };
        let mut target = army_groups
            .iter()
            .find(move |&ag| ag.borrow().id == target_id)
            .unwrap()
            .borrow_mut();

        let damage = ag.damage_against(&target);
        let kill_count = target.units.min(damage / target.hit_points);

        target.units -= kill_count;
    }

    army_groups.retain(|ag| ag.borrow().units != 0);
}

fn fight(boost: u32, mut army_groups: Battlefield<'_>) -> Option<u32> {
    for ag in army_groups
        .iter_mut()
        .filter(|ag| ag.borrow().side == Side::ImmuneSystem)
    {
        ag.borrow_mut().damage.amount += boost;
    }
    'battle: loop {
        let total_units: u32 = army_groups.iter().map(|ag| ag.borrow().units).sum();
        set_targets(&mut army_groups);
        run_attacks(&mut army_groups);

        // Check for stalemate - that leaves infection around, so is considered a failure
        if army_groups.iter().map(|ag| ag.borrow().units).sum::<u32>() == total_units {
            return None;
        }

        let (mut seen_immune, mut seen_infection) = (false, false);

        for ag in army_groups.as_slice() {
            match ag.borrow().side {
                Side::ImmuneSystem => seen_immune = true,
                Side::Infection => seen_infection = true,
            }
            if seen_immune && seen_infection {
                continue 'battle;
            }
        }

        // only reached if only one side remains
        if seen_immune {
            return Some(
                army_groups
                    .into_iter()
                    .map(|ag| ag.into_inner().units)
                    .sum(),
            );
        } else {
            return None;
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let (immune, infection) = input.split_once("\n\n").unwrap();
    let mut counter: std::ops::RangeFrom<usize> = 0..;
    let mut army_groups: Vec<_> = immune
        .strip_prefix("Immune System:\n")
        .unwrap()
        .lines()
        .map(|l| {
            RefCell::new(ArmyGroup::parse(l.trim(), &mut counter, Side::ImmuneSystem).unwrap())
        })
        .collect();
    army_groups.extend(
        infection
            .strip_prefix("Infection:\n")
            .unwrap()
            .lines()
            .map(|l| {
                RefCell::new(ArmyGroup::parse(l.trim(), &mut counter, Side::Infection).unwrap())
            }),
    );

    let (mut min, mut max): (u32, u32) = (0, 100_000);
    assert!(
        fight(max, army_groups.clone()).is_some(),
        "Starting upper bound {max} too small"
    );
    while min + 1 < max {
        let midpoint = min.midpoint(max);
        if fight(midpoint, army_groups.clone()).is_some() {
            max = midpoint;
        } else {
            min = midpoint;
        }
    }
    debug_assert_eq!(min + 1, max);
    debug_assert!(fight(min, army_groups.clone()).is_none());
    println!("{}", fight(max, army_groups.clone()).unwrap());
}
