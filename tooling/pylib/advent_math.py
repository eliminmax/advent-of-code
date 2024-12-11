"""A place for repeatedly-implemented mathematical constructs"""
# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD
from typing import NamedTuple, Self


class Point(NamedTuple):
    """A point on a grid"""

    row: int
    col: int

    def manhattan_distance(self, other: Self) -> int:
        """Calculate the taxicab distance to other"""
        return abs(self.row - other.row) + abs(self.col - other.col)
