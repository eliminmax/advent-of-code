#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 7 Part 2

import sys
from dataclasses import dataclass, field as field
from typing import Self, cast
from statistics import mode


@dataclass
class Program:
    pid: str
    weight: int
    holding: list[str]
    total_weight: int | None = field(repr=False, init=False, default=None)
    held_by: str | None = field(repr=False, init=False, default=None)


class ProgramManager:
    def __init__(self: "Self"):
        self._programs: dict[str, Program] = {}
        self._unresolved_holds: dict[str, str] = {}
        self.bottom: str | None = None
        self._resolved = False

    def parse_program(self, s: str) -> None:
        if "\n" in s[:-1]:
            raise ValueError("Can't parse a multi-line string")
        words = s.split()
        pid = words[0]
        weight = int(words[1][1:-1])  # first and last character are parens
        holding = [s.replace(",", "") for s in words[3:]]

        self._programs[pid] = Program(pid, weight, holding)

        for held in holding:
            if held in self._programs:
                if self._programs[held].held_by is not None:
                    raise ValueError(f"Multiple programs holding {held}")
                self._programs[held].held_by = pid
            elif held in self._unresolved_holds:
                raise ValueError(f"Multiple programs holding {held}")
            else:
                self._unresolved_holds[held] = pid

        if pid in self._unresolved_holds:
            self._programs[pid].held_by = self._unresolved_holds.pop(pid)
            assert pid not in self._unresolved_holds

    def find_bottom(self) -> str:
        if self.bottom is not None:
            return cast(str, self.bottom)
        if self._unresolved_holds:
            raise ValueError(
                f"Unresolved references to programs: {self._unresolved_holds}"
            )
        bottom_programs = [
            p.pid for p in self._programs.values() if p.held_by is None
        ]
        if len(bottom_programs) > 1:
            raise ValueError(f"Multiple unheld programs: {bottom_programs}")
        self.bottom = bottom_programs[0]
        return cast(str, self.bottom)

    def _resolve_weight(self, prog_name: str) -> int:
        prog = self._programs[prog_name]
        if prog.total_weight is None:
            prog.total_weight = prog.weight + sum(
                self._resolve_weight(p) for p in prog.holding
            )
        return cast(int, prog.total_weight)

    def resolve_weights(self) -> None:
        self._resolve_weight(self.find_bottom())
        self._resolved = True

    def corrected_weight(self) -> int:
        def find_unbalance(sub_branch: str) -> str:
            """Find the highest unbalanced node on the tree"""
            weights = [
                self._programs[p].total_weight
                for p in self._programs[sub_branch].holding
            ]
            if len(set(weights)) <= 1:
                return sub_branch

            # this should ensure that the outlier is moved to the front
            by_rarity = sorted(weights[:], key=lambda w: weights.count(w))
            assert weights.count(by_rarity[0]) == 1

            outlier = next(
                prog
                for prog in self._programs[sub_branch].holding
                if self._programs[prog].total_weight == by_rarity[0]
            )
            # keep following the outlier up the tree
            return find_unbalance(outlier)

        if not self._resolved:
            self.resolve_weights()
        unbalance_start = find_unbalance(cast(str, self.bottom))
        unbalanced_list: list[int] = next(
            [cast(int, self._programs[p].total_weight) for p in v.holding]
            for v in self._programs.values()
            if unbalance_start in v.holding
        )
        return mode(unbalanced_list) - sum(
            cast(int, self._programs[p].total_weight)
            for p in self._programs[unbalance_start].holding
        )


def main() -> None:
    mgr = ProgramManager()
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            mgr.parse_program(line)
    mgr.resolve_weights()
    print(mgr.corrected_weight())


if __name__ == "__main__":
    main()
