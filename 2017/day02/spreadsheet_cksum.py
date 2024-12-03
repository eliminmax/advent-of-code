#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 2 Part 1

import sys


def main():
    with open(sys.argv[1], "r") as f:
        lines = [[int(i) for i in line.split()] for line in f if line]
    print(sum(map(lambda line: max(line) - min(line), lines)))


if __name__ == "__main__":
    main()
