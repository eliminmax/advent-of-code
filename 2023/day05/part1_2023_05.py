#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 5 Part 1

import sys
from typing import Optional, TypeAlias, cast

TriInt: TypeAlias = tuple[int, int, int]


class SectionMappings:
    def __init__(self, section_text: str):
        self.section_mappings: list[TriInt] = []
        for line in (line for line in section_text.split("\n") if line):
            self.section_mappings.append(
                cast(TriInt, tuple(int(i) for i in line.split()))
            )

    def __getitem__(self, k: int) -> int:
        for dst_start, src_start, range_len in self.section_mappings:
            if (
                v := SectionMappings._mapped_val(
                    dst_start, src_start, range_len, k
                )
            ) is not None:
                return v
        return k

    @staticmethod
    def _mapped_val(
        dst_start: int, src_start: int, range_len: int, val: int
    ) -> Optional[int]:
        if val >= src_start and val < src_start + range_len:
            return val - src_start + dst_start
        return None


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        sections = f.read().split("\n\n")
    seeds: list[int] = [int(i) for i in sections[0].split()[1:]]
    for section_text in sections[1:]:
        section_mappings = SectionMappings(section_text.split("\n", 1)[1])
        seeds = [section_mappings[seed] for seed in seeds]
    print(min(seeds))


if __name__ == "__main__":
    main()
