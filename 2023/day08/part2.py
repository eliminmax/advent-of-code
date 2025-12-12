#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 8 Part 2

# I saw on the subreddit that LCM works for other peoples input as well, but
# it does not seem like it would be a general solution

import sys
from itertools import count
from math import lcm


def process_cycle(
    start: str, moves: str, elements: dict[str, dict[str, str]]
) -> int:
    current = start
    mov_len = len(moves)
    for i in count(0):
        mov_i = i % mov_len
        current = elements[current][moves[mov_i]]
        if current.endswith("Z"):
            return i + 1
    raise RuntimeError


def main() -> None:
    elements: dict[str, dict[str, str]] = {}
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        moves = next(f).strip()
        assert next(f).strip() == ""
        for line in f:
            line = line.replace("(", "").replace(")", "").replace(",", "")
            elem, _, left, right = line.split()
            elements[elem] = {"R": right, "L": left}
    cycles = {
        process_cycle(s, moves, elements) for s in elements if s.endswith("A")
    }
    print(lcm(*(cycle for cycle in cycles)))


if __name__ == "__main__":
    main()
