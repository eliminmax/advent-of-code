#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 3 Part 2

import sys
from string import ascii_letters

priorities: dict[str, int] = dict(zip(ascii_letters, range(1, 53)))


def common_item(sack_a: str, sack_b: str, sack_c: str) -> int:
    for item in sack_a:
        if item in sack_b and item in sack_c:
            return priorities[item]
    raise ValueError


def main() -> None:
    with open(sys.argv[1], "r") as f:
        sacks = [line.strip() for line in f]

    print(
        sum(
            common_item(a, b, c)
            for (a, b, c) in zip(sacks[::3], sacks[1::3], sacks[2::3])
        )
    )


if __name__ == "__main__":
    main()
