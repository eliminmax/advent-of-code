#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2018 Day 1 Part 2

function check_val(val) {
    sum += val
    if (sum in known_sums) {
        print sum
        exit
    }
    known_sums[sum] = 0 # only care about key, not value
}
BEGIN { known_sums[0] = 0 }
# store the value sequence
{vals[i++] = $0}

# loop through the value sequence
END {
    while (1) {
        for (n = 0; n < i; n++) {
            check_val(vals[n])
        }
    }
}
