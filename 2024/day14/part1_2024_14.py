#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 14 Part 1

import sys
from typing import NamedTuple, Optional
from functools import cache
from math import floor
import enum
import re
from os import environ


# set width and height
WIDTH, HEIGHT = (11, 7) if environ.get("AOC_TESTING", "") else (101, 103)
MID_WIDTH = floor(WIDTH / 2)
MID_HEIGHT = floor(HEIGHT / 2)


class Quadrant(enum.Enum):
    TOP_RIGHT = enum.auto()
    TOP_LEFT = enum.auto()
    BOTTOM_RIGHT = enum.auto()
    BOTTOM_LEFT = enum.auto()


class PatrolBot(NamedTuple):
    x: int
    y: int
    dx: int
    dy: int

    def quadrant(self) -> Optional[Quadrant]:
        if self.x < MID_WIDTH:
            if self.y > MID_HEIGHT:
                return Quadrant.TOP_RIGHT
            elif self.y < MID_HEIGHT:
                return Quadrant.BOTTOM_LEFT
        elif self.x > MID_WIDTH:
            if self.y > MID_HEIGHT:
                return Quadrant.TOP_LEFT
            elif self.y < MID_HEIGHT:
                return Quadrant.BOTTOM_RIGHT
        return None


@cache
def next_round(bot: PatrolBot) -> PatrolBot:
    return PatrolBot(
        x=(bot.x + bot.dx) % WIDTH,
        y=(bot.y + bot.dy) % HEIGHT,
        dx=bot.dx,
        dy=bot.dy,
    )


@cache
def simulate_round(robots: tuple[PatrolBot, ...]) -> tuple[PatrolBot, ...]:
    return tuple(next_round(bot) for bot in robots)


def main() -> None:
    extract_pat = re.compile(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)")
    loaded_bots: list[PatrolBot] = []
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            matched = extract_pat.match(line)
            if matched is None:
                raise ValueError(
                    f"{line.strip()} could not be parsed with {extract_pat}"
                )
            loaded_bots.append(PatrolBot(*(int(i) for i in matched.groups())))
    robots: tuple[PatrolBot, ...] = tuple(loaded_bots)
    for _ in range(100):
        robots = simulate_round(robots)

    quads: list[Optional[Quadrant]] = [bot.quadrant() for bot in robots]
    print(
        quads.count(Quadrant.TOP_RIGHT)
        * quads.count(Quadrant.TOP_LEFT)
        * quads.count(Quadrant.BOTTOM_RIGHT)
        * quads.count(Quadrant.BOTTOM_LEFT)
    )


if __name__ == "__main__":
    main()
