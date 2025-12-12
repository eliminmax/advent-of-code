#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2019 Day 6 Part 2

import sys
from collections import defaultdict, deque


def hops_to_santa(item: str, **kwargs: str) -> int:
    connections: defaultdict[str, list[str]] = defaultdict(list)
    for orbiter, orbitee in kwargs.items():
        connections[orbitee].append(orbiter)
        connections[orbiter].append(orbitee)
    distances: dict[str, int] = {"SAN": 0}
    queue = deque(connections["SAN"])
    while queue:
        item = queue.popleft()
        next_checks: set[str] = set()
        connected_distances: list[int] = []
        for con in connections[item]:
            if con in distances:
                connected_distances.append(distances[con])
            else:
                next_checks.add(con)
        distances[item] = min(connected_distances) + 1
        queue.extend(next_checks)
    return distances[item] - 2


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        raw_mapping = {
            k: v for v, k in (line.strip().split(")") for line in f)
        }
    print(hops_to_santa("YOU", **raw_mapping))


if __name__ == "__main__":
    main()
