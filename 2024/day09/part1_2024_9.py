#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 9 Part 1

import sys
from itertools import count
from collections.abc import Iterable
from typing import Optional, TypeAlias

DiskMap: TypeAlias = list[Optional[int]]


def parse_blocks(ints: Iterable[int]) -> DiskMap:
    blocks: DiskMap = []
    id_gen = count(0)
    for index, length in enumerate(ints):
        if index % 2 == 0:
            blocks += [next(id_gen)] * length
        else:
            blocks += [None] * length
    return blocks


def move_blocks(blocks: DiskMap):
    removed = 0
    while None in blocks:
        popped = blocks.pop()
        if popped is not None:
            blocks[blocks.index(None)] = popped
        # either a None was popped, or a None was replaced by popped
        removed += 1
    blocks += [None] * removed


def checksum(blocks: DiskMap) -> int:
    return sum(i * b for i, b in enumerate(blocks) if b is not None)


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        disk = parse_blocks(int(c) for c in f.read())
    move_blocks(disk)
    print(checksum(disk))


if __name__ == "__main__":
    main()
