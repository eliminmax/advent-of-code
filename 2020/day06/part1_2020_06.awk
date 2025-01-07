#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2020 Day 6 Part 1

function process_group() {
    for (k in answers) {
        total += answers[k]
        delete answers[k]
    }
}

/^$/ {process_group(); next}

{
    split($0, chars, "")
    for (i in chars) answers[chars[i]] = 1
}

END {
    process_group()
    print total
}
