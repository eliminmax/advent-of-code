#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 8 Part 2

import sys
from collections.abc import Iterable


def countwhile(tree: int, line: Iterable[int]) -> int:
    counter = 0
    for other_tree in line:
        counter += 1
        if other_tree >= tree:
            break
    return counter


def main() -> None:
    max_score = 0

    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        grid: list[list[int]] = [[int(i) for i in line.strip()] for line in f]

    for y, row in enumerate(grid[1:-1], start=1):
        for x, tree in enumerate(row[1:-1], start=1):
            north = countwhile(tree, (r[x] for r in grid[y - 1 :: -1]))
            east = countwhile(tree, row[x + 1 :])
            south = countwhile(tree, (r[x] for r in grid[y + 1 :]))
            west = countwhile(tree, row[x - 1 :: -1])
            max_score = max(max_score, east * west * north * south)
    print(max_score)


if __name__ == "__main__":
    main()
