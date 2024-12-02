#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD


# Solution to AoC 2015 Day 1 Part 2
import sys


def main():
    with open(sys.argv[1], "r") as f:
        index = 1
        floor = 0
        while instruction := f.read(1):
            if instruction == "(":
                floor += 1
            elif instruction == ")":
                floor -= 1
            if floor < 0:
                break
            index += 1
    print(index)


if __name__ == "__main__":
    main()
