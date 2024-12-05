#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 3 Part 1

import sys
import re

symbol_pat = re.compile("[^.]")


def append_to_gears(
    row: int,
    span: tuple[int, int],
    grid: list[str],
    gears: dict[tuple[int, int], list[int]],
) -> None:
    # clamp span to bounds
    start, end = (max(span[0] - 1, 0), min(span[1], len(grid[0]) - 1))
    to_check: list[tuple[int, int]] = []
    if row > 0:
        to_check += [(row - 1, i) for i in range(start, end + 1)]
    if row < len(grid) - 1:
        to_check += [(row + 1, i) for i in range(start, end + 1)]
    if start < span[0]:
        to_check.append((row, start))
    if end == span[1]:
        to_check.append((row, end))

    val = int(grid[row][span[0] : span[1]])
    for location in to_check:
        if grid[location[0]][location[1]] != "*":
            continue
        if location in gears:
            gears[location].append(val)
        else:
            gears[location] = [val]


def main() -> None:
    filter_pat = re.compile("[^*0-9]")
    with open(sys.argv[1], "r") as f:
        # replace al characters not of interest with "."
        grid: list[str] = [filter_pat.sub(".", line.strip()) for line in f]
    num_pat = re.compile("[0-9]+")
    gears: dict[tuple[int, int], list[int]] = {}
    for row_num, row in enumerate(grid):
        for match in num_pat.finditer(row):
            append_to_gears(row_num, match.span(), grid, gears)
    total = 0
    for gear_nums in gears.values():
        if len(gear_nums) == 2:
            total += gear_nums[0] * gear_nums[1]
    print(total)


if __name__ == "__main__":
    main()
