#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 11 Part 2

import sys
from functools import lru_cache


@lru_cache(maxsize=16384)
def blink_at(stone: int, remaining_blinks: int = 74) -> int:
    if remaining_blinks == 0:
        # at this point, only the number of stones matters
        return 2 if len(str(stone)) % 2 == 0 else 1
    if stone == 0:
        return blink_at(1, remaining_blinks - 1)
    if len(stone_str := str(stone)) % 2 == 0:
        cut = len(stone_str) // 2
        a = blink_at(int(stone_str[cut:]), remaining_blinks - 1)
        b = blink_at(int(stone_str[:cut]), remaining_blinks - 1)
        return a + b
    return blink_at(stone * 2024, remaining_blinks - 1)


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        stones: list[int] = [int(i) for i in f.read().split()]
    total = 0
    for stone in stones:
        total += blink_at(stone)
    print(total)


if __name__ == "__main__":
    main()
