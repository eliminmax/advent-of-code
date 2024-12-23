#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 23 Part 1

import sys
from collections import defaultdict


def main() -> None:
    links: defaultdict[str, set[str]] = defaultdict(set)
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            a, b = line.strip().split("-")
            links[a].add(b)
            links[b].add(a)
    trios: set[frozenset[str]] = set()
    for a in links:
        if not a.startswith("t"):
            continue
        for b in links[a]:
            for c in links[b]:
                if a in links[c]:
                    trios.add(frozenset((a, b, c)))
    print(len(trios))


if __name__ == "__main__":
    main()
