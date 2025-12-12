#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 6 Part 2

function process_group() {
    for (k in answers) {
        if (answers[k] == group_size) total++
        delete answers[k]
    }
    group_size = 0
}

/^$/ {process_group(); next}

{
    group_size++
    split($0, chars, "")
    for (i in chars) answers[chars[i]]++
}

END {
    process_group()
    print total
}
