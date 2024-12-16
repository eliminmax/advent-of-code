#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 7 Part 1

import sys
import re
from collections import deque

_outer_bag = r"(\w+ \w+) bags contain "
_inner_bags = r"((?:\d+ \w+ \w+ bags?)(?:, \d+ \w+ \w+ bags?)*)"
_no_bags = "no other bags"
rule_pat = re.compile(rf"^{_outer_bag}(?:{_inner_bags}|{_no_bags})\.$")
contents_pat = re.compile(r"\d+ ([\w ]+) bags?")


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        raw_rules = [m for m in (rule_pat.match(line) for line in f) if m]
    bag_rules: dict[str, list[str]] = {}
    for rule in raw_rules:
        outer, inner = rule.groups()
        if inner is None:
            bag_rules[outer] = []
        else:
            bag_rules[outer] = [
                m.groups()[0] for m in contents_pat.finditer(inner)
            ]
    have_gold: set[str] = set()
    to_check = deque(("shiny gold",))
    while len(to_check):
        color = to_check.popleft()
        for outer, inners in bag_rules.items():
            if color in inners:
                have_gold.add(outer)
                to_check.append(outer)
    print(len(have_gold))


if __name__ == "__main__":
    main()
