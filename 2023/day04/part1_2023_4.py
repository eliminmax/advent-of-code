#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 4 Part 1

import sys


def parse_line(line: str) -> int:
    card, winning = line.split(": ")[1].strip().split("|")
    winning_nums = [int(i) for i in winning.split()]
    score = 0
    for num in (int(i) for i in card.split()):
        if num in winning_nums:
            if not score:
                score = 1
            else:
                score *= 2
    return score


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        print(sum(map(parse_line, f)))


if __name__ == "__main__":
    main()
