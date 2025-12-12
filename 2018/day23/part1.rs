// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 23 Part 1

#[derive(Debug, PartialEq, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    fn distance(self, other: Self) -> u32 {
        (self.x - other.x).unsigned_abs()
            + (self.y - other.y).unsigned_abs()
            + (self.z - other.z).unsigned_abs()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Nanobot {
    pos: Position,
    sig_radius: u32,
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

        let sig_radius: u32 = r_str
            .strip_prefix("r=")
            .ok_or(s)?
            .parse()
            .map_err(|e| format!("{e:?} ({r_str})"))?;

        Ok(Self {
            pos: Position { x, y, z },
            sig_radius,
        })
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let bots: Vec<Nanobot> = input.lines().map(|l| l.parse().unwrap()).collect();
    let Nanobot {
        pos: strongest_pos,
        sig_radius: strongest_r,
    } = *bots
        .iter()
        .max_by_key(|Nanobot { pos: _, sig_radius }| sig_radius)
        .unwrap();
    println!(
        "{}",
        bots.into_iter()
            .filter(|bot| strongest_pos.distance(bot.pos) <= strongest_r)
            .count()
    );
}
