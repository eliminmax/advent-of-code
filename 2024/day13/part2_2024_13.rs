// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 13 Part 2

use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug)]
struct Button {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct Location {
    x: u64,
    y: u64,
}

#[derive(Debug)]
struct ArcadeMachine {
    a: Button,
    b: Button,
    prize: Location,
}

#[derive(Debug, PartialEq)]
struct ParseArcadeError;

impl FromStr for ArcadeMachine {
    type Err = ParseArcadeError;

    fn from_str(s: &str) -> Result<ArcadeMachine, ParseArcadeError> {
        macro_rules! extract_u64 {
            ($var: ident, $prefix: literal) => {{
                if let Some(i) = $var.strip_prefix($prefix) {
                    u64::from_str(i).map_err(|_| ParseArcadeError)
                } else {
                    Err(ParseArcadeError)
                }
            }};
            ($var: ident, $prefix: literal, $suffix: literal) => {{
                if let Some(i) = $var.strip_suffix($suffix) {
                    extract_u64!(i, $prefix)
                } else {
                    Err(ParseArcadeError)
                }
            }};
        }
        let words: Vec<&str> = s.split_ascii_whitespace().collect();
        match words.as_slice() {
            [
                "Button",
                "A:",
                ax,
                ay,
                "Button",
                "B:",
                bx,
                by,
                "Prize:",
                px,
                py,
            ] => {
                let ax = extract_u64!(ax, "X+", ",")?;
                let ay = extract_u64!(ay, "Y+")?;
                let bx = extract_u64!(bx, "X+", ",")?;
                let by = extract_u64!(by, "Y+")?;
                let px = 10000000000000 + extract_u64!(px, "X=", ",")?;
                let py = 10000000000000 + extract_u64!(py, "Y=")?;
                Ok(ArcadeMachine {
                    a: Button { x: ax, y: ay },
                    b: Button { x: bx, y: by },
                    prize: Location { x: px, y: py },
                })
            }
            _ => Err(ParseArcadeError),
        }
    }
}

impl ArcadeMachine {
    /// Made with some reference to /u/ThunderChaser's explanation of the math on the subreddit:
    /// https://www.reddit.com/r/adventofcode/comments/1hd7irq/
    ///
    /// As soon as I saw what looked like it could've been a Rust implementation, I scrolled up
    /// and copied all of the text above that into a text file and closed the page, to ensure
    /// that my implementation was not derivative, even if I did not work out the math myself
    /// originally.
    ///
    /// I then decided that rather than use the formulas from that, I'd try to "rediscover" them
    /// myself using symbolic computation tools, ultimately using the following Python code
    /// ```python
    /// from sympy import symbols, solve, Eq
    ///
    /// A, B, ax, ay, bx, by, X, Y = symbols("A B ax ay bx by X Y")
    ///
    /// equations = (
    ///     Eq(X, A * ax + B * Bx),
    ///     Eq(Y, A * ay + B * By),
    /// )
    ///
    /// print(solve(equations, (A, B)))
    /// ```
    ///
    /// That resolved to the following formulas, which this implements:
    /// {A: (X*by - Y*bx)/(ax*by - ay*bx), B: (-X*ay + Y*ax)/(ax*by - ay*bx)}
    /// A = (X*by - Y*bx) / (ax*by - ay*bx)
    /// B = (-X*ay + Y*ax) / (ax*by - ay*bx)
    fn token_count(&self) -> u64 {
        let px = self.prize.x as f64;
        let py = self.prize.y as f64;
        let ax = self.a.x as f64;
        let ay = self.a.y as f64;
        let bx = self.b.x as f64;
        let by = self.b.y as f64;
        let common_denominator = ax * by - ay * bx;
        let a = ((px * by) - (py * bx)) / common_denominator;
        let b = ((-px * ay) + (py * ax)) / common_denominator;
        if a.fract() == 0.0 && b.fract() == 0.0 {
            ((a as u64) * 3) + (b as u64)
        } else {
            0
        }
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    println!(
        "{}",
        input
            .split("\n\n")
            .map(|m| ArcadeMachine::from_str(m)
                .expect("Passed an invalid arcade machine definition")
                .token_count())
            .sum::<u64>()
    );
}
