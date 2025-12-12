#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 11 Part 1

import sys
from itertools import chain


def stone_rules(stone: int) -> list[int]:
    if stone == 0:
        return [1]
    if len(stone_str := str(stone)) % 2 == 0:
        cutoff = len(stone_str) // 2
        return [int(stone_str[:cutoff]), int(stone_str[cutoff:])]
    return [stone * 2024]


def blink(stones: list[int]) -> list[int]:
    return list(chain.from_iterable(stone_rules(stone) for stone in stones))


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        stones: list[int] = [int(i) for i in f.read().split()]
    for _ in range(25):
        stones = blink(stones)
    print(len(stones))


if __name__ == "__main__":
    main()
