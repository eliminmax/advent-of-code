#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2021 Day 1 Part 1

import sys


def main():
    with open(sys.argv[1], "r") as f:
        depth_levels = [int(i) for i in f]
    counter = 0
    for current, prev in zip(depth_levels[1:], depth_levels[:-1]):
        if current > prev:
            counter += 1
    print(counter)


if __name__ == "__main__":
    main()
