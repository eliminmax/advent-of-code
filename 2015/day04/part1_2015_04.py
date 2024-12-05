#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 4 Part 1

import sys
from hashlib import md5
from itertools import count


def main() -> None:
    with open(sys.argv[1], "r") as f:
        key = f.read().strip()
    for i in count(0):
        md5sum = md5(f"{key}{i}".encode("utf-8"), usedforsecurity=False)
        if md5sum.hexdigest().startswith("00000"):
            print(i)
            return


if __name__ == "__main__":
    main()
