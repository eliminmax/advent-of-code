#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 7 Part 1

import sys
from typing import NamedTuple
from operator import add, mul


class Equation(NamedTuple):
    target: int
    values: list[int]


def is_possible_from(start: int, tgt: int, *remaining: int) -> bool:
    if not remaining:
        if start == tgt:
            return True
        return False

    results: list[int] = []
    for func in (add, mul):
        result: int = func(start, remaining[0])
        if result <= tgt:
            results.append(result)
    return any(
        map(lambda r: is_possible_from(r, tgt, *remaining[1:]), results)
    )


def target_if_possible(equation: Equation) -> int:
    # complete rework compared to part 1
    if is_possible_from(0, equation.target, *equation.values):
        return equation.target
    return 0


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        equations: list[Equation] = [
            Equation(int(a), [int(i) for i in b.split()])
            for a, b in (line.split(": ") for line in f)
        ]
    print(sum(map(target_if_possible, equations)))


if __name__ == "__main__":
    main()
