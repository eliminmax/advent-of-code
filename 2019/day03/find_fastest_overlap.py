#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2019 Day 3 Part 2

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
    reversed: bool
    initial_distance: int
    previous: None | list[Self] = None

    def distance(self, p: Point) -> int:
        if self.reversed:
            return self.initial_distance + (
                self.get_end().get_value(self.axis) - p.get_value(self.axis)
            )
        return self.initial_distance + (
            p.get_value(self.axis) - self.start.get_value(self.axis)
        )

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
            return None

        x_seg, y_seg = (self, other) if self.axis == Axis.X else (other, self)
        if x_seg.start.x <= y_seg.start.x <= x_seg.get_end().x:
            if y_seg.start.y <= x_seg.start.y <= y_seg.get_end().y:
                return Point(x=y_seg.start.x, y=x_seg.start.y)
        return None

    def soft_repr(self) -> str:
        return (
            f"Segment(axis={self.axis}, start={self.start}, "
            f"length={self.length}, reversed={self.reversed}, "
            f"initial_distance={self.initial_distance}, "
            f"previous={'None' if self.previous is None else '[...]'}"
        )

    def intersection_distance(self, other: Self) -> int | None:
        intersect = self.intersection(other)
        if not intersect:
            return None
        return self.distance(intersect) + other.distance(intersect)


def segments(steps: list[str]) -> list[Segment]:
    x = y = 0
    total_distance = 0
    seen_segments: list[Segment] = []
    for step in steps:
        delta = int(step[1:])
        match step[0]:
            case "R":
                new_segment = Segment(
                    axis=Axis.X,
                    start=Point(x, y),
                    length=delta,
                    reversed=False,
                    initial_distance=total_distance,
                    previous=seen_segments.copy(),
                )
                x += delta
            case "L":
                x -= delta
                new_segment = Segment(
                    axis=Axis.X,
                    start=Point(x, y),
                    length=delta,
                    reversed=True,
                    initial_distance=total_distance,
                    previous=seen_segments.copy(),
                )
            case "U":
                new_segment = Segment(
                    axis=Axis.Y,
                    start=Point(x, y),
                    length=delta,
                    reversed=False,
                    initial_distance=total_distance,
                    previous=seen_segments.copy(),
                )
                y += delta
            case "D":
                y -= delta
                new_segment = Segment(
                    axis=Axis.Y,
                    start=Point(x, y),
                    length=delta,
                    reversed=True,
                    initial_distance=total_distance,
                    previous=seen_segments.copy(),
                )
        seen_segments.append(new_segment)
        total_distance += delta
    return seen_segments


def main() -> None:
    with open(sys.argv[1], "r") as f:
        w0_segments, w1_segments = (
            segments(line.strip().split(",")) for line in f
        )
    closest: int | None = None
    for w0_segment in w0_segments:
        if closest is not None and closest < w0_segment.initial_distance:
            break
        for w1_segment in w1_segments:
            if closest is not None and closest < w1_segment.initial_distance:
                break
            intersect = w0_segment.intersection_distance(w1_segment)
            if intersect:
                if closest is None or closest > intersect:
                    closest = intersect
    print(closest)


if __name__ == "__main__":
    main()
