#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 5 Part 2

import sys
import re

repeat_pat = re.compile(r"(.).\1")
doubles_pat = re.compile(r"(..).*\1")


def main() -> None:
    nice = 0
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            if doubles_pat.search(line) and repeat_pat.search(line):
                nice += 1
    print(nice)


if __name__ == "__main__":
    main()
