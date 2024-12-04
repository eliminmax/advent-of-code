#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 4 Part 1

import sys
from collections.abc import Iterable

# pre-compute list of offsets to use
check_offsets: list[list[tuple[int, int]]] = [list() for _ in range(8)]
for i in range(1, 4):
    check_offsets[0].append((0, i))  # horizontal
    check_offsets[1].append((0, -i))  # horizontal backwards
    check_offsets[2].append((i, 0))  # vertical
    check_offsets[3].append((-i, 0))  # vertical backwards
    check_offsets[4].append((i, i))  # diagonal up
    check_offsets[5].append((i, -i))  # diagonal down
    check_offsets[6].append((-i, i))  # diagonal backwards up
    check_offsets[7].append((-i, -i))  # diagonal backwards down


def count_xmases_at(row: int, col: int, grid: Iterable[Iterable[str]]) -> int:
    # check for "X" outside of loop rather than checking for each offset
    if grid[row][col] != "X":
        return 0
    matches = 0
    for offset_sequence in check_offsets:
        # keep looking in this direction until either a non-matching character
        # is found or it attempts to look out-of-bounds
        for i, (row_off, col_off) in enumerate(offset_sequence):
            # negative indices are not an index error, so explicitly check
            if row + row_off < 0 or col + col_off < 0:
                break
            try:
                if grid[row + row_off][col + col_off] != "MAS"[i]:
                    break
            except IndexError:
                break
        else:
            # had to look it off, because I never use for...else.
            # executes if loop completed unbroken
            matches += 1
    return matches


def main():
    with open(sys.argv[1], "r") as f:
        grid: list[str] = [line.strip() for line in f]
    total = 0
    for row_num, row in enumerate(grid):
        for col_num in range(len(row)):
            total += count_xmases_at(row_num, col_num, grid)
    print(total)


if __name__ == "__main__":
    main()
