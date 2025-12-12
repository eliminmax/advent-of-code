#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2018 Day 2 Part 1

import sys
import string


def has_n(box_id: str, count: int) -> bool:
    for c in string.ascii_lowercase:
        if box_id.count(c) == count:
            return True
    return False


def main() -> None:
    total_with_2 = 0
    total_with_3 = 0
    with open(sys.argv[1], "r") as f:
        for line in f:
            if has_n(line, 2):
                total_with_2 += 1
            if has_n(line, 3):
                total_with_3 += 1
    print(total_with_2 * total_with_3)


if __name__ == "__main__":
    main()
