#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 7 Part 2

import sys
import re


bracketed_pat = re.compile(r"\[[^\]]*\]")


def rearrange_hypernext(s: str) -> str:
    chars = list(s)
    re_add = []
    for match in list(bracketed_pat.finditer(s))[::-1]:
        start, end = match.span()
        re_add += chars[start:end] + ["@"]
        chars[start:end] = ["@"]
    new_s = "".join(chars) + "§" + "".join(re_add)
    return new_s


def main() -> None:
    counter = 0
    main_pat = re.compile(r"([^\[])(?!\1)([^\[])\1.*§.*\2\1\2")
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            if main_pat.search(rearrange_hypernext(line.strip())):
                counter += 1
    print(counter)


if __name__ == "__main__":
    main()
