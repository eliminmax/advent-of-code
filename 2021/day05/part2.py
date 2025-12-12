#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2021 Day 5 Part 2

import sys
from typing import NamedTuple
from collections import defaultdict


class Point(NamedTuple):
    x: int
    y: int

    @staticmethod
    def parse(s: str) -> "Point":
        x, y = s.split(",")
        return Point(x=int(x), y=int(y))


class LineSegment(NamedTuple):
    a: Point
    b: Point

    @staticmethod
    def parse(s: str) -> "LineSegment":
        a, b = s.split(" -> ")
        return LineSegment(a=Point.parse(a), b=Point.parse(b))

    def points(self) -> list[Point]:
        """taking advantage of the fact that only axis-aligned segments or
        segments at 45° angles are going to show up"""
        if self.a.x == self.b.x:
            x_start, x_stop = sorted((self.a.y, self.b.y))
            return [Point(self.a.x, y) for y in range(x_start, x_stop + 1)]
        elif self.a.y == self.b.y:
            y_start, y_stop = sorted((self.a.x, self.b.x))
            return [Point(x, self.a.y) for x in range(y_start, y_stop + 1)]

        start, end = sorted((self.a, self.b), key=lambda p: p.x)
        if start.y < end.y:
            # positive slope
            point_count = (end.y - start.y) + 1
            return [
                Point(start.x + i, start.y + i) for i in range(point_count)
            ]
        point_count = (start.y - end.y) + 1
        return [Point(start.x + i, start.y - i) for i in range(point_count)]


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        segments: list[LineSegment] = [
            LineSegment.parse(line.strip()) for line in f
        ]
    locs: defaultdict[Point, int] = defaultdict(int)
    for segment in segments:
        for point in segment.points():
            locs[point] += 1
    print(sum(1 for point in locs if locs[point] > 1))


if __name__ == "__main__":
    main()
