#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 19 Part 1

import sys
import re


def main() -> None:
    rules: list[tuple[str, str]] = []
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        # the following will keep going until it hits the blank delimiter line
        while rule_str := next(f).strip():
            a, b = rule_str.split(" => ")
            rules.append((a, b))
        starting_mol = next(f).strip()
    results: set[str] = set()
    for pat, replac in rules:
        for match in re.finditer(pat, starting_mol):
            start, end = match.span()
            results.add(starting_mol[:start] + replac + starting_mol[end:])
    print(len(results))


if __name__ == "__main__":
    main()
