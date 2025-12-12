#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 11 Part 2

# I expected part 2 to be what'll it be in 50 passwords or something, not the
# one after the next one. That's why I used a generator for part 1. Made this
# a lot easier anyway.

import sys
from collections.abc import Generator
from string import ascii_lowercase
import re


forbidden = "iol"
letter_slides = [
    slide
    for slide in (ascii_lowercase[n : n + 3] for n in range(24))
    if not any(c in slide for c in forbidden)
]
chr_sequence = tuple(filter(lambda c: c not in forbidden, ascii_lowercase))
chr_cycle = dict(zip(chr_sequence[-1:] + chr_sequence[:-1], chr_sequence))
chr_cycle |= {"i": "j", "o": "p", "l": "m"}  # legacy support for old passwords


pairs_pat = re.compile(r"([a-z])\1.*([a-z])\2")


def next_chr(old_chr: str) -> tuple[str, bool]:
    "Return the next character in order, and a bool used to flag wraparound"
    if old_chr == "z":
        return ("a", True)
    return (chr_cycle[old_chr], False)


def valid_pass(passwd: str) -> bool:
    if any(c in passwd for c in forbidden):
        return False
    if not any(slide in passwd for slide in letter_slides):
        return False
    return pairs_pat.search(passwd) is not None


def santa_pwgen(prev_pass: str) -> Generator[str, None, None]:
    base_pass = prev_pass
    while True:
        new_pass_chrs = list(base_pass)[::-1]
        i = 0
        wrap = True
        while wrap:
            new_pass_chrs[i], wrap = next_chr(new_pass_chrs[i])
            i += 1
            i %= len(prev_pass)
        base_pass = "".join(new_pass_chrs[::-1])
        if valid_pass(base_pass):
            yield base_pass


if __name__ == "__main__":
    gen = santa_pwgen(sys.argv[1])
    _ = next(gen)
    print(next(gen))
