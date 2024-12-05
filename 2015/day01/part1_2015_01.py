#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 1 Part 1
import sys


def floor_count(instruction: str) -> int:
    return instruction.count("(") - instruction.count(")")


def main() -> None:
    with open(sys.argv[1], "r") as f:
        # treating  f as an iterable iterates over lines in f
        print(floor_count(f.read()))


if __name__ == "__main__":
    main()
