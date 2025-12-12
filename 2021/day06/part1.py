#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2021 Day 6 Part 1

import sys
from functools import cache


@cache
def lanternfish_count(cycle: int, remaining_days: int = 80) -> int:
    if remaining_days == 0:
        return 1
    if cycle == 0:
        newborn = lanternfish_count(8, remaining_days - 1)
        thisone = lanternfish_count(6, remaining_days - 1)
        return newborn + thisone
    return lanternfish_count(cycle - 1, remaining_days - 1)


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        fish: list[int] = [int(i) for i in f.read().strip().split(",")]
    print(sum(lanternfish_count(i) for i in fish))


if __name__ == "__main__":
    main()
