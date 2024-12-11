#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2018 Day 6 Part 1

from collections import defaultdict
from collections.abc import Iterable
from typing import Optional
import sys

from advent_math import Point


def find_nearest(
    row: int, col: int, points: Iterable[Point]
) -> Optional[Point]:
    current = Point(row=row, col=col)
    tagged: list[tuple[Point, int]] = [
        (p, p.manhattan_distance(current)) for p in points
    ]
    tagged.sort(key=lambda t: t[1])
    if tagged[0][1] < tagged[1][1]:
        return tagged[0][0]
    return None


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
    # if a point on the outer edge is closest to a point, then an infinite
    # number will extend outwards.
    counts: defaultdict[Point, int] = defaultdict(int)

    # determine which are infinite - if it's on the edge, it'll keep going
    infinite: set[Point] = set()

    for row in range(min_row - 1, max_row + 2):
        nearest = find_nearest(row, min_col - 1, points)
        if nearest is not None:
            infinite.add(nearest)
        nearest = find_nearest(row, max_col + 1, points)
        if nearest is not None:
            infinite.add(nearest)

    for col in range(min_col - 1, max_col + 2):
        nearest = find_nearest(col, min_row - 1, points)
        if nearest is not None:
            infinite.add(nearest)
        nearest = find_nearest(col, max_row + 1, points)
        if nearest is not None:
            infinite.add(nearest)

    # find the sizes of non-infinite areas

    for row in range(min_row, max_row + 1):
        for col in range(min_col, max_col + 1):
            nearest = find_nearest(row, col, points)
            if nearest is not None and nearest not in infinite:
                counts[nearest] += 1

    print(max(counts.values()))


if __name__ == "__main__":
    main()
