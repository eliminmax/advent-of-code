#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 2 Part 2

import sys


def main():
    count = 0
    with open(sys.argv[1], "r") as f:
        for line in f:
            # some nasty parsing, but more readable and faster to write than
            # a regex
            rule_text, passwd = line.strip().split(": ")
            minmax, c = rule_text.split(" ")
            pos_1, pos_2 = (int(i) - 1 for i in minmax.split("-"))
            # exactly one of the 2 positions must have the specified character
            # "!=" is being used as an xor operator here.
            if (passwd[pos_1] == c) != (passwd[pos_2] == c):
                count += 1
    print(count)


if __name__ == "__main__":
    main()
