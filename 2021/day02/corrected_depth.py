#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2021 Day 2 Part 2

import sys


def main():
    with open(sys.argv[1], "r") as f:
        instructions: list[tuple[str, int]] = [
            (a, int(b)) for a, b in (line.strip().split() for line in f)
        ]
    forward_dist = 0
    aim = 0
    depth = 0
    for instruction in instructions:
        if instruction[0] == "forward":
            forward_dist += instruction[1]
            depth += aim * instruction[1]
        elif instruction[0] == "up":
            aim -= instruction[1]
        elif instruction[0] == "down":
            aim += instruction[1]
        else:
            raise ValueError
    print(forward_dist * depth)


if __name__ == "__main__":
    main()
