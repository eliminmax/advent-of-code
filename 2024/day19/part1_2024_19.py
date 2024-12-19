#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 19 Part 1

# NOTE: this solution shells out to ripgrep for its much faster regex engine
# https://github.com/BurntSushi/ripgrep

import sys
from subprocess import run


def main() -> None:
    infile = sys.argv[1] if sys.argv[1:] else "input"

    with open(infile, "r") as f:
        towels_pat = f"^({'|'.join(next(f).replace(',', '').split())})+$"
        run(["rg", "--count", towels_pat, infile])


if __name__ == "__main__":
    main()
