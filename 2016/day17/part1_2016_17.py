#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 17 Part 1

import sys
from collections import deque
from hashlib import md5 as hash_md5


def parse_options(md5_str: str, loc: tuple[int, int]) -> str:
    s = ""
    for c, d in zip(md5_str[:4], "UDLR"):
        if c in "bcdef":
            s += d
    if loc[0] == 0:
        s = s.replace("U", "")
    elif loc[0] == 3:
        s = s.replace("D", "")
    if loc[1] == 0:
        s = s.replace("L", "")
    elif loc[1] == 3:
        s = s.replace("R", "")
    return s


def md5(s: str) -> str:
    return hash_md5(s.encode("utf-8")).hexdigest()


def shortest_path(salt: str) -> str:
    queue: deque[tuple[str, tuple[int, int], str]] = deque(
        ((md5(salt), (0, 0), ""),)
    )
    actions = {
        "U": lambda r, c: (r - 1, c),
        "D": lambda r, c: (r + 1, c),
        "L": lambda r, c: (r, c - 1),
        "R": lambda r, c: (r, c + 1),
    }
    while queue:
        md5_hash, (r, c), path = queue.pop()
        for o in parse_options(md5_hash, (r, c)):
            next_loc = actions[o](r, c)
            if next_loc == (3, 3):
                return path + o
            queue.append((md5(f"{salt}{path}{o}"), next_loc, path + o))
    raise ValueError(f"no valid path found for {salt}")


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        salt = f.read().strip()
    print(shortest_path(salt))


if __name__ == "__main__":
    main()
