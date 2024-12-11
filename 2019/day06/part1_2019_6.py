#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2019 Day 6 Part 1

import sys
from typing import cast


class OrbitMap:
    def __init__(self, **kwargs: str):
        self._mappings: dict[str, str | int] = {**kwargs}
        self._mappings["COM"] = 0

    def get_orbit_count(self, thing: str) -> int:
        if isinstance(self._mappings[thing], int):
            return cast(int, self._mappings[thing])
        self._mappings[thing] = (
            self.get_orbit_count(cast(str, self._mappings[thing])) + 1
        )
        return cast(int, self._mappings[thing])

    def __repr__(self) -> str:
        return f"OrbitMap({self._mappings})"

    def tally(self) -> int:
        return sum(self.get_orbit_count(k) for k in self._mappings)


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        orbit_map = OrbitMap(
            **{k: v for v, k in (line.strip().split(")") for line in f)}
        )
    print(orbit_map.tally())


if __name__ == "__main__":
    main()
