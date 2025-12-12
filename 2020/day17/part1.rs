// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 17 Part 1

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ConwayCubeLocation {
    x: i16,
    y: i16,
    z: i16,
}

impl ConwayCubeLocation {
    fn neighbors(&self) -> impl Iterator<Item = Self> {
        let Self { x, y, z } = *self;

        (x - 1..=x + 1)
            .flat_map(move |nx| {
                (y - 1..=y + 1).flat_map(move |ny| {
                    (z - 1..=z + 1).map(move |nz| Self {
                        x: nx,
                        y: ny,
                        z: nz,
                    })
                })
            })
            .filter(move |nself| nself != self)
    }
}

fn update(active: &mut HashSet<ConwayCubeLocation>) {
    let mut next_gen_living = HashSet::new();
    let mut inactive_neighbors = HashSet::new();

    for location in active.iter().copied() {
        let mut count = 0;
        for neighbor in location.neighbors() {
            if active.contains(&neighbor) {
                count += 1;
            } else {
                inactive_neighbors.insert(neighbor);
            }
        }
        if count == 2 || count == 3 {
            next_gen_living.insert(location);
        }
    }

    for location in inactive_neighbors {
        if location
            .neighbors()
            .filter(|ccl| active.contains(ccl))
            .count()
            == 3
        {
            next_gen_living.insert(location);
        }
    }
    active.clear();
    active.extend(next_gen_living);
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut active: HashSet<ConwayCubeLocation> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        let y = i16::try_from(y).unwrap();
        for (x, c) in line.chars().enumerate() {
            let x = i16::try_from(x).unwrap();
            if c == '#' {
                active.insert(ConwayCubeLocation { x, y, z: 0 });
            }
        }
    }

    for _ in 0..6 {
        update(&mut active);
    }

    println!("{}", active.len());
}

#[cfg(test)]
mod tests {
    use super::*;
    fn normalize_xy(generation: &mut HashSet<ConwayCubeLocation>) {
        let x_min = generation.iter().map(|ccl| ccl.x).min().unwrap_or_default();
        let y_min = generation.iter().map(|ccl| ccl.y).min().unwrap_or_default();
        let adjusted: HashSet<ConwayCubeLocation> = generation
            .drain()
            .map(|ConwayCubeLocation { x, y, z }| ConwayCubeLocation {
                x: x - x_min,
                y: y - y_min,
                z,
            })
            .collect();
        generation.extend(adjusted);
    }

    #[test]
    fn example_setup() {
        // macro to reduce visual noize when constructing a pre-defined cube list
        macro_rules! cube_set {
            {$([x=$x: literal, y=$y: literal, z=$z: literal]),+} => {{
                macro_rules! ccl {
                    ($cclx: literal, $ccly: literal, $cclz: literal) => {{
                        ConwayCubeLocation { x: $cclx, y: $ccly, z: $cclz}
                    }};
                }
                HashSet::from([
                    $(ccl!($x, $y, $z)),+
                ])
            }};
        }

        let mut active = cube_set! {
            /* z=0 */
            /* .#. */ [x=1, y=0, z=0],
            /* ..# */ [x=2, y=1, z=0],
            /* ### */ [x=0, y=2, z=0], [x=1, y=2, z=0], [x=2, y=2, z=0]
        };

        update(&mut active);
        normalize_xy(&mut active);

        let expected_gen1 = cube_set! {
            /* z=-1 */
            /* #..  */ [x=0, y=0, z=-1],
            /* ..#  */ [x=2, y=1, z=-1],
            /* .#.  */ [x=1, y=2, z=-1],

            /* z=0 */
            /* #.# */  [x=0, y=0, z=0], [x=2, y=0, z=0],
            /* .## */  [x=1, y=1, z=0], [x=2, y=1, z=0],
            /* .#. */  [x=1, y=2, z=0],

            /* z=1 */
            /* #.. */  [x=0, y=0, z=1],
            /* ..# */  [x=2, y=1, z=1],
            /* .#. */  [x=1, y=2, z=1]

        };
        assert_eq!(active, expected_gen1);

        update(&mut active);
        normalize_xy(&mut active);

        let expected_gen2 = cube_set! {
            /* z=-2  */
            /* ..... */
            /* ..... */
            /* ..#.. */ [x=2, y=2, z=-2],
            /* ..... */
            /* ..... */

            /* z=-1  */
            /* ..#.. */ [x=2, y=0, z=-1],
            /* .#..# */ [x=1, y=1, z=-1], [x=4, y=1, z=-1],
            /* ....# */ [x=4, y=2, z=-1],
            /* .#... */ [x=1, y=3, z=-1],
            /* ..... */

            /* z=0   */
            /* ##... */ [x=0, y=0, z=0], [x=1, y=0, z=0],
            /* ##... */ [x=0, y=1, z=0], [x=1, y=1, z=0],
            /* #.... */ [x=0, y=2, z=0],
            /* ....# */ [x=4, y=3, z=0],
            /* .###. */ [x=1, y=4, z=0], [x=2, y=4, z=0], [x=3, y=4, z=0],

            /* z=1   */
            /* ..#.. */ [x=2, y=0, z=1],
            /* .#..# */ [x=1, y=1, z=1], [x=4, y=1, z=1],
            /* ....# */ [x=4, y=2, z=1],
            /* .#... */ [x=1, y=3, z=1],
            /* ..... */

            /* z=2   */
            /* ..... */
            /* ..... */
            /* ..#.. */ [x=2, y=2, z=2]
            /* ..... */
            /* ..... */
        };
        assert_eq!(active, expected_gen2);

        update(&mut active);
        assert_eq!(active.len(), 38);
        update(&mut active);
        update(&mut active);
        update(&mut active);
        assert_eq!(active.len(), 112);
    }
}
