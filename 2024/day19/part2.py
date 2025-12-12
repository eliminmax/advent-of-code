#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 19 Part 2

import sys
from functools import cache


@cache
def possible_arrangements(pattern: str, towels: tuple[str, ...]) -> int:
    total = 0
    for towel in towels:
        if pattern == towel:
            total += 1
        elif pattern.startswith(towel):
            total += possible_arrangements(
                pattern.replace(towel, "", 1), towels
            )
    return total


def main() -> None:
    infile = sys.argv[1] if sys.argv[1:] else "input"

    with open(infile, "r") as f:
        towels: tuple[str, ...] = tuple(next(f).replace(",", "").split())
        assert next(f).strip() == ""
        patterns = [line.strip() for line in f]
    print(sum(possible_arrangements(pattern, towels) for pattern in patterns))


if __name__ == "__main__":
    main()
