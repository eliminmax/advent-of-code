#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 7 Part 1

import sys
import re


abba_negation = re.compile(r"\[[^\]]*(.)(?!\1)(.)\2\1[^\]]*\]")
abba = re.compile(r"(.)(?!\1)(.)\2\1")


def main() -> None:
    counter = 0
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            if abba.search(line) and not abba_negation.search(line):
                counter += 1
    print(counter)


if __name__ == "__main__":
    main()
