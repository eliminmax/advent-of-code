#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 1 Part 1

from pathlib import Path
import sys


def main() -> None:
    infile = Path(sys.argv[1])
    with infile.open("r") as f:
        nums: list[tuple[int, int]] = [
            (int(a), int(b)) for a, b in (line.split() for line in list(f))
        ]
    col0, col1 = zip(*nums)
    dist_map = map(
        lambda a, b: abs(a - b),
        sorted(col0),
        sorted(col1),
    )
    print(sum(dist_map))


if __name__ == "__main__":
    main()
