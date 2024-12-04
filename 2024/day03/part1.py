#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 3 Part 1

import sys
import re

mul_pat = re.compile("mul\\(([0-9]+),([0-9]+)\\)")


def main() -> None:
    total = 0
    with open(sys.argv[1], "r") as f:
        for match in mul_pat.finditer(f.read()):
            group = match.groups()
            total += int(group[0]) * int(group[1])
    print(total)


if __name__ == "__main__":
    main()
