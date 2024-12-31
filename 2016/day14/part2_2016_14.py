#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 14 Part 2

import sys
from hashlib import md5
from itertools import count
from functools import cache
import re


MAYBE_KEY_PAT = re.compile(r"(.)\1\1")


@cache
def md5_lookup(s: str) -> str:
    md5sum = md5(s.encode("utf-8"), usedforsecurity=False)
    return md5sum.hexdigest()


@cache
def stretch(s: str) -> str:
    rounds = 2016
    while rounds:
        s = md5_lookup(s)
        rounds -= 1
    return md5_lookup(s)


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        salt = f.read().strip()
    key_count = 0
    for i in count(0):
        if match := MAYBE_KEY_PAT.search(stretch(f"{salt}{i}")):
            tgt = match.groups()[0] * 5
            search_span = range(i + 1, i + 1001)
            if any(tgt in stretch(f"{salt}{n}") for n in search_span):
                key_count += 1
                if key_count == 64:
                    print(i)
                    sys.exit(0)


if __name__ == "__main__":
    main()
