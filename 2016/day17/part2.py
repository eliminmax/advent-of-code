#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 17 Part 2

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


def longest_path(salt: str) -> int:
    queue: deque[tuple[str, tuple[int, int], str]] = deque(
        ((md5(salt), (0, 0), ""),)
    )
    actions = {
        "U": lambda r, c: (r - 1, c),
        "D": lambda r, c: (r + 1, c),
        "L": lambda r, c: (r, c - 1),
        "R": lambda r, c: (r, c + 1),
    }
    longest_path_len = 0
    while queue:
        md5_hash, (r, c), path = queue.pop()
        if (r, c) == (3, 3):
            if len(path) > longest_path_len:
                longest_path_len = len(path)
            continue
        for o in parse_options(md5_hash, (r, c)):
            next_loc = actions[o](r, c)
            queue.append((md5(f"{salt}{path}{o}"), next_loc, path + o))
    return longest_path_len


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        salt = f.read().strip()
    print(longest_path(salt))


if __name__ == "__main__":
    main()
