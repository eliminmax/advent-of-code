#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 24 Part 2

import sys
from itertools import combinations
from functools import reduce
from operator import mul


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        packages = [int(line.strip()) for line in f]
    target_weight = sum(packages) // 4
    max_search_len = len(packages) // 4
    options: list[list[int]] = []
    for i in range(max_search_len + 1):
        options = [
            list(c)
            for c in combinations(packages, i)
            if sum(c) == target_weight
        ]
        if options:
            break
    option_entanglements = sorted(reduce(mul, option) for option in options)
    print(option_entanglements[0])


if __name__ == "__main__":
    main()
