#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 9 Part 1

import sys
import re

pat = re.compile(r"\((\d+)x(\d+)\)")


def expanded(file_contents: str) -> str:
    if match := pat.search(file_contents):
        chars, repeat = (int(i) for i in match.groups())
        start, end = match.span()
        raw_contents = file_contents[:start]
        raw_contents += file_contents[end : end + chars] * repeat
        return raw_contents + expanded(file_contents[end + chars :])
    return file_contents


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        file_contents = "".join(f.read().split())
    print(len(expanded(file_contents)))


if __name__ == "__main__":
    main()
