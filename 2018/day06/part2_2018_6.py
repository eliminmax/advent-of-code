#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2018 Day 6 Part 2

from collections.abc import Iterable
import sys

from advent_math import Point


def total_distance(row: int, col: int, points: Iterable[Point]) -> int:
    current = Point(row=row, col=col)
    return sum((p.manhattan_distance(current)) for p in points)


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        points = [
            Point(row=int(r), col=int(c))
            for r, c in (line.split(", ") for line in f if line)
        ]
    min_row = min(p.row for p in points)
    max_row = max(p.row for p in points)
    min_col = min(p.col for p in points)
    max_col = max(p.col for p in points)

    # find the sizes of non-infinite areas
    total = 0
    # doesn't actually check if region is contiguous, but gets the right answer
    # for my input
    for row in range(min_row, max_row + 1):
        for col in range(min_col, max_col + 1):
            if total_distance(row, col, points) < 10000:
                total += 1

    print(total)


if __name__ == "__main__":
    main()
