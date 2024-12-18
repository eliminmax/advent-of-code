#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 8 Part 2

import sys
from typing import TypeAlias

Display: TypeAlias = list[list[bool]]


def rotate_row(row: int, shift: int, grid: Display) -> None:
    grid[row] = grid[row][-shift:] + grid[row][:-shift]


def rotate_col(col: int, shift: int, grid: Display) -> None:
    shifted = [row[col] for row in grid]
    shifted = shifted[-shift:] + shifted[:-shift]
    for i, row in enumerate(grid):
        row[col] = shifted[i]


def rect(width: int, height: int, grid: Display) -> None:
    for row in range(height):
        grid[row][:width] = [True] * width


def main() -> None:
    grid: Display = [[False] * 50 for _ in range(6)]
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            words = line.strip().split()
            if words[0] == "rect":
                a, b = words[1].split("x")
                rect(int(a), int(b), grid)
                continue
            index = int(words[2].split("=")[1])
            shift = int(words[4])
            if words[1] == "row":
                rotate_row(index, shift, grid)
            else:
                rotate_col(index, shift, grid)
    # Unicode FULL BLOCK my beloved
    print("\n".join("".join("█" if b else " " for b in row) for row in grid))


if __name__ == "__main__":
    main()
