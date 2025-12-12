#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2025 Day 12 Part 1

/^[0-9]+x[0-9]+: / {
    a = $1
    b = $1
    sub(/x[0-9]+:/, "", a)
    sub(/[0-9]+x/, "", b)
    sub(/:/, "", b)
    size = 9 * ($2 + $3 + $4 + $5 + $6 + $7)
    if (size <= (a * b)) total++
}

END { print total }
