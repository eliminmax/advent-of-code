// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 15 Part 1
use std::collections::{BTreeMap, BTreeSet};

// Debug impl at bottom of file
#[derive(PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
struct Location {
    y: usize,
    x: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Species {
    Elf,
    Goblin,
}

// Debug impl at bottom of file
#[derive(PartialEq, Clone)]
struct Fighter {
    species: Species,
    hp: u8,
}

type FighterGroup = BTreeMap<Location, Fighter>;

impl Fighter {
    fn new(species: Species) -> Self {
        Fighter { species, hp: 200 }
    }
}
#[derive(PartialEq)]
enum UpdateResult {
    BattleWon,
    FullRound,
}

#[derive(PartialEq)]
enum PathfindResult {
    EnemiesReachableBy(Location),
    UnreachableEnemies,
    EarlyVictory,
}

#[derive(Default)]
struct Battlefield {
    fighters: FighterGroup,
    moved: FighterGroup,
    locations: BTreeSet<Location>,
    width: usize,
    height: usize,
}

fn neighbors_of(loc: Location) -> impl Iterator<Item = Location> {
    vec![
        // make sure to return neighbors in reading order
        loc.y.checked_sub(1).map(|y| Location { y, ..loc }),
        loc.x.checked_sub(1).map(|x| Location { x, ..loc }),
        loc.x.checked_add(1).map(|x| Location { x, ..loc }),
        loc.y.checked_add(1).map(|y| Location { y, ..loc }),
    ]
    .into_iter()
    .flatten()
}

impl Battlefield {
    fn is_open(&self, loc: Location) -> bool {
        self.locations.contains(&loc) && self.get(&loc).is_none()
    }

    fn enemy_locations(&self, enemy: Species) -> BTreeSet<Location> {
        self.fighters
            .iter()
            .chain(self.moved.iter())
            .filter_map(|(&loc, f)| if f.species == enemy { Some(loc) } else { None })
            .collect()
    }

    /// Find the first step towards the nearest reachable enemy, returning None if no reachable
    /// enemies are found.
    fn pathfind_next(&self, start: Location, enemy: Species) -> PathfindResult {
        let targets = self.enemy_locations(enemy);
        if targets.is_empty() {
            return PathfindResult::EarlyVictory;
        }
        let in_range: BTreeSet<_> = targets
            .into_iter()
            .flat_map(|l| neighbors_of(l).filter(|l| self.is_open(*l)))
            .collect();

        let mut scores: BTreeMap<Location, (usize, Location)> =
            BTreeMap::from([(start, (0, start))]);
        let mut queue: Vec<(usize, Location, Location)> = neighbors_of(start)
            .filter_map(|l| {
                if self.is_open(l) {
                    Some((1, l, l))
                } else {
                    None
                }
            })
            .collect();
        for (s, l, _l) in queue.iter().cloned() {
            scores.insert(l, (s, l));
        }
        queue.sort_by(|a, b| (b.0, b.1).cmp(&(a.0, a.1)));
        while let Some((score, loc, mut start_loc)) = queue.pop() {
            if let Some(&(prev_score, prev_start)) = scores.get(&loc) {
                if prev_score < score {
                    continue;
                }
                start_loc = start_loc.min(prev_start);
            }
            for neighbor in neighbors_of(loc).filter(|l| self.locations.contains(l)) {
                if self.get(&neighbor).is_some_and(|f| f.species != enemy) {
                    continue;
                }
                let new_score = score + 1;
                if scores
                    .get(&neighbor)
                    .is_none_or(|prev| *prev > (new_score, start_loc))
                {
                    let _ = scores.insert(neighbor, (new_score, start_loc));
                    queue.push((new_score, neighbor, start_loc));
                }
            }
            queue.sort_by(|a, b| (b.0, b.1).cmp(&(a.0, a.1)));
        }

        scores.retain(|loc, _| in_range.contains(loc));
        let min_score = scores
            .into_iter()
            .map(|(end, (score, step))| (score, end, step))
            .min()
            .map(|(_s, _e, step)| step);

        if let Some(step) = min_score {
            PathfindResult::EnemiesReachableBy(step)
        } else {
            PathfindResult::UnreachableEnemies
        }
    }

    fn get(&self, loc: &Location) -> Option<&Fighter> {
        self.fighters.get(loc).or(self.moved.get(loc))
    }

    fn get_mut(&mut self, loc: &Location) -> Option<&mut Fighter> {
        self.fighters.get_mut(loc).or(self.moved.get_mut(loc))
    }

    fn update(&mut self) -> UpdateResult {
        while let Some((mut loc, fighter)) = self.fighters.pop_first() {
            if !neighbors_of(loc)
                .any(|l| self.get(&l).is_some_and(|f| f.species != fighter.species))
            {
                match self.pathfind_next(loc, !fighter.species) {
                    PathfindResult::EarlyVictory => {
                        self.fighters.insert(loc, fighter);
                        self.fighters.append(&mut self.moved);
                        return UpdateResult::BattleWon;
                    }
                    PathfindResult::EnemiesReachableBy(next_loc) => loc = next_loc,
                    PathfindResult::UnreachableEnemies => (),
                }
            }
            let mut targets: BTreeSet<Location> = BTreeSet::new();
            for neighbor in neighbors_of(loc) {
                match self.get_mut(&neighbor) {
                    None => (),
                    Some(ref target) if target.species != fighter.species => {
                        targets.insert(neighbor);
                    }
                    Some(_ally) => (),
                }
            }
            let chosen_target = targets
                .into_iter()
                .map(|target| {
                    (
                        self.get(&target).unwrap_or_else(|| unreachable!()).hp,
                        target,
                    )
                })
                .min()
                .map(|(_hp, loc)| loc);
            if let Some((target_loc, target)) =
                chosen_target.and_then(|l| self.get_mut(&l).map(|t| (l, t)))
            {
                target.hp = target.hp.saturating_sub(3);
                if target.hp == 0 {
                    self.fighters
                        .remove(&target_loc)
                        .or(self.moved.remove(&target_loc));
                }
            }
            self.moved.insert(loc, fighter);
        }
        assert!(self.fighters.is_empty());
        self.fighters.append(&mut self.moved);
        UpdateResult::FullRound
    }

    fn score_outcome(mut self) -> u32 {
        let mut rounds: u32 = 0;
        loop {
            eprintln!("Before round {rounds}:\n{self:?}");
            match self.update() {
                UpdateResult::FullRound => rounds += 1,
                UpdateResult::BattleWon => break,
            }
        }
        eprintln!("Outcome after {rounds} full rounds:\n{self:?}");
        self.fighters
            .into_values()
            .map(|f| u32::from(f.hp))
            .sum::<u32>()
            * rounds
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let battlefield: Battlefield = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!")
        .parse()
        .expect("Battlefield parsing failed");
    println!("{}", battlefield.score_outcome());
}

#[derive(Debug)]
struct UnparsableSymbol(#[allow(unused)] char);

impl std::str::FromStr for Battlefield {
    type Err = UnparsableSymbol;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut new_self = Self::default();
        for (y, row) in s.lines().enumerate() {
            new_self.height = new_self.height.max(y + 1);
            for (x, c) in row.chars().enumerate() {
                new_self.width = new_self.width.max(x + 1);
                match c {
                    '#' => (),
                    '.' => {
                        let _ = new_self.locations.insert(Location { x, y });
                    }
                    'E' | 'G' => {
                        let species = if c == 'E' {
                            Species::Elf
                        } else {
                            Species::Goblin
                        };
                        let loc = Location { x, y };
                        let _ = new_self.locations.insert(loc);
                        let _ = new_self.fighters.insert(loc, Fighter::new(species));
                    }
                    badchar => return Err(UnparsableSymbol(badchar)),
                }
            }
        }
        Ok(new_self)
    }
}

impl std::ops::Not for Species {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Species::Elf => Species::Goblin,
            Species::Goblin => Species::Elf,
        }
    }
}
impl std::fmt::Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl std::fmt::Debug for Fighter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let species_char = match self.species {
            Species::Goblin => 'G',
            Species::Elf => 'E',
        };
        write!(f, "{}({})", species_char, self.hp)
    }
}
impl std::fmt::Debug for Battlefield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut battlefield_chars = vec![vec!['█'; self.width]; self.height];
        let mut hp_displays: Vec<Vec<String>> = vec![Vec::new(); self.height];
        for Location { x, y } in self.locations.iter() {
            battlefield_chars[*y][*x] = ' ';
        }
        for (Location { x, y }, fighter) in self.fighters.iter() {
            battlefield_chars[*y][*x] = match fighter.species {
                Species::Goblin => 'G',
                Species::Elf => 'E',
            };
            hp_displays[*y].push(format!("{:?}", fighter));
        }
        let mut repr_string = String::new();
        for y in 0..self.height {
            repr_string.push_str(&battlefield_chars[y].iter().collect::<String>());
            if !hp_displays[y].is_empty() {
                repr_string.push_str("    ");
                repr_string.push_str(&hp_displays[y].join(", "));
            }
            repr_string.push('\n');
        }
        write!(f, "{}", repr_string)
    }
}
