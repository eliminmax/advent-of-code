#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 5 Part 1

# Rewrote in Python because part 2 is basically impossible w/o capturing groups

import sys
import re

bad_string_pat = re.compile("ab|cd|pq|xy")
vowel_pat = re.compile(".*(?:[aeiou].*){3}")
double_pat = re.compile(r"(.)\1")


def main() -> None:
    nice = 0
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            if double_pat.search(line) and vowel_pat.search(line):
                if not bad_string_pat.search(line):
                    nice += 1
    print(nice)


if __name__ == "__main__":
    main()
