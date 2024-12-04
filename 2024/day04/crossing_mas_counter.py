#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 4 Part 2

import sys
from collections.abc import Sequence
from typing import TypeAlias, NamedTuple

Offset: TypeAlias = tuple[int, int]


class OffsetGroup(NamedTuple):
    m_offs: tuple[Offset, Offset]
    s_offs: tuple[Offset, Offset]


# list of offsets to use
check_offsets: Sequence[OffsetGroup] = (
    # M on top
    OffsetGroup(m_offs=((-1, -1), (-1, 1)), s_offs=((1, -1), (1, 1))),
    # M on left
    OffsetGroup(m_offs=((-1, -1), (1, -1)), s_offs=((-1, 1), (1, 1))),
    # M on bottom
    OffsetGroup(m_offs=((1, -1), (1, 1)), s_offs=((-1, -1), (-1, 1))),
    # M on right
    OffsetGroup(m_offs=((-1, 1), (1, 1)), s_offs=((-1, -1), (1, -1))),
)


def is_cross_at(row: int, col: int, grid: Sequence[Sequence[str]]) -> bool:
    if grid[row][col] != "A":
        return False
    for offs in check_offsets:
        a, b = offs.m_offs
        if (
            grid[row + a[0]][col + a[1]] != "M"
            or grid[row + b[0]][col + b[1]] != "M"
        ):
            continue
        # first 2 corners checked are "M", so if the other 2 aren't "S",
        # there's no need to keep checking other rotations
        c, d = offs.s_offs
        return bool(
            grid[row + c[0]][col + c[1]] == "S"
            and grid[row + d[0]][col + d[1]] == "S"
        )
    return False


def main() -> None:
    with open(sys.argv[1], "r") as f:
        grid: list[str] = [line.strip() for line in f]
    total = 0
    row_len = len(grid[0])
    # skip first and last row
    for row in range(1, len(grid) - 1):
        for col in range(1, row_len - 1):
            total += is_cross_at(row, col, grid)
    print(total)


if __name__ == "__main__":
    main()
