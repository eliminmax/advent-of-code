# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
# SPDX-FileCopyrightText: 2008 Can Berk Güder
#
# SPDX-License-Identifier: CC-BY-SA-3.0

from typing import NamedTuple
from collections.abc import Generator


class Coords(NamedTuple):
    x: int
    y: int


def spiral(ring_count: int) -> Generator[Coords]:
    """Generatpr for coordinate pairs to iterate through n rings,
    represented as a 2D array

    based on Can Berk Güder's StackOverflow answer
    https://stackoverflow.com/a/398302
    """
    x = y = 0
    midpoint = ring_count - 1
    size = ring_count * 2 - 1
    dx = 0
    dy = -1
    for _ in range(size**2):
        if -size / 2 < x <= size / 2 and -size / 2 < y <= size / 2:
            # flip Y axis for consistency with Advent of Code
            yield Coords(x + midpoint, (-y) + midpoint)
        if x == y or (x < 0 and x == -y) or (x > 0 and x == 1 - y):
            dx, dy = -dy, dx
        x, y = x + dx, y + dy
