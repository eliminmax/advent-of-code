#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 19 Part 1

import sys
from functools import cache


@cache
def possible_arrangement(pattern: str, towels: tuple[str, ...]) -> bool:
    for towel in towels:
        if pattern == towel:
            return True
        elif pattern.startswith(towel):
            if possible_arrangement(pattern.replace(towel, "", 1), towels):
                return True
    return False


def main() -> None:
    infile = sys.argv[1] if sys.argv[1:] else "input"

    with open(infile, "r") as f:
        towels: tuple[str, ...] = tuple(next(f).replace(",", "").split())
        assert next(f).strip() == ""
        patterns = [line.strip() for line in f]
    total = 0
    for pat in patterns:
        if possible_arrangement(pat, towels):
            total += 1
    print(total)


if __name__ == "__main__":
    main()
