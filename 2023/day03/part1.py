#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 3 Part 1

import sys
import re


symbol_pat = re.compile("[^.]")


def parse_number(
    row: int, span: tuple[int, int], grid: list[str]
) -> int | None:
    # clamp span to bounds
    start, end = (max(span[0] - 1, 0), min(span[1], len(grid[0]) - 1))
    to_check = ""
    if row > 0:
        to_check += grid[row - 1][start : end + 1]
    if row < len(grid) - 1:
        to_check += grid[row + 1][start : end + 1]
    if start < span[0]:
        to_check += grid[row][start]
    if end == span[1]:
        to_check += grid[row][end]
    if symbol_pat.search(to_check):
        return int(grid[row][span[0] : span[1]])
    return None


def main() -> None:
    with open(sys.argv[1], "r") as f:
        grid: list[str] = [(line.strip()) for line in f]
    num_pat = re.compile("[0-9]+")
    total = 0
    for row_num, row in enumerate(grid):
        for match in num_pat.finditer(row):
            parsed = parse_number(row_num, match.span(), grid)
            if parsed is not None:
                total += parsed
    print(total)


if __name__ == "__main__":
    main()
