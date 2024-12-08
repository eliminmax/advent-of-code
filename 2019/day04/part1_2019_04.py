#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2019 Day 4 Part 1

import sys
import re
from itertools import pairwise

repeat_pat = re.compile(r"(\d)\1")


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        range_min, range_max = (int(i) for i in f.read().strip().split("-"))

    counter = 0
    for n in range(range_min, range_max + 1):
        n_str: str = str(n)
        if all(map(lambda s: int(s[0]) <= int(s[1]), pairwise(n_str))):
            if repeat_pat.search(n_str):
                counter += 1
    print(counter)


if __name__ == "__main__":
    main()
