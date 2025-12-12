// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD
use std::collections::HashMap;
// Solution to AoC 2021 Day 21 Part 2
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct GameState {
    p1_score: u8,
    p2_score: u8,
    p1_space: u8,
    p2_space: u8,
    p1_turn: bool,
}

#[derive(Default, Debug, PartialEq, Clone, Copy)]
struct Outcome {
    p1_wins: u64,
    p2_wins: u64,
}

impl std::ops::Add for Outcome {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            p1_wins: self.p1_wins + rhs.p1_wins,
            p2_wins: self.p2_wins + rhs.p2_wins,
        }
    }
}

impl std::ops::AddAssign for Outcome {
    fn add_assign(&mut self, rhs: Self) {
        self.p1_wins += rhs.p1_wins;
        self.p2_wins += rhs.p2_wins;
    }
}

impl std::ops::Mul<u64> for Outcome {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self::Output {
        Self {
            p1_wins: self.p1_wins * rhs,
            p2_wins: self.p2_wins * rhs,
        }
    }
}

// an array of all possible trios of Dirac dice rolls
const ROLLS: [[u8; 3]; 27] = {
    let mut arr: [[u8; 3]; 27] = [[0; 3]; 27];
    let mut i = 0;

    let mut roll = [1; 3];

    while roll[0] <= 3 {
        roll[1] = 1;
        while roll[1] <= 3 {
            roll[2] = 1;
            while roll[2] <= 3 {
                arr[i] = roll;
                i += 1;
                roll[2] += 1;
            }
            roll[1] += 1;
        }
        roll[0] += 1;
    }

    arr
};

fn run_game(state: GameState, cache: &mut HashMap<GameState, Outcome>) -> Outcome {
    if let Some(outcomes) = cache.get(&state).cloned() {
        return outcomes;
    }
    let mut outcomes = Outcome::default();
    for [a, b, c] in ROLLS {
        let total_roll = a + b + c;
        let next_state = if state.p1_turn {
            let new_space = (state.p1_space + total_roll - 1) % 10 + 1;
            GameState {
                p1_score: state.p1_score + new_space,
                p1_space: new_space,
                p1_turn: false,
                ..state
            }
        } else {
            let new_space = (state.p2_space + total_roll - 1) % 10 + 1;
            GameState {
                p2_score: state.p2_score + new_space,
                p2_space: new_space,
                p1_turn: true,
                ..state
            }
        };

        if state.p1_turn {
            if next_state.p1_score >= 21 {
                outcomes.p1_wins += 1;
                continue;
            }
        } else if next_state.p2_score >= 21 {
            outcomes.p2_wins += 1;
            continue;
        }

        outcomes += run_game(next_state, cache);
    }
    cache.insert(state, outcomes);
    outcomes
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let (p1_start, p2_start) = input.trim().split_once('\n').unwrap();
    let p1_space = p1_start
        .strip_prefix("Player 1 starting position: ")
        .unwrap()
        .parse()
        .unwrap();
    let p2_space = p2_start
        .strip_prefix("Player 2 starting position: ")
        .unwrap()
        .parse()
        .unwrap();
    let start_state = GameState {
        p1_score: 0,
        p2_score: 0,
        p1_space,
        p2_space,
        p1_turn: true,
    };
    let Outcome { p1_wins, p2_wins } = run_game(start_state, &mut HashMap::new());
    println!("{}", p1_wins.max(p2_wins));
}
