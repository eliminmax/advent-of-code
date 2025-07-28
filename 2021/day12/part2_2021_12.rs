// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 12 Part 2

use std::collections::{HashMap, HashSet, VecDeque};
#[derive(PartialEq, Eq, Hash, Clone)]
struct Route {
    caves: Vec<Cave>,
    revisited: bool,
}

impl Default for Route {
    fn default() -> Self {
        Self {
            caves: vec![Cave::Start],
            revisited: false,
        }
    }
}

impl Route {
    fn can_visit(&self, c: &Cave) -> bool {
        match c {
            Cave::Large(..) => true,
            Cave::Small(..) if !self.revisited => true,
            c => !self.caves.contains(c),
        }
    }

    fn current_pos(&self) -> &Cave {
        self.caves.last().unwrap()
    }

    fn with_next_cave(&self, c: Cave) -> Self {
        let mut new_self = self.clone();
        new_self.caves.push(c);
        if matches!(c, Cave::Small(..)) && self.caves.contains(&c) {
            new_self.revisited = true;
        }
        new_self
    }
}

type LinkGraph<T> = HashMap<T, HashSet<T>>;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Cave {
    Start,
    Large([u8; 2]),
    Small([u8; 2]),
    End,
}

impl std::str::FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes() {
            b"start" => Ok(Self::Start),
            b"end" => Ok(Self::End),
            &[a, b] if a.is_ascii_lowercase() && b.is_ascii_lowercase() => Ok(Self::Small([a, b])),
            &[a, b] if a.is_ascii_uppercase() && b.is_ascii_uppercase() => Ok(Self::Large([a, b])),
            _ => Err(s.into()),
        }
    }
}

fn count_routes(links: &LinkGraph<Cave>) -> usize {
    let mut queue = VecDeque::from([Route::default()]);
    let mut routes = HashSet::new();
    while let Some(route) = queue.pop_front() {
        let reachable = links[route.current_pos()]
            .iter()
            .copied()
            .filter(|c| route.can_visit(c));
        for cave in reachable {
            let next_route = route.with_next_cave(cave);
            if cave == Cave::End {
                routes.insert(next_route);
            } else {
                queue.push_back(next_route);
            }
        }
    }
    routes.len()
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");

    let mut links: LinkGraph<Cave> = LinkGraph::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        let a = a.parse().unwrap();
        let b = b.parse().unwrap();

        macro_rules! add_link {
            ($_0: ident, $_1: ident) => {
                if $_0 != Cave::End && $_1 != Cave::Start {
                    links
                        .entry($_0)
                        .and_modify(|m| {
                            m.insert($_1);
                        })
                        .or_insert(HashSet::from([$_1]));
                }
            };
        }

        add_link!(a, b);
        add_link!(b, a);
    }

    assert!(links.contains_key(&Cave::Start), "missing start");

    for (cave, connected) in links.iter() {
        assert!(!connected.contains(&Cave::Start), "can return to start");
        if matches!(cave, Cave::Large(..)) {
            assert!(
                connected.iter().all(|c| !matches!(c, Cave::Large(..))),
                "Infinite loop possible"
            );
        }
    }

    println!("{}", count_routes(&links));
}

mod dbg_fmts {
    use super::{Cave, Route};
    use std::fmt;

    impl fmt::Display for Cave {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Self::Start => write!(fmt, "start"),
                Self::End => write!(fmt, "end"),
                Self::Large(id) | Self::Small(id) => {
                    write!(fmt, "{}", id.as_slice().escape_ascii())
                }
            }
        }
    }
    impl fmt::Debug for Cave {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(fmt, "Cave [{self}]")
        }
    }

    impl fmt::Display for Route {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.caves.is_empty() {
                return Ok(());
            }
            write!(fmt, "{}", self.caves[0])?;

            for cave in &self.caves[1..] {
                write!(fmt, ",{cave}")?;
            }

            Ok(())
        }
    }
    impl fmt::Debug for Route {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(fmt, "Route [{self}]")
        }
    }
}
