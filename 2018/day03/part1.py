#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2018 Day 3 Part 1

import sys
import re


def claim(
    start_x: int,
    start_y: int,
    width: int,
    height: int,
    claims: dict[int, dict[int, int]],
):
    for x in range(start_x, start_x + width):
        if x not in claims:
            claims[x] = {}
        for y in range(start_y, start_y + height):
            if y in claims[x]:
                claims[x][y] += 1
            else:
                claims[x][y] = 0


parse_pattern = re.compile(
    "#[0-9]+ @ (?P<start_x>[0-9]+),(?P<start_y>[0-9]+): "
    "(?P<width>[0-9]+)x(?P<height>[0-9]+)"
)


def main() -> None:
    claims: dict[int, dict[int, int]] = {}
    with open(sys.argv[1], "r") as f:
        for line in f:
            match = parse_pattern.match(line)
            assert match is not None
            match_dict = match.groupdict()
            claim(**{k: int(match_dict[k]) for k in match_dict}, claims=claims)
    total = 0
    for row in claims.values():
        # map to bool to clamp values greater than 1 to 1
        total += sum(map(bool, row.values()))
    print(total)


if __name__ == "__main__":
    main()
