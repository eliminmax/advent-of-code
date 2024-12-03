#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 1 Part 1

{
    # remove non-digits
    gsub(/[^0-9]/, "");
    # get only first and last digit
    val = substr($0, 1, 1)substr($0, length, 1)
    printf "%s: %d\n", $0, val
    sum += val
}
END {print sum}
