#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 10 Part 2

# I was originally trying to think through a clever way to cache sub-sequences
# on the assumption that the extra ten rounds would cause problems for me, but
# before going too far down that route, I decided to try just changing the
# round number, just to see what happened. That worked.

import sys
from itertools import groupby


def look_and_say(num_str: str) -> str:
    return "".join(
        f"{len(g)}{g[0]}" for g in (list(g) for _, g in groupby(num_str))
    )


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        num_str = f.read().strip()
    for i in range(50):
        num_str = look_and_say(num_str)
    print(len(num_str))


if __name__ == "__main__":
    main()
