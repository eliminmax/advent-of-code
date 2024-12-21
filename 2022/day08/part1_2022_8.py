#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 8 Part 1

import sys


def main() -> None:
    vis = 0
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        grid: list[list[int]] = [[int(i) for i in line.strip()] for line in f]
    vis += len(grid[0])
    vis += len(grid[-1])
    for y, row in enumerate(grid[1:-1], start=1):
        vis += 2
        for x, tree in enumerate(row[1:-1], start=1):
            if tree > max(row[:x]):
                vis += 1
            elif tree > max(row[x + 1 :]):
                vis += 1
            elif tree > max(r[x] for r in grid[:y]):
                vis += 1
            elif y + 1 < len(grid) and tree > max(r[x] for r in grid[y + 1 :]):
                vis += 1
    print(vis)


if __name__ == "__main__":
    main()
