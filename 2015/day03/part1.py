#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 3 Part 1

import sys


def main() -> None:
    x, y = 0, 0
    visited: set[tuple[int, int]] = {(x, y)}
    with open(sys.argv[1], "r") as f:
        for direction in f.read():
            match direction:
                case ">":
                    x += 1
                case "<":
                    x -= 1
                case "v":
                    y += 1
                case "^":
                    y -= 1
            visited.add((x, y))
    print(len(visited))


if __name__ == "__main__":
    main()
