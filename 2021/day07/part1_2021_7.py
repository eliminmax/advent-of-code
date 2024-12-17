#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2021 Day 7 Part 1

# I noticed that the fuel cost seems to only get higher the further from the
# answer it gets, so that could be used with a binary search to find the answer
# more efficeintly, assuming that holds for more than just the sample input,
# but this runs on less than a second on my computer, and was much faster to
# implement. We'll see how part 2 goes.

import sys
from collections.abc import Sequence


def fuel_cost(target: int, positions: Sequence[int]) -> int:
    return sum(abs(pos - target) for pos in positions)


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
