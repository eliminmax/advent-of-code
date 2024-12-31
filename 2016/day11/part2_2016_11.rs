// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 11 Part 2
use std::collections::{self, VecDeque};

#[derive(Debug, PartialEq, Clone, Hash, Eq, PartialOrd, Ord)]
#[repr(u8)]
enum FacilityItem {
    Generator(String) = 0,
    Microchip(String) = 1,
}

impl FacilityItem {
    fn inner(&self) -> &str {
        use FacilityItem::{Generator, Microchip};
        match self {
            Generator(s) | Microchip(s) => s,
        }
    }

    fn set_inner(&mut self, inner: String) {
        use FacilityItem::{Generator, Microchip};
        match self {
            Generator(_) => *self = Generator(inner),
            Microchip(_) => *self = Microchip(inner),
        }
    }

    fn is_generator(&self) -> bool {
        use std::mem::discriminant;
        discriminant(self) == discriminant(&FacilityItem::Generator(String::new()))
    }

    fn safe_in_floor(&self, floor: &[Self]) -> bool {
        use FacilityItem::{Generator, Microchip};
        match *self {
            Microchip(ref material) => {
                floor.iter().all(|other| !other.is_generator())
                    || floor
                        .iter()
                        .any(|other| other.is_generator() && other.inner() == material)
            }
            Generator(_) => {
                let mut floor = floor.to_owned();
                floor.push(self.clone());
                safe_floor(&floor[..])
            }
        }
    }
}

fn safe_floor(floor: &[FacilityItem]) -> bool {
    let (geners, micros): (Vec<_>, Vec<_>) =
        floor.iter().cloned().partition(|item| item.is_generator());
    micros
        .into_iter()
        .all(|chip| chip.safe_in_floor(&geners[..]))
}

#[derive(Debug)]
struct FloorParseError;

fn parse_floor(s: &str) -> Result<Vec<FacilityItem>, FloorParseError> {
    use FacilityItem::{Generator, Microchip};
    use FloorParseError as FPE;
    let mut parsed_floor: Vec<FacilityItem> = Vec::new();
    let words: Vec<&str> = s
        .split_whitespace()
        .skip(4) // skip past "The {nth} floor contains" text
        .filter(|&word| word != "a" && word != "and") // remove irrelevant words
        .collect();
    if words.len() % 2 != 0 {
        return Err(FPE);
    }
    if words == vec!["nothing", "relevant."] {
        return Ok(parsed_floor);
    }
    for chunk in words.chunks_exact(2) {
        match chunk[1].as_bytes()[0] {
            // only look at the first byte of the string's underlying bytes, to avoid having to
            // worry about varying punctuation at the end
            b'g' => parsed_floor.push(Generator(String::from(chunk[0]))),
            b'm' => parsed_floor.push(Microchip(String::from(
                chunk[0].strip_suffix("-compatible").ok_or(FPE)?,
            ))),
            _ => return Err(FPE),
        };
    }
    Ok(parsed_floor)
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct FacilityState {
    floor_num: usize,
    elevator: Vec<FacilityItem>,
    floors: [Vec<FacilityItem>; 4],
}

impl FacilityState {
    fn current_floor_mut(&mut self) -> &mut Vec<FacilityItem> {
        &mut self.floors[self.floor_num]
    }

    fn current_floor(&self) -> &[FacilityItem] {
        &self.floors[self.floor_num]
    }

    fn abstracted_clone(&self) -> Self {
        use collections::HashMap;
        let mut generated_ids: HashMap<String, String> = HashMap::new();
        let mut id_num = 0usize;
        let mut id_from = |s: String| -> String {
            if let Some(id) = generated_ids.get(&s) {
                id.clone()
            } else {
                let new_id = id_num.to_string();
                id_num += 1;
                generated_ids.insert(s, new_id.clone());
                new_id
            }
        };

        let mut abstractify = |room: &mut Vec<FacilityItem>| {
            room.iter_mut().for_each(|item| {
                item.set_inner(id_from(item.inner().to_string()));
            });
            room.sort();
        };

        let mut elevator: Vec<_> = self.elevator.clone();
        abstractify(&mut elevator);
        let mut floors = self.floors.clone();
        floors.iter_mut().for_each(abstractify);
        FacilityState {
            floor_num: self.floor_num,
            elevator,
            floors,
        }
    }

    fn is_safe(&self) -> bool {
        let mut elevator_floor = self.current_floor().to_owned();
        elevator_floor.extend_from_slice(&self.elevator);
        self.floors.iter().all(|floor| safe_floor(floor.as_slice()))
            && safe_floor(elevator_floor.as_slice())
    }
}

fn min_steps(floors: [Vec<FacilityItem>; 4]) -> usize {
    // uses a BFS of possible steps to find the shortest number of steps that can get everything to
    // the top floor.
    use collections::HashSet;
    let mut seen_states: HashSet<FacilityState> = HashSet::new();
    let mut queue = VecDeque::from([(
        FacilityState {
            floor_num: 0,
            elevator: Vec::with_capacity(2),
            floors,
        },
        0usize,
    )]);
    seen_states.insert(queue[0].0.abstracted_clone());
    while let Some((state, steps)) = queue.pop_front() {
        // eprintln!(
        //     "{state:?}, {steps}S, {}Q, {}C",
        //     queue.len(),
        //     seen_states.len(),
        // );
        macro_rules! queue_push {
            ($val: expr, $step: literal) => {
                let new_state = $val;

                if (0..3).all(|f| state.floors[f].is_empty()) && state.elevator.is_empty() {
                    return steps;
                }
                // eprint!(" - new_state: {new_state:?}");
                if new_state.is_safe() && seen_states.insert(new_state.abstracted_clone()) {
                    queue.push_back((new_state, steps + $step));
                    // eprintln!();
                } else {
                    // eprintln!(" \x1b[3;33m(repeat)\x1b[m");
                }
            };
            ($val: expr) => {
                queue_push!($val, 0)
            };
        }
        macro_rules! add_elevator_moves {
            () => {
                // eprintln!(" - elevator is not empty");
                if state.floor_num < 3 {
                    for item in &state.elevator {
                        if item.is_generator()
                            || item.safe_in_floor(&state.floors[state.floor_num + 1][..])
                        {
                            let mut next_state = state.clone();
                            next_state.floor_num += 1;
                            queue_push!(next_state, 1);
                        }
                    }
                }
                if state.floor_num > 0 {
                    for item in &state.elevator {
                        if item.is_generator()
                            || item.safe_in_floor(&state.floors[state.floor_num - 1][..])
                        {
                            let mut next_state = state.clone();
                            next_state.floor_num -= 1;
                            queue_push!(next_state, 1);
                        }
                    }
                }
            };
        }
        match state.elevator.len() {
            0 => {
                // eprintln!(" - no items to remove from elevator.");
                for i in 0..(state.current_floor().len()) {
                    let mut new_state = FacilityState {
                        elevator: vec![state.current_floor()[i].clone()],
                        ..state.clone()
                    };
                    let in_elevator = new_state.current_floor_mut().remove(i);
                    queue_push!(new_state.clone());
                    for ii in 0..new_state.current_floor().len() {
                        let mut alt_new_state = FacilityState {
                            elevator: vec![
                                in_elevator.clone(),
                                new_state.current_floor()[ii].clone(),
                            ],
                            ..new_state.clone()
                        };
                        alt_new_state.current_floor_mut().remove(ii);
                        queue_push!(alt_new_state);
                    }
                }
            }
            1 => {
                let item = &state.elevator[0];
                add_elevator_moves!();
                // eprintln!(" - can remove {item:?} from elevator");
                let mut new_state = FacilityState {
                    elevator: Vec::with_capacity(2),
                    ..state.clone()
                };
                new_state.current_floor_mut().push(item.clone());
                queue_push!(new_state);
                for i in 0..(state.current_floor().len()) {
                    let mut new_state = FacilityState {
                        elevator: vec![item.clone(), state.current_floor()[i].clone()],
                        ..state.clone()
                    };
                    new_state.current_floor_mut().remove(i);
                    queue_push!(new_state);
                }
            }
            2 => {
                add_elevator_moves!();
                let item_a = state.elevator[0].clone();
                let item_b = state.elevator[1].clone();
                // eprintln!(" - can remove {item_a:?} and {item_b:?} from elevator");
                // remove only item a from elevator
                let mut new_state = FacilityState {
                    elevator: vec![item_b.clone()],
                    ..state.clone()
                };
                new_state.current_floor_mut().push(item_a.clone());
                queue_push!(new_state.clone());

                // remove both items from elevator
                new_state.elevator.clear();
                new_state.current_floor_mut().push(item_b.clone());
                queue_push!(new_state.clone());

                // remove only item a from elevator
                let mut new_state = FacilityState {
                    elevator: vec![item_a],
                    ..state.clone()
                };
                new_state.current_floor_mut().push(item_b);
                queue_push!(new_state);
            }
            i => unreachable!("elevator has {} items, maximum is 2", i),
        }
    }
    unreachable!();
}

fn main() {
    use std::convert::TryInto;
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut floors: [Vec<FacilityItem>; 4] = input
        .lines()
        .map(|line| parse_floor(line).expect("Failed to parse floor from line"))
        .collect::<Vec<_>>()
        .try_into()
        .expect("Wrong number of floors");
    floors[0].push(FacilityItem::Generator(String::from("elerium")));
    floors[0].push(FacilityItem::Microchip(String::from("elerium")));
    floors[0].push(FacilityItem::Generator(String::from("dilithium")));
    floors[0].push(FacilityItem::Microchip(String::from("dilithium")));

    println!("{}", min_steps(floors));
}
