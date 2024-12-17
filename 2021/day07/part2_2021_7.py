#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2021 Day 7 Part 2

# I expected to need to do a binary search, and my original adjustment of the
# fuel_cost function was slow enough to need one for sure, but I was sure that
# there would be a faster way than sum(sum(range(1, abs(pos - target) + 1))) to
# get the answer - I just didn't remember it. Sure enough, the triangular
# number formula gets the answer much faster.
# Thanks to Akshat Tamrakar's StackOverflow answer for the reminder:
# https://stackoverflow.com/a/60348809

import sys
from collections.abc import Sequence


def tally_up(upper: int) -> int:
    return upper * (upper + 1) // 2


def fuel_cost(target: int, positions: Sequence[int]) -> int:
    return sum(tally_up(abs(pos - target)) for pos in positions)


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        positions = [int(i.strip()) for i in f.read().split(",")]

    print(
        min(
            fuel_cost(i, positions)
            for i in range(min(positions), max(positions) + 1)
        )
    )


if __name__ == "__main__":
    main()
