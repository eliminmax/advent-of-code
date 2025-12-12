// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 22 Part 2

// Both for fun, and to make sure I was following the description, I decided to have debug builds
// try to match the play-by-play of the example setup. It matches it exactly, though it uses
// global, static variables to track the previous round, and the current round counter.
//
// I believe that's acceptable in this case, as they're just there for debugging.

use std::collections::{HashSet, VecDeque};

#[cfg(debug_assertions)]
use std::sync::Mutex;

type Deck = VecDeque<u8>;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Combatant {
    Player1,
    Player2,
}

#[derive(Debug, PartialEq, Clone)]
struct GameOutcome {
    winner: Combatant,
    deck: Deck,
}

// using cfg(debug_assertions) as a way to check for debug builds
//
// Need to use a Mutex, as static variables must be Sync, even though it's a single-threaded
// program
#[cfg(debug_assertions)]
static COUNTER: Mutex<std::ops::RangeFrom<u32>> = Mutex::new(1..);
#[cfg(debug_assertions)]
static GAME_TRACKER: Mutex<Vec<u32>> = Mutex::new(Vec::new());

#[cfg(debug_assertions)]
fn display_deck(deck: &Deck) -> String {
    if deck.is_empty() {
        return String::new();
    }
    let (a, b) = deck.as_slices();
    a.iter()
        .chain(b.iter())
        .skip(1)
        .fold(format!("{}", deck[0]), |mut builder, x| {
            use std::fmt::Write;
            write!(&mut builder, ", {x}").expect("Writing to string always succeeds");
            builder
        })
}

fn run_round(deck_1: &mut Deck, deck_2: &mut Deck) {
    let card_1 = deck_1.pop_front().unwrap();
    let card_2 = deck_2.pop_front().unwrap();
    if usize::from(card_1) > deck_1.len() || usize::from(card_2) > deck_2.len() {
        use std::cmp::Ordering;
        match card_1.cmp(&card_2) {
            Ordering::Greater => {
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
                #[cfg(debug_assertions)]
                eprint!("Player 1 wins");
            }
            Ordering::Less => {
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
                #[cfg(debug_assertions)]
                eprint!("Player 2 wins");
            }
            Ordering::Equal => {
                panic!("someone cheated!");
            }
        }
    } else {
        #[cfg(debug_assertions)]
        eprintln!("Playing a sub-game to determine the winner...\n");
        let subdeck_1 = deck_1.make_contiguous()[0..usize::from(card_1)]
            .iter()
            .cloned()
            .collect();
        let subdeck_2 = deck_2.make_contiguous()[0..usize::from(card_2)]
            .iter()
            .cloned()
            .collect();
        match run_game(subdeck_1, subdeck_2).winner {
            Combatant::Player1 => {
                deck_1.push_back(card_1);
                deck_1.push_back(card_2);
                #[cfg(debug_assertions)]
                eprint!("Player 1 wins");
            }
            Combatant::Player2 => {
                deck_2.push_back(card_2);
                deck_2.push_back(card_1);
                #[cfg(debug_assertions)]
                eprint!("Player 2 wins");
            }
        }
    }
}

fn run_game(mut deck_1: Deck, mut deck_2: Deck) -> GameOutcome {
    #[cfg(debug_assertions)]
    let (game_nr, mut round_counter) = { (COUNTER.lock().unwrap().next().unwrap(), 1) };
    #[cfg(debug_assertions)]
    {
        GAME_TRACKER.lock().unwrap().push(game_nr);
        eprintln!("=== Game {game_nr} ===\n");
    }

    let mut seen_states: HashSet<(Deck, Deck)> = HashSet::new();
    while (!deck_1.is_empty()) && (!deck_2.is_empty()) {
        #[cfg(debug_assertions)]
        {
            eprintln!("-- Round {round_counter} (Game {game_nr}) --");
            eprintln!("Player 1's deck: {}", display_deck(&deck_1));
            eprintln!("Player 2's deck: {}", display_deck(&deck_2));
            eprintln!("Player 1 plays: {}", deck_1[0]);
            eprintln!("Player 2 plays: {}", deck_2[0]);
        }

        if seen_states.insert((deck_1.clone(), deck_2.clone())) {
            run_round(&mut deck_1, &mut deck_2);
            #[cfg(debug_assertions)]
            {
                eprintln!(" round {round_counter} of game {game_nr}!");
                round_counter += 1;
                if (!deck_1.is_empty()) && (!deck_2.is_empty()) {
                    eprintln!();
                }
            }
        } else {
            #[cfg(debug_assertions)]
            {
                eprintln!("Decks have already been seen this game, so player 1 wins!");
                let mut lock = GAME_TRACKER.lock().unwrap();
                assert_eq!(lock.pop(), Some(game_nr));
                eprintln!("...anyway, back to game {}.", lock.last().unwrap());
            }
            return GameOutcome {
                winner: Combatant::Player1,
                deck: deck_1,
            };
        }
    }
    let outcome = if deck_2.is_empty() {
        #[cfg(debug_assertions)]
        eprintln!("The winner of game {game_nr} is player 1!\n");
        GameOutcome {
            winner: Combatant::Player1,
            deck: deck_1,
        }
    } else {
        #[cfg(debug_assertions)]
        eprintln!("The winner of game {game_nr} is player 2!\n");
        GameOutcome {
            winner: Combatant::Player2,
            deck: deck_2,
        }
    };

    #[cfg(debug_assertions)]
    if game_nr > 1 {
        let mut lock = GAME_TRACKER.lock().unwrap();
        assert_eq!(lock.pop(), Some(game_nr));
        eprintln!("...anyway, back to game {}.", lock.last().unwrap());
    }
    outcome
}

fn primary_game(deck_1: Deck, deck_2: Deck) -> u32 {
    #[cfg_attr(
        not(debug_assertions),
        expect(unused_variables, reason = "used for debug eprintln!s only")
    )]
    let GameOutcome { winner, deck } = run_game(deck_1, deck_2);

    #[cfg(debug_assertions)]
    {
        eprintln!("\n== Post-game results ==");
        match winner {
            Combatant::Player1 => {
                eprintln!("Player 1's deck: {}", display_deck(&deck));
                eprintln!("Player 2's deck: ");
            }
            Combatant::Player2 => {
                eprintln!("Player 1's deck: ");
                eprintln!("Player 2's deck: {}", display_deck(&deck));
            }
        }
    }

    score(deck)
}

fn score(deck: Deck) -> u32 {
    let mut multiplier = u32::try_from(deck.len()).unwrap();
    let mut score = 0;
    for card in deck {
        score += u32::from(card) * multiplier;
        multiplier -= 1;
    }
    score
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let (player_1, player_2) = input.split_once("\n\n").unwrap();

    let deck_1 = player_1.strip_prefix("Player 1:\n").unwrap().lines();
    let deck_2 = player_2.strip_prefix("Player 2:\n").unwrap().lines();

    let deck_1: Deck = deck_1.map(|l| l.parse().unwrap()).collect();
    let deck_2: Deck = deck_2.map(|l| l.parse().unwrap()).collect();

    println!("{}", primary_game(deck_1, deck_2));
}
