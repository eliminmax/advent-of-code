// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 20 Part 2

use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
struct TeleportID(u16);

impl TeleportID {
    const AA: Self = Self::new(b'A', b'A');
    const ZZ: Self = Self::new(b'Z', b'Z');
    const fn new(a: u8, b: u8) -> Self {
        Self(u16::from_be_bytes([a, b]))
    }
}

#[derive(PartialEq, Clone, Copy)]
enum GridSpace {
    Normal,
    Exit,
    InnerTele(TeleportID),
    OuterTele(TeleportID),
}

type Location = (usize, usize);

struct DonutMaze {
    nodes: HashMap<Location, GridSpace>,
    outer: HashMap<TeleportID, Location>,
    inner: HashMap<TeleportID, Location>,
    start: Location,
    end: Location,
}

impl DonutMaze {
    /// parse `DonutMaze` from `input`, panicking if any issue comes up
    fn panicky_parse(input: &str) -> Self {
        let mut in_grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_owned()).collect();

        let mut nodes: HashMap<Location, GridSpace> = HashMap::new();
        let mut outer: HashMap<TeleportID, Location> = HashMap::new();
        let mut inner: HashMap<TeleportID, Location> = HashMap::new();

        let mut start = None;
        let mut end = None;

        for y in 0..(in_grid.len() - 1) {
            for x in 0..(in_grid[y].len() - 1) {
                match in_grid[y][x] {
                    b' ' | b'#' => (),
                    b'\n' => unreachable!("split on newlines already"),
                    b'.' => {
                        nodes.entry((x, y)).or_insert(GridSpace::Normal);
                    }
                    b'A'..=b'Z' => {
                        let (portal_loc, letter_0, letter_1, is_outer) = {
                            if x > 0 && in_grid[y][x - 1] == b'.' {
                                ((x - 1, y), (x, y), (x + 1, y), x == in_grid[y].len() - 2)
                            } else if y > 0 && in_grid[y - 1][x] == b'.' {
                                ((x, y - 1), (x, y), (x, y + 1), y == in_grid.len() - 2)
                            } else if in_grid[y][x + 2] == b'.' {
                                ((x + 2, y), (x, y), (x + 1, y), x == 0)
                            } else if in_grid[y + 2][x] == b'.' {
                                ((x, y + 2), (x, y), (x, y + 1), y == 0)
                            } else {
                                panic!("unreachable teleport (letter at (x, y) = ({x}, {y}))")
                            }
                        };
                        assert!(
                            in_grid[letter_0.1][letter_0.0].is_ascii_uppercase()
                                && in_grid[letter_1.1][letter_1.0].is_ascii_uppercase(),
                            "Incomplete teleport node (at (x, y) = {portal_loc:?})"
                        );
                        let id = TeleportID::new(
                            in_grid[letter_0.1][letter_0.0],
                            in_grid[letter_1.1][letter_1.0],
                        );
                        match id {
                            TeleportID::AA => {
                                assert!(is_outer, "start is on the inside");
                                nodes.insert(portal_loc, GridSpace::Exit);
                                assert!(start.is_none(), "multiple starts");
                                start = Some(portal_loc);
                            }
                            TeleportID::ZZ => {
                                assert!(is_outer, "end is on the inside");
                                nodes.insert(portal_loc, GridSpace::Exit);
                                assert!(end.is_none(), "multiple ends");
                                end = Some(portal_loc);
                            }
                            _ => {
                                if is_outer {
                                    nodes.insert(portal_loc, GridSpace::OuterTele(id));
                                    let prev_linked = outer.insert(id, portal_loc);
                                    assert!(
                                        prev_linked.is_none(),
                                        "More than 1 outer node with ID {id:?}"
                                    );
                                } else {
                                    nodes.insert(portal_loc, GridSpace::InnerTele(id));
                                    let prev_linked = inner.insert(id, portal_loc);
                                    assert!(
                                        prev_linked.is_none(),
                                        "More than 1 inner node with ID {id:?}"
                                    );
                                }
                            }
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
        let start = start.expect("Missing start");
        let end = end.expect("Missing end");
        outer.retain(|k, _| inner.contains_key(k));
        inner.retain(|k, _| outer.contains_key(k));

        Self {
            nodes,
            outer,
            inner,
            start,
            end,
        }
    }

    /// return an iterator over reachable neighbors of `loc` within the maze, paired with the
    /// nest level at each neighbor
    fn links_from(&self, loc: Location, nest_lvl: u16) -> impl Iterator<Item = (Location, u16)> {
        let mut neighbor_list = Vec::with_capacity(5);

        neighbor_list.push(((loc.0 - 1, loc.1), nest_lvl));
        neighbor_list.push(((loc.0, loc.1 - 1), nest_lvl));
        neighbor_list.push(((loc.0 + 1, loc.1), nest_lvl));

        neighbor_list.push(((loc.0, loc.1 + 1), nest_lvl));
        match self.nodes[&loc] {
            GridSpace::Normal | GridSpace::Exit => (),
            GridSpace::OuterTele(id) => {
                if nest_lvl > 0 {
                    neighbor_list.push((self.inner[&id], nest_lvl - 1));
                }
            }
            GridSpace::InnerTele(id) => {
                neighbor_list.push((self.outer[&id], nest_lvl + 1));
            }
        }
        neighbor_list.retain(|(neighbor, lvl)| {
            self.nodes.get(neighbor).is_some_and(|n| {
                if *lvl == 0 {
                    !matches!(n, GridSpace::OuterTele(_))
                } else {
                    *n != GridSpace::Exit
                }
            })
        });

        neighbor_list.into_iter()
    }

    /// find the length of the shortest route from `TeleportID(b"AA")` to `TeleportID(b"ZZ")`,
    /// using Dijkstra's algorithm, modified to skip any path that goes longer than the shortest
    /// known path to the end node, so that it doesn't continue to descend forever.
    fn aa_to_zz_dist(&self) -> u32 {
        use std::cmp::Reverse;

        debug_assert_ne!(self.start, self.end, "Valid input won't have start == end");
        debug_assert_eq!(
            self.nodes[&self.start],
            GridSpace::Exit,
            "start is not an exit"
        );
        debug_assert_eq!(self.nodes[&self.end], GridSpace::Exit, "end is not an exit");
        debug_assert_eq!(
            self.nodes
                .values()
                .filter(|&v| *v == GridSpace::Exit)
                .count(),
            2,
            "more than 2 exits"
        );

        let mut distances: HashMap<(u16, Location), u32> =
            HashMap::with_capacity(self.nodes.len() * 8);
        // Prioritize lower depths to avoid running infinitely. (x, y) position is then used as a
        // tie breaker
        let mut queue: BinaryHeap<Reverse<(u32, (u16, Location))>> =
            BinaryHeap::with_capacity(self.nodes.len());
        let mut min_distance = None;
        queue.push(Reverse((0, (0, self.start))));
        while let Some(Reverse((dist, (nest_lvl, loc)))) = queue.pop() {
            if distances
                .get(&(nest_lvl, loc))
                .is_some_and(|prev| *prev < dist)
            {
                continue;
            } else if let Some(md) = min_distance
                && md <= dist
            {
                continue;
            }
            let new_dist = dist + 1;
            for (neighbor, next_nest) in self.links_from(loc, nest_lvl) {
                if neighbor == self.end {
                    assert_eq!(next_nest, 0);
                    debug_assert!(min_distance.is_none_or(|md| md > new_dist));
                    min_distance = Some(new_dist);
                } else if distances
                    .get(&(next_nest, neighbor))
                    .is_none_or(|prev| *prev > new_dist)
                {
                    queue.push(Reverse((new_dist, (next_nest, neighbor))));
                    distances.insert((next_nest, neighbor), new_dist);
                }
            }
        }

        min_distance.expect("Couldn't find path to exit")
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
        write!(fmt, "{}", self.0.to_be_bytes().escape_ascii())
    }
}

impl std::fmt::Debug for GridSpace {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            GridSpace::Exit => write!(fmt, "Exit"),
            GridSpace::Normal => write!(fmt, "Normal"),
            GridSpace::OuterTele(id) => write!(fmt, "OTel({id:?}"),
            GridSpace::InnerTele(id) => write!(fmt, "ITel({id:?}"),
        }
    }
}
