#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 3 Part 2

import sys
from itertools import cycle
from collections.abc import Iterable, Generator, Sequence
from typing import TypeVar

T = TypeVar("T")


def column_generator(
    template: Sequence[Iterable[T]],
) -> Generator[Sequence[T], None, None]:
    cycles = [cycle(i) for i in template]
    while True:
        yield [next(i) for i in cycles]


def slope_check(rows: Sequence[Iterable[str]], right: int, down: int) -> int:
    cols = column_generator(rows)
    # skip starting column
    _ = next(cols)
    total = 0
    for i in range(down, len(rows), down):
        result: Sequence[str] = "."
        # 3 over, 1 down
        for _ in range(right):
            result = next(cols)
        if result[i] == "#":
            total += 1
    return total


def main() -> None:
    with open(sys.argv[1], "r") as f:
        rows: list[Iterable[str]] = [line.strip() for line in f]
    product = 1
    for right, down in ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2)):
        product *= slope_check(rows, right, down)
    print(product)


if __name__ == "__main__":
    main()
