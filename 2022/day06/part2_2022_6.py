#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 6 Part 2

import sys
from itertools import count


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        signal = f.read().strip()
    for i in count(14):
        if len(set(signal[i - 14 : i])) == 14:
            print(i)
            break


if __name__ == "__main__":
    main()
