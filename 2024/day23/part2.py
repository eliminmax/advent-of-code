#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 23 Part 2

import sys
from collections import defaultdict
from typing import cast


def main() -> None:
    links: defaultdict[str, set[str]] = defaultdict(set)
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            a, b = line.strip().split("-")
            links[a].add(b)
            links[b].add(a)
    groups: set[frozenset[str]] = set()
    for node, linked_nodes in links.items():
        group: set[str] = {node}
        for linked_node in linked_nodes:
            if all(n in links[linked_node] for n in group):
                group.add(linked_node)
        groups.add(frozenset(group))
    print(",".join(sorted(max(groups, key=lambda g: len(cast(frozenset, g))))))


if __name__ == "__main__":
    main()
