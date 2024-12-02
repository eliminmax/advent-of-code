#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD
import sys


def is_safe(vals: list[int]) -> bool:
    distances = [a - b for a, b in zip(vals[:-1], vals[1:])]
    if all(map(lambda n: n in (1, 2, 3), distances)):
        return True
    elif all(map(lambda n: n in (-1, -2, -3), distances)):
        return True
    return False


def main():
    with open(sys.argv[1], "r") as f:
        lines: list[str] = list(f)
    rows: list[list[int]] = [[int(i) for i in line.split()] for line in lines]
    print(len(list(filter(is_safe, rows))))


if __name__ == "__main__":
    main()
