#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 1 Part 1

import sys


def main():
    with open(sys.argv[1], "r") as f:
        expenses = [int(i) for i in f]
    for index, val in enumerate(expenses):
        for i in expenses[index+1:]:
            if val + i == 2020:
                print(val * i)
                return
    sys.exit(1)


if __name__ == "__main__":
    main()
