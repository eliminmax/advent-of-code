#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 3 Part 1

import sys


def main() -> None:
    valid_triangles = 0
    with open(sys.argv[1], "r") as f:
        for line in f:
            a, b, c = (int(i) for i in line.split())
            if (a < b + c) and (b < a + c) and (c < a + b):
                valid_triangles += 1
    print(valid_triangles)


if __name__ == "__main__":
    main()
