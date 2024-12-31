#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 14 Part 1

import sys
from hashlib import md5
from itertools import count
from functools import cache
import re


MAYBE_KEY_PAT = re.compile(r"(.)\1\1")


@cache
def md5_lookup(salt: str, index: int) -> str:
    md5sum = md5(f"{salt}{index}".encode("utf-8"), usedforsecurity=False)
    return md5sum.hexdigest()


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        salt = f.read().strip()
    key_count = 0
    for i in count(0):
        if match := MAYBE_KEY_PAT.search(md5_lookup(salt, i)):
            tgt = match.groups()[0] * 5
            if any(tgt in md5_lookup(salt, n) for n in range(i + 1, i + 1001)):
                key_count += 1
                if key_count == 64:
                    print(i)
                    sys.exit(0)


if __name__ == "__main__":
    main()
