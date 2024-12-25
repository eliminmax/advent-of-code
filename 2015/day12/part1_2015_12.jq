#!/usr/bin/env -S jq -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 12 Part 1

[recurse | select(. | type == "number")] | add
