#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD
import re

lines: list[str] = []

while True:
    try:
        lines.append(input())
    except EOFError:
        break

# need to know which labels to keep.
used_labels: set[str] = set()
for line in lines:
    for match in re.findall(r"L\d\d", line[1:]):
        used_labels.add(match)

# print nonlabel lines, and labels that are actually used.
for line in lines:
    if line[0] != "L" or line[:-1] in used_labels:
        print(line)
