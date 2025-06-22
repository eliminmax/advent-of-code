// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 23 Part 2

// This one was quite challenging, and I found that in order to solve it, I had see the way other
// people approached it conceptually, mainly in a Reddit discussion about handling adversarial
// input - not my goal, but still helped me think through an approach.
//
// https://www.reddit.com/r/adventofcode/comments/a9co1u/

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    const fn distance(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

const ORIGIN: Position = Position { x: 0, y: 0, z: 0 };

#[derive(PartialEq, Clone, Copy)]
struct Nanobot {
    pos: Position,
    radius: i32,
}

#[derive(PartialEq, Clone, Copy)]
struct Region {
    start: Position,
    size: i32,
}

impl Region {
    fn reachable_by(&self, bot: &Nanobot) -> bool {
        debug_assert!(self.start.x < self.start.x + self.size, "{self:?}");

        // clamp bot's location on each axis to a point within the cube, then see if the resulting
        // location is in range of the bot
        let x = bot.pos.x.clamp(self.start.x, self.start.x + self.size - 1);
        let y = bot.pos.y.clamp(self.start.y, self.start.y + self.size - 1);
        let z = bot.pos.z.clamp(self.start.z, self.start.z + self.size - 1);

        bot.pos.distance(Position { x, y, z }) <= bot.radius
    }

    /// Split a taxicab sphere into 6 smaller taxicab spheres at its midpoint
    fn split(self) -> [Region; 8] {
        let Region { start, mut size } = self;
        size /= 2;

        macro_rules! region {
            ($axis: ident) => {{
                Region {
                    start: Position {
                        $axis: start.$axis + size,
                        ..start
                    },
                    size,
                }
            }};
            ($axis0: ident, $axis1: ident) => {{
                Region {
                    start: Position {
                        $axis0: start.$axis0 + size,
                        $axis1: start.$axis1 + size,
                        ..start
                    },
                    size,
                }
            }};
        }

        [
            Self { start, size },
            region!(x),
            region!(y),
            region!(z),
            region!(x, y),
            region!(x, z),
            region!(y, z),
            Region {
                start: Position {
                    x: start.x + size,
                    y: start.y + size,
                    z: start.z + size,
                },
                size,
            },
        ]
    }
}

fn solve(bots: &[Nanobot]) -> i32 {
    let (mut x_min, mut x_max, mut y_min, mut y_max, mut z_min, mut z_max) = (0, 0, 0, 0, 0, 0);
    for bot in bots {
        x_min = i32::min(x_min, bot.pos.x);
        y_min = i32::min(y_min, bot.pos.y);
        z_min = i32::min(z_min, bot.pos.z);

        x_max = i32::max(x_max, bot.pos.x);
        y_max = i32::max(y_max, bot.pos.y);
        z_max = i32::max(z_max, bot.pos.z);
    }

    let size: i32 = x_min
        .abs_diff(x_max)
        .max(y_min.abs_diff(y_max))
        .max(z_min.abs_diff(z_max))
        .next_power_of_two()
        .try_into()
        .unwrap();
    let size = size + 1;

    let start_region = Region {
        start: Position {
            x: x_min,
            y: y_min,
            z: z_min,
        },
        size,
    };
    debug_assert_eq!(
        bots.iter().filter(|b| start_region.reachable_by(b)).count(),
        bots.len()
    );

    let mut queue: Vec<(usize, Region)> = vec![(bots.len(), start_region)];

    loop {
        use std::cmp::Ordering;
        // reorder queue with the following as the priority factors:
        //
        // * first, higher number of bots that can reach anywhere within the region
        // * second, lower distance to origin
        // * last, smaller region size
        //
        // sorting by those factors in that order, the first time that a region of size 1 is popped
        // from the queue, it is guaranteed to be the closest position with the maximum possible
        // reachable bots
        queue.sort_by(|a, b| {
            let mut result = a.0.cmp(&b.0);

            if result == Ordering::Equal {
                result = b.1.start.distance(ORIGIN).cmp(&a.1.start.distance(ORIGIN));
            }

            if result == Ordering::Equal {
                result = b.1.size.cmp(&a.1.size);
            }

            result
        });
        #[cfg(debug_assertions)]
        {
            eprintln!();
            for entry in queue.iter() {
                eprintln!("{entry:?}");
            }
        }

        // size of the queue will keep growing
        let (_, region) = queue.pop().unwrap();
        if region.size == 1 {
            return region.start.distance(ORIGIN);
        }
        queue.extend(
            region
                .split()
                .into_iter()
                .map(|r| (bots.iter().filter(|b| r.reachable_by(b)).count(), r)),
        );
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let bots: Vec<Nanobot> = input.lines().map(|l| l.parse().unwrap()).collect();
    println!("{}", solve(&bots));
}

impl std::str::FromStr for Nanobot {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_str, r_str) = s.trim().split_once(", ").ok_or(s)?;

        let [x, y, z]: [i32; 3] = pos_str
            .strip_prefix("pos=<")
            .and_then(|p| p.strip_suffix('>'))
            .ok_or(pos_str)?
            .split(',')
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("{e:?} ({pos_str})"))?
            .try_into()
            .map_err(|e| format!("{e:?} ({pos_str})"))?;

        let sig_radius: i32 = r_str
            .strip_prefix("r=")
            .ok_or(s)?
            .parse()
            .map_err(|e| format!("{e:?} ({r_str})"))?;

        Ok(Self {
            pos: Position { x, y, z },
            radius: sig_radius,
        })
    }
}

impl std::fmt::Debug for Region {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "start=<{},{},{}>, size={}",
            self.start.x, self.start.y, self.start.z, self.size
        )
    }
}

impl std::fmt::Debug for Nanobot {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            fmt,
            "pos=<{},{},{}>, r={}",
            self.pos.x, self.pos.y, self.pos.z, self.radius
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_bot_overlap() {
        /// macro to construct bots using the same syntax as the input.
        macro_rules! bot {
            (pos=<$x:literal,$y:literal,$z:literal>, r=$r:literal) => {{
                &Nanobot {
                    pos: Position {
                        x: $x,
                        y: $y,
                        z: $z,
                    },
                    radius: $r,
                }
            }};
        }
        let region = Region {
            start: ORIGIN,
            size: 28,
        };

        // bot is just out of cube itself, but can reach into cube
        assert!(region.reachable_by(bot!(pos=<28,28,28>, r=3)));
        assert!(region.reachable_by(bot!(pos=<27,27,28>, r=1)));
        // bot at negative coordinates able to reach
        // space needed to avoid parsing `<-10` as `<- 10` instead of `< -10`
        assert!(region.reachable_by(bot!(pos=< -10,-10,-10>, r=30)));

        // bot is in cube
        assert!(region.reachable_by(bot!(pos=<10,11,10>, r=1)));
        assert!(region.reachable_by(bot!(pos=<9,11,11>, r=1)));
        // bot is outside of cube and can't reach
        assert!(!region.reachable_by(bot!(pos=<28,27,28>, r=1)));
        assert!(!region.reachable_by(bot!(pos=<28,29,28>, r=2)));
    }

    #[test]
    fn sample_input() {
        const TEST_BOTS: &str = concat!(
            "pos=<10,12,12>, r=2\n",
            "pos=<12,14,12>, r=2\n",
            "pos=<16,12,12>, r=4\n",
            "pos=<14,14,14>, r=6\n",
            "pos=<50,50,50>, r=200\n",
            "pos=<10,10,10>, r=5\n"
        );
        let bots: Vec<Nanobot> = TEST_BOTS
            .lines()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();

        let expected_solve = Region {
            start: Position {
                x: 12,
                y: 12,
                z: 12,
            },
            size: 1,
        };
        assert_eq!(
            bots.iter()
                .filter(|bot| expected_solve.reachable_by(bot))
                .count(),
            5
        );
        assert_eq!(solve(&bots), 36);
    }
}
