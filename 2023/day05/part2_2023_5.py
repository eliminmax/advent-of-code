#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 5 Part 2

import sys
from typing import Self, Optional, Any
from collections.abc import Iterable
from dataclasses import dataclass


@dataclass
class AlmanacRange:
    start: int
    stop: int

    def intersects(self, other: Self) -> bool:
        ret: bool = self.start in other or other.start in self
        return ret

    def __contains__(self, other: Self | int) -> bool:
        if isinstance(other, int):
            return other in range(self.start, self.stop)
        return other.start >= self.start and other.stop <= self.stop

    def __bool__(self) -> bool:
        return self.start < self.stop

    def with_rule(
        self, src: Self, dest: Self
    ) -> tuple[list["AlmanacRange"], Optional["AlmanacRange"]]:
        """1st returned list is still unchanged. 2nd has been changed."""
        offset = dest.start - src.start
        if not self.intersects(src):
            ret: Any = ([self], None)

        elif src in self:
            pre = AlmanacRange(self.start, src.start)
            processed = AlmanacRange(dest.start, dest.stop)
            post = AlmanacRange(src.stop, self.stop)
            leftovers: list["AlmanacRange"] = []
            if pre:
                leftovers.append(pre)
            if post:
                leftovers.append(post)
            ret = (leftovers, processed)

        elif self in src:
            ret = ([], AlmanacRange(self.start + offset, self.stop + offset))

        elif src.start in self:
            cut_size = self.stop - src.start
            pre = AlmanacRange(self.start, src.start)
            processed = AlmanacRange(dest.start, dest.start + cut_size)
            ret = ([pre] if pre else [], processed)

        elif self.start in src:
            cut_size = src.stop - self.start
            processed = AlmanacRange(dest.stop - cut_size, dest.stop)
            post = AlmanacRange(src.stop, self.stop)
            ret = ([post] if post else [], processed)

        else:
            raise ValueError("I'm a doofus")
        return ret


def process_section(
    section_text: str, ranges: list[AlmanacRange]
) -> list[AlmanacRange]:
    section_mappings: Iterable[tuple[AlmanacRange, AlmanacRange]] = (
        (AlmanacRange(src, src + length), AlmanacRange(dest, dest + length))
        for dest, src, length in (
            (int(i) for i in line.split())
            for line in section_text.splitlines()
        )
    )

    processed: list[AlmanacRange] = []
    unprocessed: list[AlmanacRange] = ranges[:]
    for rule in section_mappings:
        unchanged: list[AlmanacRange] = []
        while unprocessed:
            almanac_range = unprocessed.pop()
            start_sz = almanac_range.stop - almanac_range.start
            remaining, newly_processed = almanac_range.with_rule(*rule)
            if newly_processed is None:
                unchanged += remaining
            else:
                if (
                    sum(
                        i.stop - i.start for i in (newly_processed, *remaining)
                    )
                    != start_sz
                ):
                    raise ValueError
                unprocessed += remaining
                processed.append(newly_processed)
        if unchanged:
            unprocessed = unchanged
        else:
            break
    processed += unprocessed
    processed.sort(key=lambda a: (a.start, a.stop))
    return processed


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        sections = f.read().split("\n\n")
    seed_numbers = [int(i) for i in sections[0].split()[1:]]
    ranges: list[AlmanacRange] = [
        AlmanacRange(a, a + b)
        for a, b in zip(seed_numbers[::2], seed_numbers[1::2])
    ]
    for section_text in sections[1:]:
        section_text = section_text.split("\n", 1)[1]  # discard heading
        ranges = process_section(section_text, ranges)
    print(min(r.start for r in ranges))


if __name__ == "__main__":
    main()
