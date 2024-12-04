#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 1 Part 1

{
    # replace spelled-out digits with numerals
    # need to make sure that overlapping letters are handled right
    # (e.g. twoneight should be 218, not 28).
    # Luckily, no overlap is more than one character long
    gsub(/zero/, "z0o")
    gsub(/one/, "o1e")
    gsub(/two/, "t2o")
    gsub(/three/, "t3e")
    gsub(/four/, "f4r")
    gsub(/five/, "f5e")
    gsub(/six/, "s6x")
    gsub(/seven/, "s7n")
    gsub(/eight/, "e8t")
    gsub(/nine/, "n9e")
    # remove non-digits
    gsub(/[^0-9]/, "");
    # get only first and last digit
    val = substr($0, 1, 1)substr($0, length, 1)
    sum += val
}
END {print sum}
