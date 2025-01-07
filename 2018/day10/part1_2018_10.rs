// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2018 Day 10 Part 1

use light_point::LightPoint; // internal module with parsing logic and the like

trait LightSystem {
    fn current_spread(&self) -> u64;
    fn light_display(&self) -> String;
    fn update_lights(&mut self);
}

impl LightSystem for Vec<LightPoint> {
    fn current_spread(&self) -> u64 {
        if self.is_empty() {
            return 0;
        }
        let mut x_min: i32 = i32::MAX;
        let mut x_max: i32 = i32::MIN;
        let mut y_min: i32 = i32::MAX;
        let mut y_max: i32 = i32::MIN;
        for loc in self.iter().map(|lp| lp.location) {
            x_min = x_min.min(loc.0);
            x_max = x_max.max(loc.0);
            y_min = y_min.min(loc.1);
            y_max = y_max.max(loc.1);
        }
        u64::from(x_min.abs_diff(x_max)) * u64::from(y_min.abs_diff(y_max))
    }

    fn light_display(&self) -> String {
        use std::collections::HashSet;
        if self.is_empty() {
            return String::from("\n");
        }
        let mut x_min: i32 = i32::MAX;
        let mut x_max: i32 = i32::MIN;
        let mut y_min: i32 = i32::MAX;
        let mut y_max: i32 = i32::MIN;
        for loc in self.iter().map(|lp| lp.location) {
            x_min = x_min.min(loc.0);
            x_max = x_max.max(loc.0);
            y_min = y_min.min(loc.1);
            y_max = y_max.max(loc.1);
        }
        let points: HashSet<(u32, u32)> = self
            .iter()
            .map(|lp| {
                (
                    (lp.location.0 - x_min) as u32,
                    (lp.location.1 - y_min) as u32,
                )
            })
            .collect();
        let cols = x_min.abs_diff(x_max);
        let rows = y_min.abs_diff(y_max);

        let mut repr_string = String::new();

        for y in 0..=rows {
            for x in 0..=cols {
                repr_string.push(if points.contains(&(x, y)) { '█' } else { ' ' });
            }
            repr_string.push('\n');
        }
        repr_string
    }

    fn update_lights(&mut self) {
        for lp in self.iter_mut() {
            lp.location.0 += lp.velocity.0;
            lp.location.1 += lp.velocity.1;
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut light_system: Vec<LightPoint> = input
        .lines()
        .map(|line| line.parse().expect("Parsing line into LightPoint"))
        .collect();
    let mut prev_spread = light_system.current_spread();
    loop {
        let prev_system = light_system.clone();
        light_system.update_lights();
        let current_spread = light_system.current_spread();
        if prev_spread < current_spread {
            // use print instead of println because light_display output ends with a newline already
            print!("{}", prev_system.light_display());
            std::process::exit(0);
        }
        prev_spread = current_spread;
    }
}

mod light_point {

    #[derive(Debug, PartialEq, Clone, Copy)]
    pub struct LightPoint {
        pub location: (i32, i32),
        pub velocity: (i32, i32),
    }
    use std::num::ParseIntError;
    use std::str::FromStr;

    #[derive(Debug)]
    pub enum PointParseError {
        UnknownStructure(#[allow(unused)] Box<str>),
        IntParseFail(#[allow(unused)] ParseIntError),
    }

    impl From<&str> for PointParseError {
        fn from(s: &str) -> Self {
            PointParseError::UnknownStructure(s.into())
        }
    }

    impl From<ParseIntError> for PointParseError {
        fn from(e: ParseIntError) -> Self {
            PointParseError::IntParseFail(e)
        }
    }

    impl FromStr for LightPoint {
        type Err = PointParseError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (loc, vel) = s
                .strip_prefix("position=<")
                .and_then(|t| t.strip_suffix('>'))
                .and_then(|t| t.split_once("> velocity=<"))
                .map(|(p, v)| (p.trim(), v.trim()))
                .ok_or(s)?;
            let loc = loc
                .split_once(',')
                .map(|(v, h)| (v.trim(), h.trim()))
                .ok_or(s)?;
            let vel = vel
                .split_once(',')
                .map(|(v, h)| (v.trim(), h.trim()))
                .ok_or(s)?;
            Ok(LightPoint {
                location: (loc.0.parse()?, loc.1.parse()?),
                velocity: (vel.0.parse()?, vel.1.parse()?),
            })
        }
    }
}
