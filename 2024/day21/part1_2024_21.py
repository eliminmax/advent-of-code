#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 21 Part 1

# Some of the recursive logic was giving me trouble, and u/ThatMakesMeM0ist's
# post on the subreddit was very helpful in working it out, even though it was
# for part 2, rather than part 1.
# https://www.reddit.com/r/adventofcode/comments/1hjx0x4


import sys
from functools import cache
from itertools import pairwise

# the code to generate the button links is in a separate file
from button_links import BUTTON_LINKS


def build_seq(
    base_seq: str, index: int, prev: str, current: str, results: set[str]
):
    if index == len(base_seq):
        results.add(current)
        return
    for path in BUTTON_LINKS[prev][base_seq[index]]:
        build_seq(
            base_seq, index + 1, base_seq[index], current + path + "A", results
        )


@cache
def shortest_len(base_seq: str, depth: int) -> int:
    if depth == 0:
        return len(base_seq)
    total = 0
    # can't just use .split("A") as that would not include the "A" itself,
    # and [s + "A" for s in base_seq.split("A") if s] has a too-low result,
    # while [s + "A" for s in base_seq.split("A")] has a too-high result.
    split_indexes = [0]
    split_indexes += [i + 1 for i, c in enumerate(base_seq) if c == "A"]
    split_indexes += [len(base_seq)]
    for start, end in pairwise(split_indexes):
        sub_seq = base_seq[start:end]
        results: set[str] = set()
        build_seq(sub_seq, 0, "A", "", results)
        total += min(shortest_len(r, depth - 1) for r in results)

    return total


def main() -> None:
    total = 0
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            line = line.strip()
            sequence_length = shortest_len(line.strip(), 3)
            complexity = int(line.replace("A", "")) * sequence_length
            total += complexity
    print(total)


if __name__ == "__main__":
    main()
