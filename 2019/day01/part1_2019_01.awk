#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2019 Day 1 Part 1

{ sum += (int($0 / 3) - 2); }

END { print sum }
