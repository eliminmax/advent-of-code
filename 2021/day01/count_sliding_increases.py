#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2021 Day 1 Part 2

import sys


def main() -> None:
    with open(sys.argv[1], "r") as f:
        depth_levels = [int(i) for i in f]
    counter = 0
    smoothed_levels = [
        a + b + c
        for a, b, c in zip(
            depth_levels[1:], depth_levels[2:], depth_levels[:-2]
        )
    ]
    for current, prev in zip(smoothed_levels[1:], smoothed_levels[:-1]):
        if current > prev:
            counter += 1
    print(counter)


if __name__ == "__main__":
    main()
