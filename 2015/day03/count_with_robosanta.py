#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 3 Part 2

import sys


def main():
    positions: dict[bool, dict[str, int]] = {
        True: {"x": 0, "y": 0},
        False: {"x": 0, "y": 0},
    }
    visited: set[tuple[int, int]] = {(0, 0)}
    robo_turn = False
    with open(sys.argv[1], "r") as f:
        for direction in f.read():
            match direction:
                case ">":
                    positions[robo_turn]["x"] += 1
                case "<":
                    positions[robo_turn]["x"] -= 1
                case "v":
                    positions[robo_turn]["y"] += 1
                case "^":
                    positions[robo_turn]["y"] -= 1
                case _:
                    continue
            x = positions[robo_turn]["x"]
            y = positions[robo_turn]["y"]
            visited.add((x, y))
            robo_turn = not robo_turn
    print(len(visited))


if __name__ == "__main__":
    main()
