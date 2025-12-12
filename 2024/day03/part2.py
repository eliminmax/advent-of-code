#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 3 Part 2

import sys
import re

mul_pat = "(?:mul\\((?P<arg0>[0-9]+),(?P<arg1>[0-9]+)\\))"
enable_pat = "(?P<enable>do\\(\\))"
disable_pat = "(?P<disable>don't\\(\\))"
pat = re.compile("|".join((mul_pat, enable_pat, disable_pat)))


def main() -> None:
    total = 0
    enabled = True
    with open(sys.argv[1], "r") as f:
        for match in pat.finditer(f.read()):
            groupdict = match.groupdict()
            if groupdict["enable"]:
                enabled = True
            elif groupdict["disable"]:
                enabled = False
            elif enabled:
                total += int(groupdict["arg0"]) * int(groupdict["arg1"])
    print(total)


if __name__ == "__main__":
    main()
