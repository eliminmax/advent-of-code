#!/usr/bin/env -S jq -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 12 Part 2

def nonred:
    type == "object" and (has("red") or ([.[]] | contains(["red"]))) | not
;

[recurse(.[]?; nonred) | select(. | type == "number")] | add
