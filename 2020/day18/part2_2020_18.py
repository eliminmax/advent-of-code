#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 18 Part 2

import sys
import re

SUBEXPR_PATTERN = re.compile(r"\([0-9 +*]*\)")
ADDITION_PATTERN = re.compile(r"(\d+) \+ (\d+)")


def resolve(expr: str) -> int:
    expr = expr.strip()

    while m := SUBEXPR_PATTERN.search(expr):
        start, stop = m.span()
        expr_val = resolve(expr[start + 1: stop - 1])
        expr = expr[:start] + str(expr_val) + expr[stop:]

    while m := ADDITION_PATTERN.search(expr):
        a, b = m.groups()
        start, stop = m.span()
        expr = expr[:start] + f"{int(a) + int(b)}" + expr[stop:]

    terms = expr.split(" ")

    i = int(terms[0])

    terms = terms[1:]

    while terms:
        match terms[0]:
            case "+":
                i += int(terms[1])
            case "-":
                i -= int(terms[1])
            case "*":
                i *= int(terms[1])
        terms = terms[2:]

    return i


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        print(sum(resolve(line) for line in f))


if __name__ == "__main__":
    main()
