#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 9 Part 2

import sys
import re
from functools import cache

pat = re.compile(r"\((\d+)x(\d+)\)")


@cache
def expanded(file_contents: str) -> int:
    total = 0
    while match := pat.search(file_contents):
        chars, repeat = (int(i) for i in match.groups())
        start, end = match.span()
        total += start + expanded(file_contents[end : end + chars] * repeat)
        file_contents = file_contents[end + chars :]
    return total + len(file_contents)


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        file_contents = "".join(f.read().split())
    print(expanded(file_contents))


if __name__ == "__main__":
    assert expanded("X(8x2)(3x3)ABCY") == len("XABCABCABCABCABCABCY")
    main()
