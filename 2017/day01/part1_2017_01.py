#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 1 Part 1

import sys


def main() -> None:
    with open(sys.argv[1], "r") as f:
        digits = f.read()
    count = 0
    for digit, next_digit in zip(digits, digits[1:] + digits[0]):
        if digit == next_digit:
            count += int(digit)
    print(count)


if __name__ == "__main__":
    main()
