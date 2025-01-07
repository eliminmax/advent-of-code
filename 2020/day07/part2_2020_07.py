#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 7 Part 2

import sys
import re
from typing import TypeAlias, cast

BagRules: TypeAlias = dict[str, int | dict[str, int]]

_outer_bag = r"(\w+ \w+) bags contain "
_inner_bags = r"((?:\d+ \w+ \w+ bags?)(?:, \d+ \w+ \w+ bags?)*)"
_no_bags = "no other bags"
rule_pat = re.compile(rf"^{_outer_bag}(?:{_inner_bags}|{_no_bags})\.$")
contents_pat = re.compile(r"(\d+) ([\w ]+) bags?")


def resolve_rule(rule: str, bag_rules: BagRules) -> int:
    if isinstance(bag_rules[rule], int):
        return cast(int, bag_rules[rule])
    contained = cast(dict[str, int], bag_rules[rule])
    total = 0
    for bag, count in contained.items():
        total += count + (resolve_rule(bag, bag_rules) * count)
    bag_rules[rule] = total
    return total


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        raw_rules = [m for m in (rule_pat.match(line) for line in f) if m]
    bag_rules: dict[str, int | dict[str, int]] = {}
    for rule in raw_rules:
        outer, inner = rule.groups()
        if inner is None:
            bag_rules[outer] = 0
        else:
            bag_rules[outer] = {
                k: int(v)
                for v, k in (m.groups() for m in contents_pat.finditer(inner))
            }
    print(resolve_rule("shiny gold", bag_rules))


if __name__ == "__main__":
    main()
