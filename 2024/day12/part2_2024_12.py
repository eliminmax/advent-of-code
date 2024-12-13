#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 12 Part 2

import sys
from collections.abc import Iterable
from itertools import product


class Garden:
    def __init__(self, plot: Iterable[str]):
        self._grid: list[list[str]] = [
            [c for c in row.strip()] for row in plot
        ]
        self._rows: int = len(self._grid)
        self._cols: int = len(self._grid[0])
        self._uncounted: list[list[bool]] = [
            [True] * self._cols for _ in range(self._rows)
        ]

    @staticmethod
    def _score(matches: list[tuple[int, int]]) -> int:
        # while I'd realized that number of edges and number of corners are
        # equal, I'd been unable to get counting corners to work until I saw
        # u/Polaric_Spiral's comment linked here, and the diagram helped me
        # quite a bit.
        # reddit.com/r/adventofcode/comments/1hcdnk0/comment/m1nkmol
        offset_sets = (
            # bottom right corner
            lambda r, c: (r, c + 1) not in matches
            and (r + 1, c) not in matches,
            # bottom left corner
            lambda r, c: (r, c - 1) not in matches
            and (r + 1, c) not in matches,
            # top left corner
            lambda r, c: (r, c - 1) not in matches
            and (r - 1, c) not in matches,
            # top right corner
            lambda r, c: (r, c + 1) not in matches
            and (r - 1, c) not in matches,
            # bottom left inner corner
            lambda r, c: (r + 1, c + 1) not in matches
            and (r + 1, c) in matches
            and (r, c + 1) in matches,
            # bottom right inner corner
            lambda r, c: (r + 1, c - 1) not in matches
            and (r + 1, c) in matches
            and (r, c - 1) in matches,
            # top left inner corner
            lambda r, c: (r - 1, c - 1) not in matches
            and (r - 1, c) in matches
            and (r, c - 1) in matches,
            # top right inner corner
            lambda r, c: (r - 1, c + 1) not in matches
            and (r - 1, c) in matches
            and (r, c + 1) in matches,
        )

        corners = 0
        for r, c in matches:
            for offs in offset_sets:
                if offs(r, c):
                    corners += 1
        return len(matches) * corners

    def region(self, row: int, col: int) -> int:
        def _traverse(row: int, col: int, plant: str) -> list[tuple[int, int]]:
            if self._grid[row][col] != plant:
                return []
            self._uncounted[row][col] = False
            region = [(row, col)]
            if row and self._uncounted[row - 1][col]:
                region += _traverse(row - 1, col, plant)
            if row < self._rows - 1 and self._uncounted[row + 1][col]:
                region += _traverse(row + 1, col, plant)
            if col and self._uncounted[row][col - 1]:
                region += _traverse(row, col - 1, plant)
            if col < self._cols - 1 and self._uncounted[row][col + 1]:
                region += _traverse(row, col + 1, plant)
            return region

        plant = self._grid[row][col]

        matches = _traverse(row, col, plant)
        matches.sort()
        return self._score(matches)

    def fence_count(self) -> int:
        total = 0
        for row, col in product(range(self._rows), range(self._cols)):
            if self._uncounted[row][col]:
                total += self.region(row, col)
        return total


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        garden = Garden(f)
    print(garden.fence_count())


if __name__ == "__main__":
    main()
