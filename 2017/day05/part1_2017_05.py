#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 5 Part 1

import sys


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        jumps = [int(i) for i in f]
    steps = i = 0
    while i in range(len(jumps)):
        jump_len = jumps[i]
        jumps[i] += 1
        i += jump_len
        steps += 1
    print(steps)


if __name__ == "__main__":
    main()
