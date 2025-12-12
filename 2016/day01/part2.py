#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 1 Part 2

import sys

turns: dict[str, dict[str, str]] = {
    "R": dict(zip("NESW", "ESWN")),
    "L": dict(zip("NESW", "WNES")),
}


def main() -> None:
    positions_visited: list[tuple[int, int]] = []
    pos: tuple[int, int] = (0, 0)
    direction = "N"

    with open(sys.argv[1], "r") as f:
        for step in f.read().split(", "):
            direction = turns[step[0]][direction]
            distance = int(step[1:])
            match direction:
                # this is messy and inefficient, but it works fine for the
                # expected input size.
                case "N":
                    segment = [
                        (pos[0], pos[1] + i) for i in range(distance + 1)
                    ]
                case "E":
                    segment = [
                        (pos[0] + i, pos[1]) for i in range(distance + 1)
                    ]
                case "S":
                    segment = [
                        (pos[0], pos[1] - i) for i in range(distance + 1)
                    ]
                case "W":
                    segment = [
                        (pos[0] - i, pos[1]) for i in range(distance + 1)
                    ]
            # skip first point in segment, as it's shared with previous segment
            for pos in segment[1:]:
                if pos in positions_visited:
                    print(abs(sum(pos)))
                    return
            positions_visited += segment
    sys.exit(1)


if __name__ == "__main__":
    main()
