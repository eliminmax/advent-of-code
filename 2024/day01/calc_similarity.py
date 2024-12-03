#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 1 Part 2

from pathlib import Path
import sys
from functools import cache


# cache lookups
@cache
def get_frequency(n: int, col: list[int]) -> int:
    return n * col.count(n)


def main():
    infile = Path(sys.argv[1])
    with infile.open("r") as f:
        nums: list[tuple[int, int]] = [
            (int(a), int(b)) for a, b in (line.split() for line in list(f))
        ]
    col0, col1 = zip(*nums)
    similarity_map = map(lambda n: get_frequency(n, col1), col0)
    print(sum(similarity_map))


if __name__ == "__main__":
    main()
