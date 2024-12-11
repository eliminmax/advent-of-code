#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 6 Part 2

import sys
from collections import defaultdict
from typing import TypeAlias


CharCount: TypeAlias = defaultdict[str, int]


def main() -> None:
    template: CharCount = defaultdict(int)
    position_counts: defaultdict[int, CharCount] = defaultdict(template.copy)
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            word = line.strip()
            for i, c in enumerate(word):
                position_counts[i][c] += 1
    most_common: list[str] = [""] * len(position_counts)
    for pos, counts in position_counts.items():
        most_common[pos] = min(counts, key=lambda c: counts[c])
    print("".join(most_common))


if __name__ == "__main__":
    main()
