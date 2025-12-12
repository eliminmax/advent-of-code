<!--
SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff

SPDX-License-Identifier: 0BSD
-->

# My Advent of Code solutions

These are my solutions to Advent of Code.

| Year           | Status               |
|----------------|----------------------|
| [2015](./2015) | Completed 2024-12-29 |
| [2016](./2016) | Completed 2025-01-02 |
| [2017](./2017) | Completed 2025-01-02 |
| [2018](./2018) | Completed 2025-06-23 |
| [2019](./2019) | Completed 2025-07-18 |
| [2020](./2020) | Completed 2025-07-27 |
| [2021](./2021) | Completed 2025-12-03 |
| [2022](./2022) | In progress          |
| [2023](./2023) | On hold              |
| [2024](./2024) | Completed 2024-12-25 |
| [2025](./2025) | Completed 2025-12-12 |

My attempt at Advent of Code 2023 was quickly lost in the shuffle of life by day
3, and while cleaning up space on my laptop, I regrettably `rm`ed my first 2
solutions. I have since solved the first 2 days of 2023 again.

When I tried again in 2024, my original, overly-ambitious plan was to do all 10
years' puzzles at once - after 2024's day 1, I did 2015's day 1, then 2016's,
then 2017's, and so on.

I quickly realized that was not realistic, and was okay with taking previous
years at my own pace, but continued to go through all previous years at once
until I saw 2019's day 9 built on a previous day's puzzle that year, and decided
to switch to taking it year by year, mainly because of Intcode.

Almost every solution is in a single language, in a standalone source file, with
dependencies only on its language's standard library and occasionally other
source files in the same directory. The most common exception is
reverse-engineering/decompilation problems that have sometimes come up, for
which I'll use a mix of tools to poke at the problem from different angles,
typically keeping notes in a Markdown file.

At the start of my efforts, I primarily used Python for my solutions, or
occasionally AWK or Rust, but as I worked my way through, I found myself using
Rust more and more, and occasionally using C as well. The one time that the
problem involved filtering JSON data, I used jq, and when AWK seemed to me to be
the best tool for the job, I use it, but that's rare.

Most of the solutions are in Rust, though are built without cargo.

Due to issues figuring out how to get rustaceanvim and rust-analyzer not to
freak out with standalone files, I develop Rust solutions out-of-tree in a
cargo-managed crate called `aoc-rs`, with `main.rs` copied from the Rust source
template in `./tooling/templates/`, then copy it back in tree once it works.
Once in tree, I compile it with `./tooling/templates/aoc-rustc`, to pass a
conditional compilation flag in case anything needs to be build or linked
differently.
