#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 5 Part 1

import sys
from typing import TypeAlias, Self
from collections.abc import Iterable
from collections import defaultdict
from dataclasses import dataclass


@dataclass
class Rule:
    before: list[int]
    after: list[int]

    @classmethod
    def _new(cls) -> Self:
        return cls(before=[], after=[])


PageOrder: TypeAlias = tuple[int, int]
Rules: TypeAlias = defaultdict[int, Rule]


def process_rules(rules: Iterable[PageOrder]) -> Rules:
    rules_dict: Rules = defaultdict(Rule._new)
    for a, b in rules:
        rules_dict[a].before.append(b)
        rules_dict[b].after.append(a)
    return rules_dict


def mid_if_ordered(ordering: list[int], rules: Rules) -> int:
    for i, entry in enumerate(ordering):
        for prev in ordering[:i]:
            if prev in rules[entry].before or entry in rules[prev].after:
                return 0
    return ordering[len(ordering) // 2]


def main() -> None:
    with open(sys.argv[1], "r") as f:
        lines = [line.strip() for line in f]
    split = lines.index("")
    raw_rules: list[PageOrder] = [
        (int(a), int(b))
        for a, b in (line.split("|") for line in lines[:split])
    ]
    orderings: Iterable[list[int]] = (
        [int(i) for i in line.split(",")] for line in lines[split + 1 :]
    )
    rules = process_rules(raw_rules)
    print(sum(map(lambda o: mid_if_ordered(o, rules), orderings)))


if __name__ == "__main__":
    main()
