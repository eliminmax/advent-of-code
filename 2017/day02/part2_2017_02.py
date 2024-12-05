#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 2 Part 2

import sys


def line_sum(line: list[int]) -> int:
    """Finds the pair of values in a line that evenly divide.
    Returns the result of that division"""
    line = sorted(line, reverse=True)
    for index, val in enumerate(line):
        for i in line[index + 1 :]:
            if val % i == 0:
                return val // i
    raise ValueError


def main() -> None:
    with open(sys.argv[1], "r") as f:
        lines = [[int(i) for i in line.split()] for line in f if line]
    print(sum(map(line_sum, lines)))


if __name__ == "__main__":
    main()
