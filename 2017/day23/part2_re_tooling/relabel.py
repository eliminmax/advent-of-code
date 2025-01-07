#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD
lines: list[str] = []

while True:
    try:
        lines.append(input())
    except EOFError:
        break

# need to know which labels to keep.
used_labels: set[str] = set()
for line in lines:
    words = line.split()
    if words[0] == "if":
        used_labels.add(words[-1][:-1])  # exclude trailing semicolon

# print nonlabel lines, and labels that are actually used.
for line in lines:
    if line[0] != "L" or line[:-1] in used_labels:
        print(line)
