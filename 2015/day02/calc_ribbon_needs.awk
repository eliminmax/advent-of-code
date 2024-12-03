#!/usr/bin/env -S awk -F x -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2015 Day 2 Part 2

function ribbon_needs(a, b, c) {
    if (a > b && a > c) base_needs = 2 * (b + c)
    else if (b > c) base_needs = 2 * (a + c)
    else base_needs = 2 * (a + b)
    return (a * b * c) + base_needs
}

{
    needed_ribbon += ribbon_needs($1, $2, $3)
}
END { printf "%d\n", needed_ribbon }
