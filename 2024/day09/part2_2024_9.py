#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 9 Part 2

import sys
from itertools import count, pairwise
from collections.abc import Iterable
from typing import Optional, NamedTuple
from dataclasses import dataclass


class _FileListing(NamedTuple):
    start: int
    stop: int


@dataclass
class DiskMap:
    _size: int
    _file_table: dict[int, _FileListing]
    _order: list[int]

    def checksum(self) -> int:
        flat: list[Optional[int]] = [None] * self._size
        for file_id, (start, stop) in self._file_table.items():
            flat[start:stop] = [file_id] * (stop - start)
        return sum(i * b for i, b in enumerate(flat) if b is not None)

    def defrag(self) -> None:
        move_order = self._order[::-1]
        for i in move_order:
            self._defrag_pass(i)

    def _defrag_pass(self, fid: int) -> None:
        # For some reason, this breaks if it's used as the loop body in defrag
        # but works when moved into its own function.
        max_index = self._order.index(fid)
        current_f = self._file_table[fid]
        current_sz = current_f.stop - current_f.start
        possible_points = enumerate(
            pairwise(self._order[:max_index]),
            start=1,  # 0 would be before the start
        )
        for i, (p, n) in possible_points:
            prev_f = self._file_table[p]
            next_f = self._file_table[n]
            if current_sz <= next_f.start - prev_f.stop:
                self._file_table[fid] = _FileListing(
                    start=prev_f.stop,
                    stop=(prev_f.stop + current_sz),
                )
                self._order.remove(fid)
                self._order.insert(i, fid)
                return

        # if this is reached, there's no space between previous entries, but
        # there still may be space between the previous entry and this one
        if not self._order[max_index + 1 :]:
            prev_stop = self._file_table[self._order[max_index - 1]].stop
            if current_f.start - prev_stop >= current_sz:
                self._file_table[fid] = _FileListing(
                    prev_stop, prev_stop + current_sz
                )

    @staticmethod
    def parse_blocks(block_ints: Iterable[int]) -> "DiskMap":
        id_gen = count(0)
        size: int = 0
        table: dict[int, _FileListing] = {}
        for index, length in enumerate(block_ints):
            if index % 2 == 0:
                file_id = next(id_gen)
                table[file_id] = _FileListing(size, size + length)
            size += length
        return DiskMap(_size=size, _file_table=table, _order=sorted(table))


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        disk: DiskMap = DiskMap.parse_blocks(int(c) for c in f.read())
    disk.defrag()
    print(disk.checksum())


if __name__ == "__main__":
    main()
