#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 4 Part 2

import sys


def parse_line(line: str) -> int:
    card, winning = line.split(": ")[1].strip().split("|")
    winning_nums = [i for i in winning.split()]
    return sum(1 for n in (i for i in card.split()) if n in winning_nums)


def main() -> None:
    total_cards = 0
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        copy_tracker: list[int] = []
        for line in f:
            card_count = 1 + len(copy_tracker)
            total_cards += card_count
            copy_tracker = [i - 1 for i in copy_tracker if i > 1]
            card_val = parse_line(line)
            if card_val:
                copy_tracker += [card_val] * card_count
    print(total_cards)


if __name__ == "__main__":
    main()
