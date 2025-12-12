#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 16 Part 2

# I feel like making it this far into Advent of Code without a formal academic
# background in algorithms like Dijkstra's or A* is quite the achievement. To
# be fair, I have spent lot of time getting hands-on experience writing code
# for various purposes, and have taken classes on programming, particularly for
# system automation, while working towards my Computer Networking and
# Cybersecurity degree, and have been programming for fun since middle school

import sys
from collections import defaultdict
from collections.abc import Iterable
from dataclasses import dataclass
from typing import Optional, NamedTuple, Self
from enum import Enum
from itertools import chain


class Direction(Enum):
    # use even for vertical orientation, and odd for horizontal orientation
    NORTH = 0
    EAST = 1
    SOUTH = 2
    WEST = 3

    def aligns_with(self, other: Self) -> bool:
        return self.value & 1 == other.value & 1


class Location(NamedTuple):
    row: int
    col: int


class Link(NamedTuple):
    dest: Location
    direction: Direction

    def weight(self, from_dir: Direction) -> int:
        if self.direction == from_dir:
            return 1
        if self.direction.aligns_with(from_dir):
            # 2 90-degree turns + 1 move
            return 2001
        # 1 90-degree turn + 1 move
        return 1001


@dataclass
class DijkstraEntry:
    score: int
    paths: list[list[Link]]


class Maze:
    def __init__(self, maze_rows: Iterable[str]):
        locs: set[Location] = set()
        start: Optional[Location] = None
        end: Optional[Location] = None
        rows = 0
        cols: Optional[int] = None
        for row, row_text in enumerate(maze_rows):
            rows += 1
            if cols is None:
                cols = len(row_text)
            elif cols != len(row_text):
                raise ValueError("rows of different lengths")
            for col, col_chr in enumerate(row_text):
                match col_chr:
                    case ".":
                        locs.add(Location(row, col))
                    case "E":
                        locs.add(end := Location(row, col))
                    case "S":
                        locs.add(start := Location(row, col))
                    case "#":
                        pass
                    case i:
                        raise ValueError(f"{repr(i)} is not a valid node")
        if start is None:
            raise ValueError("no start node found")
        if end is None:
            raise ValueError("no end location found")
        if cols is None:
            raise ValueError("no rows found")
        self.rows = rows
        self.cols = cols
        self.locs = locs
        self.start = start
        self.end = end
        self.links_built = False
        self.dijkstra_ran = False

    def build_links(self) -> None:
        if self.links_built:
            return
        # take advantage of the fact that the borders are all closed here, so
        # there's no need to check for overflow
        self.links: defaultdict[Location, list[Link]] = defaultdict(list)
        for loc in self.locs:
            if (loc_b := Location(loc.row + 1, loc.col)) in self.locs:
                self.links[loc].append(Link(loc_b, Direction.SOUTH))
            if (loc_b := Location(loc.row - 1, loc.col)) in self.locs:
                self.links[loc].append(Link(loc_b, Direction.NORTH))
            if (loc_b := Location(loc.row, loc.col + 1)) in self.locs:
                self.links[loc].append(Link(loc_b, Direction.EAST))
            if (loc_b := Location(loc.row, loc.col - 1)) in self.locs:
                self.links[loc].append(Link(loc_b, Direction.WEST))
        self.links_built = True

    def dijkstra(self) -> None:
        # made with reference to pseudocode implementation on datagy.io's
        # "Dijkstra’s Algorithm (Shortest Path) in Python" page, though I did
        # not scroll down to the actual Python implementation the title
        # of the page promised
        # https://datagy.io/dijkstras-algorithm-python/
        if self.dijkstra_ran:
            return

        self.build_links()
        distances: dict[Link, DijkstraEntry] = {}

        start_link = Link(self.start, Direction.EAST)
        start = DijkstraEntry(0, [[start_link]])
        distances[start_link] = start

        priority_queue: list[tuple[int, Link]] = [(0, start_link)]

        while len(priority_queue):
            priority_queue.sort(key=lambda loc: loc[0], reverse=True)
            dist, node = priority_queue.pop()
            if node not in distances:
                continue
            old_dist = distances[node].score
            if dist > old_dist:
                continue
            paths = distances[node].paths

            for link in self.links[node.dest]:
                link_weight = link.weight(node.direction) + dist
                if link not in distances:
                    distances[link] = DijkstraEntry(
                        link_weight,
                        [path[:] + [link] for path in paths],
                    )
                    priority_queue.append((link_weight, link))
                    continue
                old_link_dist = distances[link].score
                if link_weight < old_link_dist:
                    distances[link] = DijkstraEntry(
                        link_weight,
                        [path[:] + [link] for path in paths],
                    )
                    priority_queue.append((link_weight, link))
                elif link_weight == old_link_dist:
                    distances[link].paths += [
                        path[:] + [link] for path in paths
                    ]

        minsc: Optional[DijkstraEntry] = None
        for i in range(4):
            link = Link(self.end, Direction(i))
            if link in distances:
                if minsc is None or distances[link].score < minsc.score:
                    minsc = distances[link]
                elif distances[link].score == minsc.score:
                    minsc.paths += distances[link].paths

        if minsc is None:
            raise ValueError("No path from start to end could be found")

        self.min_scoring_paths = minsc
        self.dijkstra_ran = True

    def visited(self) -> int:
        self.dijkstra()
        return len(
            set(
                i.dest
                for i in chain.from_iterable(self.min_scoring_paths.paths)
            )
        )


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        maze = Maze(line.strip() for line in f)
    maze.dijkstra()
    print(maze.visited())


if __name__ == "__main__":
    main()
