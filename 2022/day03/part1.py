#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 3 Part 1

import sys
from string import ascii_letters

priorities: dict[str, int] = dict(zip(ascii_letters, range(1, 53)))


def process_rucksack(contents: str) -> int:
    divider = len(contents) // 2
    comp_a = contents[:divider]
    comp_b = contents[divider:]
    for item in comp_a:
        if item in comp_b:
            return priorities[item]
    raise ValueError


def main() -> None:
    total = 0
    with open(sys.argv[1], "r") as f:
        for line in f:
            total += process_rucksack(line.strip())
    print(total)


if __name__ == "__main__":
    main()
