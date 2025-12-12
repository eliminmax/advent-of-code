#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 3 Part 1

import sys
from math import sqrt, ceil


def spiral_distance(n: int) -> int:
    # determine what ring of the spiral n is in

    # each ring goes up to the square of an odd number - the innermost ring
    # contains 1**2, then it's (1**2)+1 through (3**2), then (3**2 + 1) through
    # (5**2), and so on. The starting and ending ring number can be calculated
    # with the following formula:
    ring_num: int = ceil(sqrt(n))
    if ring_num % 2 == 0:
        ring_num += 1
    ring_max = ring_num**2
    # thanks to kennytm's SO answer: https://stackoverflow.com/a/12141207
    # that is an answer to a different question than what I needed, but the
    # solution was similar enough to deserve acknowledgement
    distance_to_corner = min(
        map(
            lambda x: abs(x - n),
            (ring_max - (ring_num * i) for i in range(4)),
        )
    )

    return ring_num - distance_to_corner - 1


def main() -> None:
    with open(sys.argv[1], "r") as f:
        distance = int(f.read().strip())
    print(spiral_distance(distance))


if __name__ == "__main__":
    main()
