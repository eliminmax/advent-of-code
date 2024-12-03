#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 2 Part 1

import sys

scoring_rules: dict = {
    "X": lambda elf: ("BAC".index(elf) * 3) + 1,
    "Y": lambda elf: ("CBA".index(elf) * 3) + 2,
    "Z": lambda elf: ("ACB".index(elf) * 3) + 3,
}


def main():
    score = 0
    with open(sys.argv[1], "r") as f:
        for round in f:
            elf_move, my_move = round.split()
            score += scoring_rules[my_move](elf_move)
    print(score)


if __name__ == "__main__":
    main()
