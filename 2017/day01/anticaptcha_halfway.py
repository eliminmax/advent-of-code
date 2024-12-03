#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 1 Part 2

import sys


def main():
    with open(sys.argv[1], "r") as f:
        digits = f.read()
    count = 0
    cut = len(digits) // 2
    for digit, next_digit in zip(digits, digits[cut:] + digits[:cut]):
        if digit == next_digit:
            count += int(digit)
    print(count)


if __name__ == "__main__":
    main()
