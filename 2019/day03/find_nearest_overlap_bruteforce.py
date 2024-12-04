#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2019 Day 3 Part 1

# By solution, I mean ugly, slow, brute-force solution.
# I should've gone with some linear algebra to make it faster, but computers
# are fast enough that this didn't take as long.

import sys


def dist(a: int, b: int) -> int:
    return abs(a) + abs(b)


def locs(steps: list[str]) -> list[tuple[int, int, int]]:
    x = y = 0
    seen_locs: set[tuple[int, int, int]] = set()
    for step in steps:
        delta = int(step[1:])
        ox = x
        oy = y
        match step[0]:
            case "R":
                x += delta
                new_locs = {
                    (i, y, dist(i, y)) for i in range(ox, x + 1) if y or i
                }
            case "L":
                x -= delta
                new_locs = {
                    (i, y, dist(i, y)) for i in range(ox, x - 1, -1) if y or i
                }
            case "U":
                y += delta
                new_locs = {
                    (x, i, dist(x, i)) for i in range(oy, y + 1) if x or i
                }
            case "D":
                y -= delta
                new_locs = {
                    (x, i, dist(x, i)) for i in range(oy, y - 1, -1) if x or i
                }
        seen_locs = seen_locs.union(new_locs)
    return sorted(seen_locs, key=lambda loc: loc[2])


def main() -> None:
    with open(sys.argv[1], "r") as f:
        w0_path, w1_path = [line.strip().split(",") for line in f]
    w0_locs = locs(w0_path)
    cross_point = next(filter(lambda p: p in w0_locs, locs(w1_path)))
    print(cross_point[2])


if __name__ == "__main__":
    main()
