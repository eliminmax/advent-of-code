#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 3 Part 1

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


def main() -> None:
    with open(sys.argv[1], "r") as f:
        rows: list[Iterable[str]] = [line.strip() for line in f]
    cols = column_generator(rows)
    # move over 3 and down 1
    total = 0
    # skip starting column
    _ = next(cols)
    for i in range(1, len(rows)):
        # 3 over, 1 down
        _ = next(cols)
        _ = next(cols)
        if next(cols)[i] == "#":
            total += 1
    print(total)


if __name__ == "__main__":
    main()
