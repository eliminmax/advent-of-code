#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 4 Part 1

from typing import NamedTuple, TypeAlias
import sys
from string import ascii_lowercase as lowers
from functools import cache


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
            # decending order and the latter is not, it's not as simple as just
            # passing reverse=True
            key=lambda c: (-room_name.crypttext.count(c), c),
        )
    ).startswith(room_name.cksum)


Cipher: TypeAlias = dict[str, str]


@cache
def gen_cipher(shift: int) -> Cipher:
    return dict(zip(lowers, lowers[shift:] + lowers[:shift]))


def decipher_with(cipher: Cipher, crypttext: str) -> str:
    return "".join(
        map(
            lambda c: cipher[c] if c in cipher else c,
            crypttext
        )
    )


def decode(room_name: RoomName) -> str:
    return decipher_with(
        gen_cipher(room_name.sector_id % 26),
        room_name.crypttext
    )


def main() -> None:
    with open(sys.argv[1], "r") as f:
        rooms: list[RoomName] = [
            RoomName(a, b, c)
            for a, b, c in
            # last character is newline
            ((line[:-11], int(line[-11:-8]), line[-7:-2]) for line in f)
        ]
    for room in filter(is_valid, rooms):
        decoded = decode(room)
        if "pole" in decoded or "north" in decoded:
            print(room.sector_id)


if __name__ == "__main__":
    main()
