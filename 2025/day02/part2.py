#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2025 Day 02 Part 2

import sys
from itertools import chain
import re

BAD_ID_RE = re.compile(r"(\d+)(\1)+")


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        ranges = [
            range(int(a), int(b) + 1)
            for a, b in (s.split("-") for s in f.read().strip().split(","))
        ]
    print(sum(n for n in chain(*ranges) if BAD_ID_RE.fullmatch(str(n))))


if __name__ == "__main__":
    main()
