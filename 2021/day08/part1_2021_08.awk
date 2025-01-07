#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2021 Day 8 Part 1


function check_match(word) {
    # a 1 takes 2 signals
    # a 4 takes 4 signals
    # a 7 takes 3 signals
    # an 8 takes 7 signals
    w = ":"word":"
    if (w ~ /:[a-g]{2}:/ || w ~ /:[a-g]{4}:/ ||
        w ~ /:[a-g]{3}:/ || w ~ /:[a-g]{7}:/) {
        total++
    }
}

{
    sub(/.*[|]/, "")
    check_match($1)
    check_match($2)
    check_match($3)
    check_match($4)
}

END { print total }
