#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2018 Day 2 Part 2

import sys


def find_similar(box_id: str, known_ids: list[str]) -> None | str:
    for known_id in known_ids:
        for i in range(len(known_id)):
            if (
                box_id[:i] + box_id[i + 1 :]
                == known_id[:i] + known_id[i + 1 :]
            ):
                return box_id[:i] + box_id[i + 1 :]
    return None


def main() -> None:
    with open(sys.argv[1], "r") as f:
        lines = [line.strip() for line in f]
    for i in range(len(lines)):
        result = find_similar(lines[i], lines[:i] + lines[i + 1 :])
        if result is not None:
            print(result)
            break


if __name__ == "__main__":
    main()
