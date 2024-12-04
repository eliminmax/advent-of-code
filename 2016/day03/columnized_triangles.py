#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 3 Part 1

import sys


def is_valid(a: int, b: int, c: int) -> bool:
    return (a < b + c) and (b < a + c) and (c < a + b)


def main() -> None:
    valid_triangles = 0
    with open(sys.argv[1], "r") as f:
        lines = list(f)
    trios = [
        (a.split(), b.split(), c.split())
        for a, b, c in zip(lines[::3], lines[1::3], lines[2::3])
    ]
    for trio in trios:
        for i in range(3):
            if is_valid(int(trio[0][i]), int(trio[1][i]), int(trio[2][i])):
                valid_triangles += 1
    print(valid_triangles)


if __name__ == "__main__":
    main()
