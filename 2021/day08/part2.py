#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2021 Day 8 Part 2

import sys


def deduce_rules(signals: list[str]) -> dict[str, str]:
    signals = sorted(signals, key=lambda s: len(s))
    mappings: dict[int, str] = {
        1: signals[0],  # shortest element is 1
        7: signals[1],  # next shortest element is 7
        4: signals[2],  # after that is 4
        8: signals[9],  # the only one with all signals active is eight.
    }

    # Now, deduce the more complex ones
    # set of signals used in the digit 1
    right_sigs = set(mappings[1])
    # set of signals used in the digit 4 but not 1
    mid_and_top_left: set[str] = set(mappings[4]).difference(right_sigs)

    for sig in signals[6:9]:  # six active signals
        # the only 6-signal digit which isn't a superset of the signals used
        # for 4 but not 1 is 0
        if mid_and_top_left.difference(set(sig)):
            mappings[0] = sig
        # of the remaining 2 possibilities, 1 is a strict subset of 9, not 6
        elif right_sigs.difference(set(sig)):
            mappings[6] = sig
        else:
            mappings[9] = sig

    # the overlap between signals in 6 and signals in 9 are signals in 5
    set_5_sigs = set(mappings[9]).intersection(set(mappings[6]))

    for sig in signals[3:6]:  # five active signals
        if set(sig) == set_5_sigs:
            mappings[5] = sig
        # of the 2 remaining possibilities, 1 is a strict subset of 3, not 2
        elif right_sigs.difference(set(sig)):
            mappings[2] = sig
        else:
            mappings[3] = sig

    # Now that the mappings are known, sort and reverse them before returning
    # convert values to string because they'll need to be concatenated as
    # digits later on
    return {"".join(sorted(v)): str(k) for k, v in mappings.items()}


def displayed_value(display_signals: list[str], rules: dict[str, str]) -> int:
    disp_str = ""
    for sig in display_signals:
        disp_str += rules["".join(sorted(sig))]
    return int(disp_str)


def main() -> None:
    total = 0
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            rule_strs, display_sigs = (seg.split() for seg in line.split("|"))
            rules = deduce_rules(rule_strs)
            total += displayed_value(display_sigs, rules)
    print(total)


if __name__ == "__main__":
    main()
