#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 5 Part 2

import sys
from hashlib import md5
from itertools import count
from typing import cast
from collections.abc import Sequence


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        door = f.read().strip()
    counter = count(0)

    passwd: list[None | str] = [None] * 8
    for _ in range(8):
        while True:
            i = next(counter)
            md5sum = md5(f"{door}{i}".encode("utf-8"), usedforsecurity=False)
            md5sum_hex: str = md5sum.hexdigest()
            if md5sum_hex.startswith("00000"):
                # I'd rather cast to a value that's too large than catch
                # constant ValueErrors
                hash_index = int(md5sum_hex[5], 16)
                if hash_index in range(8) and passwd[hash_index] is None:
                    passwd[hash_index] = md5sum_hex[6]
                    break
    print("".join(cast(Sequence[str], passwd)))


if __name__ == "__main__":
    main()
