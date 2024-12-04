#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 2 Part 2

import sys
from typing import NamedTuple

keypad = [
    "  1  ",
    " 234 ",
    "56789",
    " ABC ",
    "  D  ",
]


class Point(NamedTuple):
    x: int
    y: int


handlers = {
    "U": lambda position: Point(position.x, (position.y - 1)),
    "L": lambda position: Point((position.x - 1), position.y),
    "D": lambda position: Point(position.x, (position.y + 1)),
    "R": lambda position: Point((position.x + 1), position.y),
}


def process_line(instructions: str, position: Point) -> Point:
    for instruction in instructions:
        new_pos = handlers[instruction](position)
        if abs(new_pos.x - 2) + abs(new_pos.y - 2) <= 2:
            position = new_pos

    return position


def main() -> None:
    position = Point(x=0, y=2)
    with open(sys.argv[1], "r") as f:
        for line in f:
            position = process_line(line.strip(), position)
            print(keypad[position.y][position.x], end="")
    print()


if __name__ == "__main__":
    main()
