#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 2 Part 1

import sys
from typing import NamedTuple

keypad = ["123", "456", "789"]


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
        if new_pos.x in range(3) and new_pos.y in range(3):
            position = new_pos
    return position


def main():
    position = Point(x=1, y=1)
    with open(sys.argv[1], "r") as f:
        for line in f:
            position = process_line(line.strip(), position)
            print(keypad[position.y][position.x], end="")
    print()


if __name__ == "__main__":
    main()
