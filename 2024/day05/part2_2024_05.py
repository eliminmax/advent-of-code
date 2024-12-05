#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 5 Part 2

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


def process_rules(raw_rules: Iterable[PageOrder]) -> Rules:
    rules_dict: Rules = defaultdict(Rule._new)
    for a, b in raw_rules:
        rules_dict[a].before.append(b)
        rules_dict[b].after.append(a)
    return rules_dict


def cant_insert_before(entry: int, sequence: list[int], rules: Rules) -> bool:
    return any(
        map(
            lambda p: p in rules[entry].after or entry in rules[p].before,
            sequence,
        )
    )


def cant_insert_after(entry: int, sequence: list[int], rules: Rules) -> bool:
    return any(
        map(
            lambda p: p in rules[entry].before or entry in rules[p].after,
            sequence,
        )
    )


def mid_if_unordered(unordered: list[int], rules: Rules) -> int:
    # inefficient, but with the size of each line, still fast enough
    ordered: list[int] = [unordered[0]]
    # insert each item into a spot that meets its rules and others
    for entry in unordered[1:]:
        for i in range(len(ordered) + 1):
            if cant_insert_after(entry, ordered[:i], rules):
                continue
            if cant_insert_before(entry, ordered[i:], rules):
                continue
            ordered.insert(i, entry)
            break
        else:
            # executes if loop ran to completion
            raise ValueError(f"Can't fit {entry} in {ordered} with {rules=}")

        if ordered == unordered:
            return 0
    return ordered[len(ordered) // 2]


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
    print(sum(map(lambda o: mid_if_unordered(o, rules), orderings)))


if __name__ == "__main__":
    main()
