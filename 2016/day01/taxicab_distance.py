#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 1 Part 1

import sys


turns: dict[str, dict[str, str]] = {
    "R": dict(zip("NESW", "ESWN")),
    "L": dict(zip("NESW", "WNES")),
}


def main():
    distances: dict[str, int] = {"N": 0, "E": 0, "S": 0, "W": 0}
    current_direction = "N"
    with open(sys.argv[1], "r") as f:
        for step in f.read().split(", "):
            current_direction = turns[step[0]][current_direction]
            distances[current_direction] += int(step[1:])
    ns_distance = abs(distances["N"] - distances["S"])
    ew_distance = abs(distances["E"] - distances["W"])
    print(ns_distance + ew_distance)


if __name__ == "__main__":
    main()
