#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 17 Part 1

import sys


def count_combos(containers: list[int], remaining: int) -> int:
    fits = sum(1 for c in containers if c == remaining)
    containers = [c for c in containers if c < remaining]
    for i, c in enumerate(containers):
        fits += count_combos(containers[i + 1 :], remaining - c)
    return fits


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        containers = [int(line.strip()) for line in f]
    containers.sort(reverse=True)
    print(count_combos(containers, 150))


if __name__ == "__main__":
    main()
