#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 8 Part 1

import sys
from itertools import cycle


def main() -> None:
    elements: dict[str, dict[str, str]] = {}
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        move_cycle = cycle(next(f).strip())
        assert next(f).strip() == ""
        for line in f:
            line = line.replace("(", "").replace(")", "").replace(",", "")
            elem, _, left, right = line.split()
            elements[elem] = {"R": right, "L": left}
    moves = 0
    current_element = "AAA"
    while current_element != "ZZZ":
        moves += 1
        current_element = elements[current_element][next(move_cycle)]
    print(moves)


if __name__ == "__main__":
    main()
