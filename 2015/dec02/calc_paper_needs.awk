#!/usr/bin/env -S awk -F x -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 2 Part 1

function min_size(a, b, c) {
    if (a < b && a < c) return a
    if (b < c) return b
    return c
}

{
    w = $1 * $2
    l = $2 * $3
    h = $3 * $1
    needed_paper += ((w + l + h) * 2) + min_size(w, l, h)
}
END { printf "%d\n", needed_paper }
