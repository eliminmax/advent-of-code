#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2022 Day 1 Part 2

# set up new elf on blank lines
/^$/ { 
    # update max if needed

    if (sum > top[0]) {
        top[2] = top[1]
        top[1] = top[0]
        top[0] = sum
    } else if (sum > top[1]) {
        top[2] = top[1]
        top[1] = sum
    } else if (sum > top[2]) {
        top[2] = sum
    }
        
    # reset sum on blank lines
    sum = 0
}

# add to sum on non-blank lines
/./ { sum += $0 }

END { print top[0] + top[1] + top[2] }
