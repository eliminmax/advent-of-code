#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 4 Part 2

import sys
import re
from collections.abc import Callable


def validate_height(hgt: str) -> bool:
    n_str = hgt[:-2]
    if not n_str.isnumeric():
        return False
    n = int(n_str)
    match hgt[-2:]:
        case "in":
            return n in range(59, 77)
        case "cm":
            return n in range(150, 194)
        case _:
            return False


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        passport_strs: list[str] = [
            grouping.strip().replace("\n", " ")
            for grouping in f.read().split("\n\n")
        ]
    passports: list[dict[str, str]] = [
        {k: v for k, v in (entry.split(":") for entry in passport.split())}
        for passport in passport_strs
    ]
    counter = 0
    eye_colors = ("amb", "blu", "brn", "gry", "grn", "hzl", "oth")
    hair_color = re.compile("#[0-9a-f]{6}")
    required: dict[str, Callable[[str], bool]] = {
        "byr": lambda byr: int(byr) in range(1920, 2003),
        "iyr": lambda iyr: int(iyr) in range(2010, 2021),
        "eyr": lambda eyr: int(eyr) in range(2020, 2031),
        "hgt": validate_height,
        "hcl": lambda hcl: bool(hair_color.match(hcl)),
        "ecl": lambda ecl: ecl in eye_colors,
        "pid": lambda pid: pid.isnumeric() and len(pid) == 9,
    }
    for passport in passports:
        if all(
            map(lambda k: k in passport and required[k](passport[k]), required)
        ):
            counter += 1
    print(counter)


if __name__ == "__main__":
    main()
