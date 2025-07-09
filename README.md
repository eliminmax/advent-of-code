<!--
SPDX-FileCopyrightText: 2024 Eli Array Minkoff

SPDX-License-Identifier: 0BSD
-->

# My Advent of Code solutions

These are my solutions to Advent of Code - mostly written in 2024 through 2025, but not just for 2024 - while working on Advent of Code 2024, I went through previous years as well - whenever I was caught up on 2024's solutions, I'd go from 2015 day N to 2016 day N through to 2023 day N, then 2015 day N + 1. Upon completing 2024's Advent of code, I decided to iterate through the years one by one rather than iterating through the days. As of June 23th, 2025, I've completed 2015 through 2018.

I started Advent of Code in 2023 in Rust, but only got 2 days in before I was caught up with other things, and I lost the solutions from those 2 days. I redid those days in 2024, and all code in this repo was written during or after December 2024.

At the start of my efforts, I primarily used Python for my solutions, or occasionally AWK or Rust, but as I've worked my way through, I've found myself using Rust more and more, and occasionally using C as well. The one time that the problem involved filtering JSON data, I used jq, and when AWK seems to me to be the best tool for the job, I use it, but that's become rarer as I've gotten into more challenging tasks.

Almost every solution is in a single language, in a standalone source file, with dependencies only on its language's standard library and occasionally other source files in the same directory.

Most of the solutions are in Rust, though are build without cargo.
Due to issues figuring out how to get rustaceanvim and rust-analyzer not to freak out with standalone files, I develop Rust solutions out-of-tree in a cargo-managed crate called `aoc-rs`, with `main.rs` copied from the Rust source template in `./tooling/templates/`, then copy it back in tree once it works. Once in tree, I compile it with `./tooling/templates/aoc-rustc`, to pass a conditional compilation flag in case anything needs to be build or linked differently.
