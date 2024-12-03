#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2018 Day 3 Part 1

import sys
import re


def claim(
    num: int,
    start_x: int,
    start_y: int,
    width: int,
    height: int,
    claims: dict[int, dict[int, set[int]]],
):
    for x in range(start_x, start_x + width):
        if x not in claims:
            claims[x] = {}
        for y in range(start_y, start_y + height):
            if y in claims[x]:
                claims[x][y].add(num)
            else:
                claims[x][y] = {num}


parse_pattern = re.compile(
    "#(?P<num>[0-9]+) @ (?P<start_x>[0-9]+),(?P<start_y>[0-9]+): "
    "(?P<width>[0-9]+)x(?P<height>[0-9]+)"
)


def main():
    location_claims: dict[int, dict[int, set[int]]] = {}
    claim_nums: set[int] = set()
    with open(sys.argv[1], "r") as f:
        for line in f:
            match = parse_pattern.match(line).groupdict()
            args = {k: int(match[k]) for k in match}
            claim_nums.add(args["num"])
            claim(**args, claims=location_claims)
    # now, remove any ID that appears in a shared claim
    for column in location_claims.values():
        for row in column.values():
            # only care about positions with overlapping claims
            if len(row) <= 1:
                continue
            for claim_num in row:
                # if not already removed, then do so
                if claim_num in claim_nums:
                    claim_nums.remove(claim_num)
    for claim_num in claim_nums:
        print(claim_num)


if __name__ == "__main__":
    main()
