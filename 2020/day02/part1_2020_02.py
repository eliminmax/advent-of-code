#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 2 Part 1

import sys


def main() -> None:
    count = 0
    with open(sys.argv[1], "r") as f:
        for line in f:
            # some nasty parsing, but more readable and faster to write than
            # a regex
            rule_text, passwd = line.strip().split(": ")
            minmax, c = rule_text.split(" ")
            min_ct, max_ct = (int(i) for i in minmax.split("-"))
            if passwd.count(c) in range(min_ct, max_ct + 1):
                count += 1
    print(count)


if __name__ == "__main__":
    main()
