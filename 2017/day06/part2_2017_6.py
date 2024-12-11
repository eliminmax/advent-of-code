#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 6 Part 2

import sys


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        blocks = [int(i) for i in f.read().split()]
    seen_states: list[list[int]] = []
    redists = 0
    redist_sz = len(blocks)
    while blocks not in seen_states:
        seen_states.append(blocks[:])
        redist_amnt = max(blocks)
        index = blocks.index(redist_amnt)
        blocks[index] = 0
        while redist_amnt:
            index += 1
            index %= redist_sz
            blocks[index] += 1
            redist_amnt -= 1
        redists += 1
    print(redists - seen_states.index(blocks))


if __name__ == "__main__":
    main()
