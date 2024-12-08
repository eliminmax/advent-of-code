#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2019 Day 4 Part 2

import sys
from itertools import pairwise


def has_pair(s: str) -> bool:
    pairs: list[str] = list("".join((a, b)) for a, b in pairwise(s))
    for i, pair in enumerate(pairs):
        if pairs[i + 1 :] and pair == pairs[i + 1]:
            continue
        if i and pair == pairs[i - 1]:
            continue
        a = pair[0]
        b = pair[1]
        if a == b:
            return True
    return False


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        range_min, range_max = (int(i) for i in f.read().strip().split("-"))

    counter = 0
    for n in range(range_min, range_max + 1):
        n_str: str = str(n)
        if all(map(lambda s: int(s[0]) <= int(s[1]), pairwise(n_str))):
            if has_pair(n_str):
                counter += 1
    print(counter)


if __name__ == "__main__":
    main()
