// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 22 Part 2

#[derive(Debug, PartialEq, Copy, Clone)]
enum Effect {
    Shield = 0,
    Poison = 1,
    Recharge = 2,
}

#[derive(Debug, Copy, Clone, Default)]
struct EffectTimers([u8; 3]);

impl std::ops::Index<Effect> for EffectTimers {
    type Output = u8;
    fn index(&self, e: Effect) -> &u8 {
        &self.0[e as usize]
    }
}

impl std::ops::IndexMut<Effect> for EffectTimers {
    fn index_mut(&mut self, e: Effect) -> &mut u8 {
        &mut self.0[e as usize]
    }
}

impl EffectTimers {
    fn tick_down(&mut self) {
        self.0
            .iter_mut()
            .for_each(|timer| *timer = timer.saturating_sub(1));
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum SpellAction {
    Instant(fn(&mut GameState)),
    Persistent(Effect),
}

#[derive(Debug, PartialEq)]
struct Spell {
    name: &'static str,
    cost: u16,
    action: SpellAction,
}

#[derive(Debug, Clone)]
struct GameState {
    boss_hp: i16,
    boss_damage: i16,
    player_hp: i16,
    mana: u16,
    mana_spent: u16,
    effect_timers: EffectTimers,
}

#[derive(Debug)]
struct BossStatParseError;

impl std::str::FromStr for GameState {
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
        parse_stat!(boss_hp);
        parse_stat!(boss_damage);
        if lines.next().is_none() {
            Ok(GameState {
                boss_hp,
                boss_damage,
                player_hp: 50,
                mana: 500,
                mana_spent: 0,
                effect_timers: EffectTimers([0u8; 3]),
            })
        } else {
            Err(BossStatParseError)
        }
    }
}

impl GameState {
    fn turn_common(&mut self) {
        if self.effect_timers[Effect::Poison] != 0 {
            self.boss_hp -= 3;
        }
        if self.effect_timers[Effect::Recharge] != 0 {
            self.mana += 101;
        }
        self.effect_timers.tick_down();
    }

    fn boss_turn(&mut self) -> TurnOutcome {
        let damage = if self.effect_timers[Effect::Shield] == 0 {
            self.boss_damage
        } else {
            0.max(self.boss_damage - 7)
        };

        self.turn_common();
        if self.boss_hp <= 0 {
            return TurnOutcome::Won;
        }
        self.player_hp -= damage;
        if self.player_hp <= 0 {
            TurnOutcome::Lost
        } else {
            TurnOutcome::Running
        }
    }

    fn player_turn(&mut self, spell: &Spell) -> TurnOutcome {
        self.player_hp -= 1;
        if self.player_hp <= 0 {
            return TurnOutcome::Lost;
        }
        self.turn_common();
        if self.boss_hp <= 0 {
            return TurnOutcome::Won;
        }
        match spell {
            // can't afford to cast chosen spell
            Spell { cost, .. } if *cost > self.mana => return TurnOutcome::Lost,
            Spell { cost, action, .. } => {
                self.mana -= cost;
                self.mana_spent += cost;
                match action {
                    SpellAction::Instant(action_runner) => action_runner(self),
                    SpellAction::Persistent(effect) => {
                        if self.effect_timers[*effect] != 0 {
                            return TurnOutcome::Lost;
                        }
                        self.effect_timers[*effect] = match effect {
                            Effect::Shield | Effect::Poison => 6,
                            Effect::Recharge => 5,
                        }
                    }
                }
            }
        }
        if self.boss_hp <= 0 {
            TurnOutcome::Won
        } else {
            TurnOutcome::Running
        }
    }

    fn turn(&mut self, spell: &Spell) -> TurnOutcome {
        let player_turn_outcome = self.player_turn(spell);
        if player_turn_outcome == TurnOutcome::Running {
            self.boss_turn()
        } else {
            player_turn_outcome
        }
    }
}

const SPELLS: [Spell; 5] = [
    Spell {
        name: "Magic Missle",
        cost: 53,
        action: SpellAction::Instant(|state: &mut GameState| state.boss_hp -= 4),
    },
    Spell {
        name: "Drain",
        cost: 73,
        action: SpellAction::Instant(|state: &mut GameState| {
            state.boss_hp -= 2;
            state.player_hp += 2
        }),
    },
    Spell {
        name: "Shield",
        cost: 113,
        action: SpellAction::Persistent(Effect::Shield),
    },
    Spell {
        name: "Poison",
        cost: 173,
        action: SpellAction::Persistent(Effect::Poison),
    },
    Spell {
        name: "Recharge",
        cost: 229,
        action: SpellAction::Persistent(Effect::Recharge),
    },
];

fn find_min_mana(starting_state: GameState) -> u16 {
    use std::collections::VecDeque;
    let mut min_mana = u16::MAX;
    let mut queue: VecDeque<GameState> = VecDeque::from([starting_state]);

    while let Some(state) = queue.pop_front() {
        if state.mana_spent >= min_mana {
            continue;
        }
        for spell in SPELLS.iter() {
            let mut clone = state.clone();
            match clone.turn(spell) {
                TurnOutcome::Running => queue.push_back(clone),
                TurnOutcome::Won => min_mana = min_mana.min(clone.mana_spent),
                TurnOutcome::Lost => (),
            }
        }
    }

    min_mana
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let starting_state: GameState = input
        .parse()
        .expect("Failed to parse boss stats from input");
    let min_mana = find_min_mana(starting_state);
    println!("{min_mana}");
}

#[derive(Debug, PartialEq)]
enum TurnOutcome {
    Won,
    Running,
    Lost,
}
