#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 5 Part 1

# Rewrote in Python because part 2 is basically impossible w/o capturing groups

/[aeiou].*[aeiou].*[aeiou]/ && !(/ab/ || /cd/ || /pq/ || /xy/) {
    # need to do this because AWK doesn't have capturing groups
    split($0, line, "")
    for (i = 1; i < length($0); i++) {
        if (line[i] == line[i+1]) {
            nice++
            next
        }
    }
}

END {print nice}
