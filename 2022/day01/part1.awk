#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 1 Part 1

# set up new elf on blank lines
/^$/ { 
    # update max if needed

    if (sum > max) {
        max = sum
    }

    # reset sum on blank lines
    sum = 0
}

# add to sum on non-blank lines
/./ { sum += $0 }

END { print max }