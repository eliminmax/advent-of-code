#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2019 Day 1 Part 2

function fuel_for_mass(n) {
    next_step = fuel_for_module = int(n / 3) - 2
    if (next_step <= 0) {
        return 0
    } else {
        return next_step + fuel_for_mass(next_step)
    }
}

{ sum += fuel_for_mass($0) }

END { print sum }
