// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 21 Part 1

use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
struct GameItem {
    cost: i16,
    damage: i16,
    armor: i16,
}

const WEAPONS: [GameItem; 5] = [
    GameItem {
        // Dagger
        cost: 8,
        damage: 4,
        armor: 0,
    },
    GameItem {
        // Shortsword
        cost: 10,
        damage: 5,
        armor: 0,
    },
    GameItem {
        // Warhammer
        cost: 25,
        damage: 6,
        armor: 0,
    },
    GameItem {
        // Longsword
        cost: 40,
        damage: 7,
        armor: 0,
    },
    GameItem {
        // Greataxe
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMORS: [Option<GameItem>; 6] = [
    Some(GameItem {
        // Leather
        cost: 13,
        damage: 0,
        armor: 1,
    }),
    Some(GameItem {
        // Chainmail
        cost: 31,
        damage: 0,
        armor: 2,
    }),
    Some(GameItem {
        // Splintmail
        cost: 53,
        damage: 0,
        armor: 3,
    }),
    Some(GameItem {
        // Bandedmail
        cost: 75,
        damage: 0,
        armor: 4,
    }),
    Some(GameItem {
        // Platemail
        cost: 102,
        damage: 0,
        armor: 5,
    }),
    None,
];

const RINGS: [GameItem; 6] = [
    GameItem {
        // Damage +1
        cost: 25,
        damage: 1,
        armor: 0,
    },
    GameItem {
        // Damage +2
        cost: 50,
        damage: 2,
        armor: 0,
    },
    GameItem {
        // Damage +3
        cost: 100,
        damage: 3,
        armor: 0,
    },
    GameItem {
        // Defense +1
        cost: 20,
        damage: 0,
        armor: 1,
    },
    GameItem {
        // Defense +2
        cost: 40,
        damage: 0,
        armor: 2,
    },
    GameItem {
        // Defense +3
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

#[derive(Debug)]
struct BossStats {
    hp: i16,
    damage: i16,
    armor: i16,
}

#[derive(Debug)]
struct BossStatParseError;

impl std::str::FromStr for BossStats {
    type Err = BossStatParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        macro_rules! parse_stat {
            ($stat: ident) => {
                let (_, stat_str) = lines
                    .next()
                    .ok_or(BossStatParseError)?
                    .split_once(": ")
                    .ok_or(BossStatParseError)?;
                let $stat = stat_str.parse::<i16>().map_err(|_| BossStatParseError)?;
            };
        }
        parse_stat!(hp);
        parse_stat!(damage);
        parse_stat!(armor);
        if lines.next().is_none() {
            Ok(BossStats { hp, damage, armor })
        } else {
            Err(BossStatParseError)
        }
    }
}

/// Return Some(cost) if inventory would result in victory, otherwise None
fn test_inventory(boss_stats: &BossStats, inventory: Vec<GameItem>) -> Option<i16> {
    macro_rules! sum_stat {
        ($stat: ident) => {{
            inventory.iter().map(|i| i.$stat).sum::<i16>()
        }};
    }
    let mut boss_hp = boss_stats.hp;
    let mut player_hp = 100_i16;
    let player_damage: i16 = sum_stat!(damage) - boss_stats.armor;
    let boss_damage: i16 = boss_stats.damage - sum_stat!(armor);
    loop {
        boss_hp -= player_damage;
        if boss_hp <= 0 {
            return Some(sum_stat!(cost));
        }
        player_hp -= boss_damage;
        if player_hp <= 0 {
            return None;
        }
    }
}

#[derive(Debug)]
#[must_use]
/// Think of this like an Option, but with the potential of either 0, 1, or 2 items, rather than
/// just 0 or 1 items
enum RingSelection {
    Two(GameItem, GameItem),
    One(GameItem),
    None,
}

#[derive(Debug, Default)]
struct RingChoices(usize, usize);
impl Iterator for RingChoices {
    type Item = RingSelection;
    fn next(&mut self) -> Option<RingSelection> {
        use RingSelection as RS;
        const LEN: usize = RINGS.len();
        let ret: Option<RS> = match self.0 {
            i if i < LEN && i == self.1 => Some(RS::One(RINGS[i])),
            i if i < LEN => Some(RS::Two(RINGS[i], RINGS[self.1])),
            i if i == LEN => Some(RS::None),
            _ => None,
        };

        self.1 += 1;
        if self.1 >= LEN {
            self.0 += 1;
            // don't reset if self.0 == LEN to avoid `LEN` `Some(RS::None)`s in a row
            if self.0 < LEN {
                self.1 = 0;
            }
        }
        ret
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let boss_stats: BossStats = input
        .parse()
        .expect("Failed to parse boss stats from input");
    let mut min_cost: Option<i16> = None;

    for weapon in WEAPONS.iter() {
        for armor in ARMORS.iter() {
            for rings in RingChoices::default() {
                let mut inventory = vec![*weapon];
                if let Some(a) = armor {
                    inventory.push(*a)
                }
                match rings {
                    RingSelection::Two(a, b) => inventory.extend_from_slice(&[a, b]),
                    RingSelection::One(a) => inventory.push(a),
                    RingSelection::None => (),
                }
                if let Some(cost) = test_inventory(&boss_stats, inventory) {
                    if min_cost.is_none_or(|old_min| old_min > cost) {
                        min_cost = Some(cost);
                    }
                }
            }
        }
    }

    println!("{}", min_cost.expect("No inventory could beat the boss"));
}
