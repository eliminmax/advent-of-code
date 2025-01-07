#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 8 Part 2

import sys
from typing import TypeAlias
from collections import defaultdict
from itertools import combinations, count

Location: TypeAlias = tuple[int, int]
Char: TypeAlias = int


def find_antinodes(
    antennas: list[Location], rows: int, cols: int
) -> set[Location]:
    locations: set[Location] = set()
    combos = (tuple(sorted(combo)) for combo in combinations(antennas, 2))
    for combo in combos:
        # already sorted by row - need to know if slope is positive or not
        row_offset = combo[1][0] - combo[0][0]
        col_offset = combo[1][1] - combo[0][1]
        for i in count(0):
            r = combo[0][0] - (row_offset * i)
            c = combo[0][1] - (col_offset * i)
            if r in range(rows) and c in range(cols):
                locations.add((r, c))
            else:
                break
        for i in count(0):
            r = combo[1][0] + (row_offset * i)
            c = combo[1][1] + (col_offset * i)
            if r in range(rows) and c in range(cols):
                locations.add((r, c))
            else:
                break

    return locations


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "rb") as f:
        grid: list[list[Char]] = [list(line.strip()) for line in f]
    antinodes: set[Location] = set()
    antennas: defaultdict[Char, list[Location]] = defaultdict(list)
    for row_num, row in enumerate(grid):
        for col_num, val in enumerate(row):
            if val != ord("."):
                antennas[val].append((row_num, col_num))
    rows = len(grid)
    cols = len(grid[0])
    for antenna_group in antennas.values():
        antinodes |= find_antinodes(antenna_group, rows, cols)
    print(len(antinodes))


if __name__ == "__main__":
    main()
