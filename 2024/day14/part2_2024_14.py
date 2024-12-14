#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 14 Part 2

import sys
from typing import NamedTuple, Optional
from functools import cache
from math import floor
import enum
import re
import asyncio
from itertools import count
from pathlib import Path
from os import environ


# set width and height
WIDTH, HEIGHT = (11, 7) if environ.get("AOC_TESTING", "") else (101, 103)
MID_WIDTH = floor(WIDTH / 2)
MID_HEIGHT = floor(HEIGHT / 2)


STATES_DIR = Path(__file__).resolve().parent.joinpath("states")
STATES_DIR.mkdir(exist_ok=True)
STATES_DIR.joinpath(".gitignore").write_text("*\n")


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


async def output_state(round: int, robots: tuple[PatrolBot, ...]) -> None:
    try:
        locations = [False] * WIDTH * HEIGHT
        for robot in robots:
            locations[robot.y * WIDTH + robot.x] = True
        if any(
            all((a, b, c, d, e))
            for a, b, c, d, e in zip(
                locations[:-5:5],
                locations[1:-4:5],
                locations[2:-3:5],
                locations[3:-2:5],
                locations[4::5],
            )
        ):
            chars = ["#" if loc else " " for loc in locations]
            repstr = ""
            for i in range(0, WIDTH * HEIGHT, WIDTH):
                repstr += "".join(chars[i : i + WIDTH]) + "\n"
            STATES_DIR.joinpath(str(round)).write_text(repstr)
    except asyncio.CancelledError:
        pass
    except KeyboardInterrupt:
        pass


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

    states: set[int] = set()
    counter = count(0)
    while hash(robots) not in states:
        round = next(counter)
        asyncio.run(output_state(round, robots))
        robots = simulate_round(robots)


if __name__ == "__main__":
    print(
        "Watch for a file with a christmas tree in " + str(STATES_DIR) + ".",
        file=sys.stderr,
    )
    print(
        (
            "Its name will be the round number. Once you have that, kill this "
            "program with ctrl+C. It will catch the keyboard interrupt and "
            "cleanly exit."
        ),
        file=sys.stderr,
    )
    try:
        main()
    except KeyboardInterrupt:
        pass
