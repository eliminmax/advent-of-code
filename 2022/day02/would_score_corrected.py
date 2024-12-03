#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 2 Part 2

import sys
from collections.abc import Callable

scoring_rules: dict[str, Callable[[str], int]] = {
    "R": lambda elf: ("BAC".index(elf) * 3) + 1,
    "P": lambda elf: ("CBA".index(elf) * 3) + 2,
    "S": lambda elf: ("ACB".index(elf) * 3) + 3,
}

movement_table: dict[str, dict[str, str]] = {}

# X means throw the round
movement_table["X"] = dict(zip("ABC", "SRP"))
# Y means end in a draw
movement_table["Y"] = dict(zip("ABC", "RPS"))
# Z means win the round
movement_table["Z"] = dict(zip("ABC", "PSR"))


def main():
    score = 0
    with open(sys.argv[1], "r") as f:
        for round in f:
            elf_move, outcome = round.split()
            my_move = movement_table[outcome][elf_move]
            score += scoring_rules[my_move](elf_move)
    print(score)


if __name__ == "__main__":
    main()
