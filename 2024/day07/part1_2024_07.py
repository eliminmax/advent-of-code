#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 07 Part 1

import sys
from typing import NamedTuple
from collections.abc import Callable, Iterable
from operator import add, mul
from itertools import accumulate


class Equation(NamedTuple):
    target: int
    values: list[int]


def is_possible(equation: Equation) -> None | int:
    # prepare for something cursed
    op_count = len(equation.values) - 1
    specifier = f"0{op_count}b"
    for i in range(2 ** op_count):
        # map the zeroes in the binary representation of i to add, and the
        # ones to mul
        ops: Iterable[Callable[[int, int], int]] = map(
            lambda c: add if c == '0' else mul,
            f"{i:{specifier}}"
        )
        *_, result = accumulate(
            equation.values[1:],
            func=lambda a, b: next(ops)(a, b),
            initial=equation.values[0]
        )
        if result == equation.target:
            return equation.target
    return None


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        equations: list[Equation] = [
            Equation(int(a), [int(i) for i in b.split()])
            for a, b in (line.split(": ") for line in f)
        ]
    print(sum(filter(lambda f: f is not None, map(is_possible, equations))))


if __name__ == "__main__":
    main()
