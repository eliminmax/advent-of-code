// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 19 Part 1

// Some ideas for pruning the search space came from the following Reddit thread:
// https://www.reddit.com/r/adventofcode/comments/zpy5rm/

const MINUTES: u16 = 24;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct Inventory {
    ore: u16,
    clay: u16,
    obsidian: u16,
}

impl Inventory {
    const EMPTY: Self = Self {
        ore: 0,
        clay: 0,
        obsidian: 0,
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    geodes: u16,
    inventory: Inventory,
    ore_bots: u16,
    clay_bots: u16,
    obsidian_bots: u16,
}

impl State {
    const fn production(&self) -> Inventory {
        Inventory {
            ore: self.ore_bots,
            clay: self.clay_bots,
            obsidian: self.obsidian_bots,
        }
    }
    const START: Self = Self {
        inventory: Inventory::EMPTY,
        ore_bots: 1,
        clay_bots: 0,
        obsidian_bots: 0,
        geodes: 0,
    };
}

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u16,
    costs: costs::BotCosts,
}

impl Blueprint {
    fn into_neighbor_fn(self) -> impl Fn(State, u16) -> Vec<(State, u16)> {
        let max_ore_cost = [
            self.costs.ore.ore,
            self.costs.clay.ore,
            self.costs.obsidian.ore,
            self.costs.geode.ore,
        ]
        .into_iter()
        .max()
        .expect("iterator is nonempty");

        move |state, rounds| {
            let mut reachable = Vec::new();
            macro_rules! add_bot {
                {$bot_field: ident, ore: $ore_cost: expr} => {{
                    let mut inventory = state.inventory;
                    let mut resource_time = 0;
                    while inventory.ore < $ore_cost {
                        inventory = inventory + state.production();
                        resource_time += 1;
                    }
                    inventory.ore -= $ore_cost;
                    inventory = inventory + state.production();
                    let new_cost = resource_time + rounds + 1;
                    if new_cost < MINUTES {
                        reachable.push((
                            State {
                                inventory,
                                $bot_field: state.$bot_field + 1,
                                ..state
                            },
                            new_cost,
                        ));
                    }
                }};
            }

            if state.ore_bots < max_ore_cost {
                add_bot! {ore_bots, ore: self.costs.ore.ore}
            }
            if state.clay_bots < self.costs.obsidian.clay {
                add_bot! {clay_bots, ore: self.costs.clay.ore}
            }

            if state.clay_bots > 0 && state.obsidian_bots < self.costs.geode.obsidian {
                let mut inventory = state.inventory;
                let mut resource_time = 0;
                while inventory.clay < self.costs.obsidian.clay
                    || inventory.ore < self.costs.obsidian.ore
                {
                    inventory = inventory + state.production();
                    resource_time += 1;
                }
                inventory.ore -= self.costs.obsidian.ore;
                inventory.clay -= self.costs.obsidian.clay;
                inventory = inventory + state.production();
                let new_cost = resource_time + rounds + 1;
                if new_cost < MINUTES {
                    reachable.push((
                        State {
                            inventory,
                            obsidian_bots: state.obsidian_bots + 1,
                            ..state
                        },
                        new_cost,
                    ))
                }
            }

            if state.obsidian_bots > 0 {
                let mut inventory = state.inventory;
                let mut resource_time = 0;
                while inventory.ore < self.costs.geode.ore
                    || inventory.obsidian < self.costs.geode.obsidian
                {
                    inventory = inventory + state.production();
                    resource_time += 1;
                }
                inventory = inventory + state.production();
                let new_cost = resource_time + rounds + 1;
                if new_cost < MINUTES {
                    inventory.ore -= self.costs.geode.ore;
                    inventory.obsidian -= self.costs.geode.obsidian;
                    let geodes = state.geodes + (MINUTES - new_cost);
                    reachable.push((
                        State {
                            inventory,
                            geodes,
                            ..state
                        },
                        new_cost,
                    ));
                }
            }

            debug_assert!(reachable.iter().all(|&(_, cost)| cost <= MINUTES));
            if cfg!(debug_assertions) {
                eprintln!("{state:?}, {rounds} => {reachable:?}");
            }
            reachable
        }
    }

    fn quality(&self) -> u16 {
        use std::cmp::Reverse;
        use std::collections::{BinaryHeap, HashMap};

        let neighbor_fn = self.into_neighbor_fn();

        fn superior(new_state: State, new_cost: u16, old_state: State, old_cost: u16) -> bool {
            new_cost <= old_cost && (new_state, new_cost) != (old_state, old_cost) && {
                new_state.geodes >= old_state.geodes
                    && new_state.inventory.ore >= old_state.inventory.ore
                    && new_state.inventory.clay >= old_state.inventory.clay
                    && new_state.inventory.obsidian >= old_state.inventory.obsidian
                    && new_state.ore_bots >= old_state.ore_bots
                    && new_state.clay_bots >= old_state.clay_bots
                    && new_state.obsidian_bots >= old_state.obsidian_bots
            }
        }

        let mut costs = HashMap::from([(State::START, 0)]);
        let mut queue = BinaryHeap::from([Reverse((0, State::START))]);

        'outer: while let Some(Reverse((cost, node))) = queue.pop() {
            if costs.get(&node).is_none_or(|&c| c < cost) {
                continue;
            }
            for (&s, &c) in costs.iter() {
                if superior(s, c, node, cost) {
                    continue 'outer;
                }
            }

            for (neighbor, next_cost) in neighbor_fn(node, cost) {
                costs.insert(neighbor, next_cost);
                if cfg!(debug_assertions) {
                    let l = costs.len();
                    costs.retain(|&s, &mut c| !superior(s, c, neighbor, next_cost));
                    let nl = costs.len();
                    if nl != l {
                        eprintln!(
                            "{} nodes pruned for being inferior to ({neighbor:?}, {next_cost})",
                            l - nl
                        );
                    }
                }
                queue.push(Reverse((next_cost, neighbor)));
            }
        }
        if cfg!(debug_assertions) {
            eprintln!("{costs:?}");
        }
        costs
            .into_keys()
            .map(|State { geodes, .. }| geodes)
            .max()
            .unwrap_or_default()
            * self.id
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    use std::thread;

    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let mut handles = Vec::new();
    for line in input.lines() {
        let bp = line.parse::<Blueprint>().unwrap();
        handles.push(thread::spawn(move || bp.quality()));
    }

    let mut total_qual = 0;

    for handle in handles {
        total_qual += handle.join().unwrap();
    }
    println!("{total_qual}");
}

mod costs {
    //! A module with structs that exist to allow named fields and subfields, as rust doesn't
    //! support C-style inline structs. I want to use rust, but also want to do the equivalent to
    //! the following:
    //! ```c
    //! typedef struct {
    //!     uint8_t id;
    //!     struct {
    //!         struct { uint8_t ore; } ore_bot;
    //!         struct { uint8_t ore; } clay_bot;
    //!         struct { uint8_t ore; uint8_t clay; } obsidian_bot;
    //!         struct { uint8_t ore; uint8_t obsidian; } geode_bot;
    //!     } costs;
    //! } Blueprint;
    //! ```

    /// a cost consisting of an amount of ore
    #[derive(Debug, Clone, Copy)]
    pub struct Ore {
        pub ore: u16,
    }

    /// a cost consisting of an amount of ore and an amount of clay
    #[derive(Debug, Clone, Copy)]
    pub struct OreClay {
        pub ore: u16,
        pub clay: u16,
    }

    /// a cost consisting of an amount of ore and an amount of obsidian
    #[derive(Debug, Clone, Copy)]
    pub struct OreObsidian {
        pub ore: u16,
        pub obsidian: u16,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct BotCosts {
        pub ore: Ore,
        pub clay: Ore,
        pub obsidian: OreClay,
        pub geode: OreObsidian,
    }
}

mod std_impls {
    use super::*;

    use std::cmp::{Ord, Ordering, PartialOrd};
    use std::error::Error;
    use std::fmt::{self, Display};
    use std::num::ParseIntError;
    use std::ops::Add;
    use std::str::FromStr;

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            let a = (
                self.geodes,
                self.clay_bots,
                self.obsidian_bots,
                self.ore_bots,
                self.inventory,
            );
            let b = (
                other.geodes,
                other.clay_bots,
                other.obsidian_bots,
                other.ore_bots,
                other.inventory,
            );
            a.cmp(&b)
        }
    }

    impl Add<Inventory> for Inventory {
        type Output = Inventory;

        fn add(self, rhs: Inventory) -> Self::Output {
            Inventory {
                ore: self.ore + rhs.ore,
                clay: self.clay + rhs.clay,
                obsidian: self.obsidian + rhs.obsidian,
            }
        }
    }

    #[derive(Debug)]
    pub enum BlueprintParseError {
        IntParse(ParseIntError),
        MismatchedToken {
            expected: &'static str,
            got: Box<str>,
        },
        ExtraToken(Box<str>),
        MissingField(&'static str),
        MissingToken(&'static str),
    }

    impl FromStr for Blueprint {
        type Err = BlueprintParseError;

        // a rather gnarly parser implementation
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut words = s.split_ascii_whitespace();
            macro_rules! consume_tokens {
                ($tok: literal) => {{
                    let tok = words.next().ok_or(Self::Err::MissingToken($tok))?;
                    if tok != $tok {
                        return Err(Self::Err::MismatchedToken {
                            expected: $tok,
                            got: Box::from(tok),
                        });
                    }
                }};
                ($tok: literal $($remaining: literal)+) => {
                    consume_tokens!($tok);
                    consume_tokens!($($remaining)+);
                };
            }

            consume_tokens!("Blueprint");
            let id = words
                .next()
                .ok_or(Self::Err::MissingField("blueprint number"))?
                .strip_suffix(':')
                .ok_or(Self::Err::MissingToken("blueprint number colon"))?
                .parse()?;
            let costs = {
                macro_rules! parse_field {
                    ($field_desc: expr) => {
                        words
                            .next()
                            .ok_or(Self::Err::MissingField($field_desc))?
                            .parse()?
                    };
                }
                macro_rules! parse_cost {
                    {$bot_type: literal} => {{
                        consume_tokens!("Each" $bot_type "robot" "costs");
                        let ore = parse_field!(concat!($bot_type, " robot cost"));
                        consume_tokens!("ore.");
                        costs::Ore { ore }
                    }};
                }
                let ore_bot = parse_cost! { "ore" };
                let clay_bot = parse_cost! { "clay" };

                let obsidian_bot = {
                    consume_tokens!("Each" "obsidian" "robot" "costs");
                    let ore = words
                        .next()
                        .ok_or(Self::Err::MissingField("obsidian robot ore cost"))?
                        .parse()?;
                    consume_tokens!("ore" "and");
                    let clay = words
                        .next()
                        .ok_or(Self::Err::MissingField("obsidian robot clay cost"))?
                        .parse()?;
                    consume_tokens!("clay.");
                    costs::OreClay { ore, clay }
                };

                let geode_bot = {
                    consume_tokens!("Each" "geode" "robot" "costs");
                    let ore = words
                        .next()
                        .ok_or(Self::Err::MissingField("geode robot ore cost"))?
                        .parse()?;
                    consume_tokens!("ore" "and");
                    let obsidian = words
                        .next()
                        .ok_or(Self::Err::MissingField("geode robot obsidian cost"))?
                        .parse()?;

                    consume_tokens!("obsidian.");
                    costs::OreObsidian { ore, obsidian }
                };

                costs::BotCosts {
                    ore: ore_bot,
                    clay: clay_bot,
                    obsidian: obsidian_bot,
                    geode: geode_bot,
                }
            };
            if let Some(t) = words.next() {
                Err(Self::Err::ExtraToken(Box::from(t)))
            } else {
                Ok(Self { id, costs })
            }
        }
    }

    impl Display for BlueprintParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::IntParse(e) => write!(f, "failed to parse integer: {e}"),
                Self::ExtraToken(tok) => write!(f, "leftover tokens in input: {tok:?}"),
                Self::MissingToken(tok) => write!(f, "missing token in input: {tok:?}"),
                Self::MismatchedToken { expected, got } => {
                    write!(f, "mismatched tokens: expected {expected:?}, got {got:?}")
                }
                Self::MissingField(field) => write!(f, "missing value for {field}"),
            }
        }
    }

    impl Error for BlueprintParseError {}

    impl Default for State {
        fn default() -> Self {
            Self::START
        }
    }

    impl From<ParseIntError> for BlueprintParseError {
        fn from(value: ParseIntError) -> Self {
            Self::IntParse(value)
        }
    }
}
