#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 17 Part 2

import sys


def gen_combos(containers: list[int], remaining: int) -> list[list[int]]:
    fits = [[c] for c in containers if c == remaining]
    containers = [c for c in containers if c < remaining]
    for i, c in enumerate(containers):
        fits += [
            [c] + combo
            for combo in gen_combos(containers[i + 1 :], remaining - c)
        ]
    return fits


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        containers = [int(line.strip()) for line in f]
    containers.sort(reverse=True)
    combo_lens = [len(c) for c in gen_combos(containers, 150)]
    print(combo_lens.count(min(combo_lens)))


if __name__ == "__main__":
    main()
