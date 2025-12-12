#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 4 Part 2

import sys


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
    required = ("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid")
    for passport in passports:
        if all(map(lambda k: k in passport, required)):
            counter += 1
    print(counter)


if __name__ == "__main__":
    main()
