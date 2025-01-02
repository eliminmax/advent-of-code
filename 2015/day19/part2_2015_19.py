#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 19 Part 2
# based entirely on u/askalski's insights shared in their answer, but they
# worked it out by hand, not programmatically, using a different approach based
# on the insights.

# This was after days of struggling and failing to find a simple enough crash
# course in computational linguistics to implement the CYK algorithm.

# https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4etju/

import sys
import re


def transform(s: str) -> str:
    return s.replace("Rn", "(").replace("Y", ",").replace("Ar", ")")


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        # the following will keep going until it hits the blank delimiter line
        while next(f).strip():
            pass
        mol = re.sub("[A-Z][a-z]?", "X", transform(next(f).strip()))
    steps = 0
    while len(mol) > 1:
        while "XX" in mol:
            mol = mol.replace("XX", "X", 1)
            steps += 1
        while "X(X)" in mol:
            mol = mol.replace("X(X)", "X", 1)
            steps += 1
        while "X(X,X)" in mol:
            mol = mol.replace("X(X,X)", "X", 1)
            steps += 1
        while "X(X,X,X)" in mol:
            mol = mol.replace("X(X,X,X)", "X", 1)
            steps += 1
    print(steps)


if __name__ == "__main__":
    main()
