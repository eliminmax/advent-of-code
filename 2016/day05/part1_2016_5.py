#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2016 Day 5 Part 1

import sys
from hashlib import md5
from itertools import count


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        door = f.read().strip()
    counter = count(0)

    passwd = ""
    for _ in range(8):
        while True:
            i = next(counter)
            md5sum = md5(f"{door}{i}".encode("utf-8"), usedforsecurity=False)
            md5sum_hex = md5sum.hexdigest()
            if md5sum_hex.startswith("00000"):
                passwd += md5sum_hex[5]
                break
    print(passwd)


if __name__ == "__main__":
    main()
