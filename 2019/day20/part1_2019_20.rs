// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 20 Part 1

use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
struct TeleportID(u16);

#[derive(Debug, PartialEq, Clone, Copy)]
enum GridSpace {
    Normal,
    Teleport(TeleportID),
}

type Location = (usize, usize);

struct DonutMaze {
    nodes: HashMap<Location, GridSpace>,
    linked: HashMap<TeleportID, [Location; 2]>,
    unlinked: HashMap<TeleportID, Location>,
}

impl DonutMaze {
    /// parse `DonutMaze` from `input`, panicking if any issue comes up
    fn panicky_parse(input: &str) -> Self {
        let mut in_grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_owned()).collect();

        let mut nodes: HashMap<Location, GridSpace> = HashMap::new();
        let mut linked: HashMap<TeleportID, [Location; 2]> = HashMap::new();
        let mut unlinked: HashMap<TeleportID, Location> = HashMap::new();

        for y in 0..(in_grid.len() - 1) {
            for x in 0..(in_grid[y].len() - 1) {
                match in_grid[y][x] {
                    b' ' | b'#' => (),
                    b'\n' => unreachable!("split on newlines already"),
                    b'.' => {
                        nodes.entry((x, y)).or_insert(GridSpace::Normal);
                    }
                    b'A'..=b'Z' => {
                        let (portal_loc, letter_0, letter_1) = {
                            if x > 0 && in_grid[y][x - 1] == b'.' {
                                ((x - 1, y), (x, y), (x + 1, y))
                            } else if y > 0 && in_grid[y - 1][x] == b'.' {
                                ((x, y - 1), (x, y), (x, y + 1))
                            } else if in_grid[y][x + 2] == b'.' {
                                ((x + 2, y), (x, y), (x + 1, y))
                            } else if in_grid[y + 2][x] == b'.' {
                                ((x, y + 2), (x, y), (x, y + 1))
                            } else {
                                panic!("unreachable teleport (letter at (x, y) = ({x}, {y}))")
                            }
                        };
                        assert!(
                            in_grid[letter_0.1][letter_0.0].is_ascii_uppercase()
                                && in_grid[letter_1.1][letter_1.0].is_ascii_uppercase(),
                            "Incomplete teleport node (at (x, y) = {portal_loc:?})"
                        );
                        let id = TeleportID(u16::from_be_bytes([
                            in_grid[letter_0.1][letter_0.0],
                            in_grid[letter_1.1][letter_1.0],
                        ]));

                        nodes.insert(portal_loc, GridSpace::Teleport(id));

                        if let Some(linked_loc) = unlinked.remove(&id) {
                            let prev_linked = linked.insert(id, [portal_loc, linked_loc]);
                            assert!(prev_linked.is_none(), "More than 2 nodes with ID {id:?}");
                        } else {
                            unlinked.insert(id, portal_loc);
                        }
                        in_grid[letter_0.1][letter_0.0] = b' ';
                        in_grid[letter_1.1][letter_1.0] = b' ';
                    }
                    badbyte => panic!(
                        "invalid input byte 0x{badbyte:02x} (\"{}\")",
                        badbyte.escape_ascii()
                    ),
                }
            }
        }
        Self {
            nodes,
            linked,
            unlinked,
        }
    }

    /// return an iterator over reachable neighbors of `location` within the maze
    fn links_from(&self, loc: Location) -> impl Iterator<Item = Location> {
        let mut neighbor_list = Vec::with_capacity(4);
        if loc.0 > 0 {
            neighbor_list.push((loc.0 - 1, loc.1));
        }
        if loc.1 > 0 {
            neighbor_list.push((loc.0, loc.1 - 1));
        }
        neighbor_list.push((loc.0 + 1, loc.1));
        neighbor_list.push((loc.0, loc.1 + 1));
        neighbor_list.retain(|neighbor| self.nodes.contains_key(neighbor));
        if let Some(GridSpace::Teleport(id)) = self.nodes.get(&loc)
            && let Some(&[a, b]) = self.linked.get(id)
        {
            if a == loc {
                debug_assert_ne!(b, loc, "self-referential portal {id:?}");
                neighbor_list.push(b);
            } else {
                debug_assert_eq!(b, loc, "improperly-linked portal {id:?}");
                neighbor_list.push(a);
            }
        }

        neighbor_list.into_iter()
    }

    /// find the length of the shortest route from `TeleportID(b"AA")` to `TeleportID(b"ZZ")`,
    /// using Dijkstra's algorithm
    fn aa_to_zz_dist(&self) -> u32 {
        use std::cmp::Reverse;

        let start_id = TeleportID::from(b"AA");
        let end_id = TeleportID::from(b"ZZ");
        assert!(self.unlinked.contains_key(&start_id), "Missing start node");
        assert!(self.unlinked.contains_key(&end_id), "Missing target node");

        let mut distances: HashMap<Location, u32> = HashMap::with_capacity(self.nodes.len());
        let mut queue: BinaryHeap<Reverse<(u32, Location)>> =
            BinaryHeap::with_capacity(self.nodes.len());
        queue.push(Reverse((0, self.unlinked[&start_id])));
        while let Some(Reverse((dist, loc))) = queue.pop() {
            if distances.get(&loc).is_some_and(|prev| *prev < dist) {
                continue;
            }

            let new_dist = dist + 1;
            for neighbor in self.links_from(loc) {
                if distances.get(&neighbor).is_none_or(|prev| *prev > new_dist) {
                    distances.insert(neighbor, new_dist);
                    queue.push(Reverse((new_dist, neighbor)));
                }
            }
        }

        distances[&self.unlinked[&end_id]]
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;

    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let maze = DonutMaze::panicky_parse(&input);
    println!("{}", maze.aa_to_zz_dist());
}

impl std::fmt::Debug for TeleportID {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "TeleportID(b\"{}\")",
            self.0.to_be_bytes().escape_ascii()
        )
    }
}

impl From<&[u8; 2]> for TeleportID {
    fn from(id: &[u8; 2]) -> Self {
        Self(u16::from_be_bytes(*id))
    }
}
