#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 4 Part 1

from typing import NamedTuple
import sys


class RoomName(NamedTuple):
    crypttext: str
    sector_id: int
    cksum: str


def is_valid(room_name: RoomName) -> bool:
    return "".join(
        sorted(
            # filter out non-lowercase characters, and convert to set to
            # remove duplicates
            set(filter(str.islower, room_name.crypttext)),
            # primary key is the number of occurrences in descending order, and
            # fall back to alphabetical order. Because the former is in
            # descending order and the latter is not, it's not as simple as
            # just passing reverse=True
            key=lambda c: (-room_name.crypttext.count(c), c),
        )
    ).startswith(room_name.cksum)


def main() -> None:
    with open(sys.argv[1], "r") as f:
        rooms: list[RoomName] = [
            RoomName(a, b, c)
            for a, b, c in
            # last character is newline
            ((line[:-11], int(line[-11:-8]), line[-7:-2]) for line in f)
        ]
    print(sum(p.sector_id for p in rooms if is_valid(p)))


if __name__ == "__main__":
    main()
