// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 16 Part 1
//
// Went through major revisions after getting stuck on Part 2, and seeing improvements that also
// applied to part 1 while troubleshooting.
//
// My original approach used Dijkstra's algorithm with a `State` struct as the nodes, with the
// `State` holding the current pipe, and a list of activated pipes, and of which minutes they were
// activated on.
// The available actions were to go to an adjacent node, at cost `1`, or to activate the pipe, if
// it hadn't been activated already, and had a flow rate greater than `0`.
//
// The list was behind an `Rc`, so it wasn't cloning the whole thing for each entry, but it was
// still inefficient - consistently taking 5.5 to 7 seconds on my system with my input.
//
// I realized while trying to do part 2 that I could use this approach, and it seems to take around
// 93 to 96 milliseconds.

use std::collections::{HashMap, HashSet, VecDeque};
mod dijkstra;
use dijkstra::dijkstra;

#[derive(PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct PipeId(u16);
impl PipeId {
    const START: PipeId = PipeId::parse("AA");
    const fn parse(s: &str) -> Self {
        assert!(s.len() == 2);
        assert!(s.as_bytes()[0].is_ascii_uppercase());
        assert!(s.as_bytes()[1].is_ascii_uppercase());
        Self(u16::from_ne_bytes([s.as_bytes()[0], s.as_bytes()[1]]))
    }
}

impl std::fmt::Debug for PipeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("PipeId")
            .field(&str::from_utf8(&self.0.to_ne_bytes()).unwrap())
            .finish()
    }
}

#[derive(Debug, PartialEq, Clone)]
struct PipeInfo {
    id: PipeId,
    flow_rate: u32,
    links: Box<[PipeId]>,
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut pipe_info: HashMap<PipeId, PipeInfo> = HashMap::new();
    let mut lowercase_or_space = ('a'..='z').collect::<Vec<_>>();
    lowercase_or_space.push(' ');

    for line in input.lines() {
        // This parsing code is horrible, but it worked flawlessly first try. Somehow.
        // So of cource, I went and made it wven worse. It still worked. I feel gross.
        let (id, flow_rate, links) = line
            .strip_prefix("Valve ")
            .and_then(|s| s.split_once(" has flow rate="))
            .and_then(|(a, b)| {
                b.split_once("; tunnel").map(|(b, c)| {
                    (
                        PipeId::parse(a),
                        b.parse::<u32>().unwrap(),
                        c.trim_start_matches(lowercase_or_space.as_slice())
                            .split(", ")
                            .map(PipeId::parse)
                            .collect::<Vec<_>>()
                            .into_boxed_slice(),
                    )
                })
            })
            .unwrap();

        pipe_info.insert(
            id,
            PipeInfo {
                id,
                flow_rate,
                links,
            },
        );
    }
    let flowing_pipes: HashSet<PipeId> = pipe_info
        .keys()
        .filter(|k| pipe_info[k].flow_rate > 0)
        .copied()
        .collect();
    let link_fn = |p| pipe_info[&p].links.iter().map(|&l| (l, 1));
    let mut pipe_links: HashMap<PipeId, HashMap<PipeId, u32>> = flowing_pipes
        .iter()
        .cloned()
        .map(|p| (p, dijkstra(p, link_fn)))
        .collect();
    pipe_links
        .entry(PipeId::START)
        .or_insert_with(|| dijkstra(PipeId::START, link_fn));
    for (pipe, map) in pipe_links.iter_mut() {
        map.retain(|p, _| flowing_pipes.contains(p) && p != pipe);
    }

    let mut sequences: Vec<(u32, Vec<PipeId>)> = Vec::new();
    let mut queue: VecDeque<(u32, u32, Vec<PipeId>)> =
        VecDeque::from([(30, 0, vec![PipeId::parse("AA")])]);

    while let Some((mins, release, sequence)) = queue.pop_front() {
        let current_pipe = sequence.last().unwrap();

        let new_released = release + (mins) * pipe_info[current_pipe].flow_rate;
        sequences.push((new_released, sequence.clone()));

        'inner: for target in flowing_pipes.iter() {
            if sequence.contains(target) {
                continue 'inner;
            }
            let travel_time = pipe_links[current_pipe][target];
            if let Some(new_mins) = mins.checked_sub(travel_time + 1) {
                let mut new_seq = sequence.clone();
                new_seq.push(*target);
                queue.push_back((new_mins, new_released, new_seq));
            }
        }
    }

    let max_pressure = sequences
        .into_iter()
        .map(|(p, _)| p)
        .max()
        .unwrap_or_default();
    println!("{max_pressure}");
}
