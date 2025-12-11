#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2025 Day 10 Part 2

import sys
import z3


def min_presses(line: str) -> int:
    words = line.split()[1:]
    optimizer = z3.Optimize()
    button_vals: list[list[int]] = []
    for word in words[:-1]:
        button_vals.append([int(i) for i in word[1:-1].split(',')])

    presses = z3.IntVector('b', len(button_vals))
    optimizer.assert_exprs(*((i >= 0) for i in presses))

    counter_vals: list[int] = [int(i) for i in words[-1][1:-1].split(',')]

    for i, target in enumerate(counter_vals):
        relevant = (b for b, vals in zip(presses, button_vals) if i in vals)
        optimizer.assert_exprs(z3.Sum(*relevant) == target)

    optimizer.minimize(z3.Sum(*presses))
    optimizer.check()

    return optimizer.model().evaluate(z3.Sum(*presses)).as_long()


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        print(sum(min_presses(line) for line in f))


if __name__ == "__main__":
    main()
