#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 3 Part 2

import sys
from math import sqrt, ceil
from collections.abc import Generator

from spiral import spiral, Coords


def sum_neighbors(pos: Coords, grid: list[list[int]]) -> int:

    coord_pairs = [
        Coords(x, y)
        for x in range(max(pos.x - 1, 0), min(pos.x + 2, len(grid)))
        for y in range(max(pos.y - 1, 0), min(pos.y + 2, len(grid)))
    ]
    return sum(map(lambda loc: grid[loc.y][loc.x], coord_pairs))


def find_first_highest(n: int) -> int:
    ring_count: int = ceil(sqrt(n))
    size = ring_count * 2 - 1
    # create a zero-initialized size*size grid
    # use `… for _ in range(size)]` rather than `…] * size` as the latter
    # would create multiple references to the same list rather than 1 list
    # per row
    grid = [[0] * size for _ in range(size)]
    coords: Generator[Coords] = spiral(ring_count)
    start = next(coords)
    # set middle value to 1
    grid[start.y][start.x] = 1
    for pos in coords:
        new_val = sum_neighbors(pos, grid)
        if new_val > n:
            return new_val
        else:
            grid[pos.y][pos.x] = new_val


def main():
    with open(sys.argv[1], "r") as f:
        distance = int(f.read().strip())
    print(find_first_highest(distance))


if __name__ == "__main__":
    main()
