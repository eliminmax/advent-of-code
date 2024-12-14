#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2017 Day 7 Part 1

import sys
from dataclasses import dataclass
from typing import Self


@dataclass
class Program:
    pid: str
    weight: int
    held_by: str | None = None


class ProgramManager:
    def __init__(self: Self):
        self._programs: dict[str, Program] = {}
        self._unresolved: dict[str, str] = {}

    def parse_program(self, s: str) -> None:
        if "\n" in s[:-1]:
            raise ValueError("Can't parse a multi-line string")
        words = s.split()
        pid = words[0]
        weight = int(words[1][1:-1])  # first and last character are parens

        self._programs[pid] = Program(pid, weight)

        for held in words[3:]:
            held = held.replace(",", "")
            if held in self._programs:
                if self._programs[held].held_by is not None:
                    raise ValueError(f"Multiple programs holding {held}")
                self._programs[held].held_by = pid
            elif held in self._unresolved:
                raise ValueError(f"Multiple programs holding {held}")
            else:
                self._unresolved[held] = pid

        if pid in self._unresolved:
            self._programs[pid].held_by = self._unresolved.pop(pid)

    def find_bottom(self) -> str:
        if self._unresolved:
            raise ValueError(
                f"Unresolved references to programs: {self._unresolved}"
            )
        bottom_programs = [
            p.pid for p in self._programs.values() if p.held_by is None
        ]
        if len(bottom_programs) > 1:
            raise ValueError(f"Multiple unheld programs: {bottom_programs}")
        return bottom_programs[0]

    def __repr__(self) -> str:
        return (
            f"ProgramManager(_programs={self._programs}, "
            + f"_unresovled={self._unresolved}"
        )


def main() -> None:
    mgr = ProgramManager()
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        for line in f:
            mgr.parse_program(line)
    print(mgr.find_bottom())


if __name__ == "__main__":
    main()
