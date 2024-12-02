#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 1 Part 2

import sys


def main():
    with open(sys.argv[1], "r") as f:
        expenses = [int(i) for i in f]
    for index, val in enumerate(expenses):
        for inner_index, inner_val in enumerate(expenses[index+1:]):
            for i in expenses[inner_index+1:]:
                if val + inner_val + i == 2020:
                    print(val * inner_val * i)
                    return
    sys.exit(1)


if __name__ == "__main__":
    main()
