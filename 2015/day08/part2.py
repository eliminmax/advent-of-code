#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 8 Part 2

import sys


def main() -> None:
    count = 0
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in (i.strip() for i in f):
            # python's repr will use single quotes rather than escape double
            # quotes, so escape them manually to get an accurate count
            # my input does not have any single-quote characters anyway.
            count += len(repr(line).replace('"', '\\"')) - len(line)
    print(count)


if __name__ == "__main__":
    main()
