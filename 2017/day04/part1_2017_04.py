#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 4 Part 1

import sys


def main() -> None:
    counter = 0
    with open(sys.argv[1], "r") as f:
        lines = list(f)
    for line in lines:
        words = line.split()
        if sorted(set(words)) == sorted(words):
            counter += 1
    print(counter)


if __name__ == "__main__":
    main()
