#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2019 Day 3 Part 1

import sys
from dataclasses import dataclass
from typing import Self
import enum


class Axis(enum.Enum):
    X = enum.auto()
    Y = enum.auto()


@dataclass
class Point:
    x: int
    y: int

    def distance(self):
        return abs(self.x) + abs(self.y)

    def get_value(self, axis: Axis):
        if axis == Axis.X:
            return self.x
        return self.y

    def __bool__(self) -> bool:
        return self.x != 0 or self.y != 0


@dataclass(order=True)
class Segment:
    axis: Axis
    start: Point
    length: int

    def get_end(self) -> Point:
        try:
            return self.end
        except AttributeError:
            if self.axis == Axis.X:
                self.end: Point = Point(
                    x=self.start.x + self.length, y=self.start.y
                )
            else:
                self.end = Point(x=self.start.y, y=self.start.y + self.length)
            return self.end

    def intersection(self, other: Self) -> None | Point:
        """Returns intersection point with other, or None if they don't
        intersect."""
        # handle segments along the same axis
        if self.axis == other.axis:
            other_axis = Axis.Y if self.axis == Axis.X else Axis.X
            if (self.start.get_value(other_axis)) != (
                other.start.get_value(other_axis)
            ):
                return None
            a, b = sorted(
                (self, other), key=lambda s: s.start.get_value(self.axis)
            )
            a_end = a.start.get_value(self.axis) + a.length
            b_start = b.start.get_value(self.axis)
            if a_end == b_start:
                return b.start
            # wrote the following to handle an edge case than never comes up:

            # elif a_end > b_start:
            #     if abs(a_end) < abs(b_start):
            #         if self.axis == Axis.X:
            #             return Point(x=a_end, y=self.start.y)
            #         return Point(x=self.start.x, y=a_end)
            #     if self.axis == Axis.X:
            #         return Point(x=b_start, y=self.start.y)
            #     return Point(x=self.start.x, y=b_start)
            return None

        x_seg, y_seg = (self, other) if self.axis == Axis.X else (other, self)
        if x_seg.start.x <= y_seg.start.x <= x_seg.get_end().x:
            if y_seg.start.y <= x_seg.start.y <= y_seg.get_end().y:
                return Point(x=y_seg.start.x, y=x_seg.start.y)
        return None


def segments(steps: list[str]) -> list[Segment]:
    x = y = 0
    seen_segments = []
    for step in steps:
        delta = int(step[1:])
        match step[0]:
            case "R":
                new_segment = Segment(
                    axis=Axis.X, start=Point(x, y), length=delta
                )
                x += delta
            case "L":
                x -= delta
                new_segment = Segment(
                    axis=Axis.X, start=Point(x, y), length=delta
                )
            case "U":
                new_segment = Segment(
                    axis=Axis.Y, start=Point(x, y), length=delta
                )
                y += delta
            case "D":
                y -= delta
                new_segment = Segment(
                    axis=Axis.Y, start=Point(x, y), length=delta
                )
        seen_segments.append(new_segment)
    return sorted(seen_segments, key=lambda s: (s.start.x, s.start.y))


def main():
    with open(sys.argv[1], "r") as f:
        w0_segments, w1_segments = (
            segments(line.strip().split(",")) for line in f
        )
    closest: int | None = None
    for w0_segment in w0_segments:
        for w1_segment in w1_segments:
            intersect = w0_segment.intersection(w1_segment)
            if intersect:
                distance = intersect.distance()
                if closest is None or closest > distance:
                    closest = distance
    print(closest)


if __name__ == "__main__":
    main()
