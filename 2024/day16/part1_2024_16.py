#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2024 Day 16 Part 1

# I feel like making it this far into Advent of Code without a formal academic
# background in algorithms like Dijkstra's or A* is quite the achievement. To be
# fair, I have spent lot of time getting hands-on experience writing code for
# various purposes, and have taken classes on programming, particularly for
# system automation, while working towards my Computer Networking and
# Cybersecurity degree, and have been programming for fun since middle school

import sys
from collections import defaultdict
from collections.abc import Iterable
from typing import Optional, NamedTuple, Self
from enum import Enum


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
        links: defaultdict[Location, list[Link]] = defaultdict(list)
        for loc in self.locs:
            if (loc_b := Location(loc.row + 1, loc.col)) in self.locs:
                links[loc].append(Link(loc_b, Direction.SOUTH))
            if (loc_b := Location(loc.row - 1, loc.col)) in self.locs:
                links[loc].append(Link(loc_b, Direction.NORTH))
            if (loc_b := Location(loc.row, loc.col + 1)) in self.locs:
                links[loc].append(Link(loc_b, Direction.EAST))
            if (loc_b := Location(loc.row, loc.col - 1)) in self.locs:
                links[loc].append(Link(loc_b, Direction.WEST))
        self.links = links
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
        distances: defaultdict[Link, Optional[int]] = defaultdict(lambda: None)

        start = (0, Link(self.start, Direction.EAST))
        distances[start[1]] = 0

        priority_queue: list[tuple[int, Link]] = [start]

        while len(priority_queue):
            priority_queue.sort(key=lambda loc: loc[0], reverse=True)
            dist, node = priority_queue.pop()
            old_dist = distances[node]
            if old_dist is not None and dist > old_dist:
                continue

            for link in self.links[node.dest]:
                link_weight = link.weight(node.direction) + dist
                old_link_dist = distances[link]
                if old_link_dist is None or link_weight < old_link_dist:
                    distances[link] = link_weight
                    priority_queue.append((link_weight, link))

        mins: dict[Location, tuple[int, Direction]] = {}
        for link, score in distances.items():
            if score is None:
                continue
            if link.dest not in mins or score < mins[link.dest][0]:
                mins[link.dest] = (score, link.direction)
        self.min_scores = mins
        self.dijkstra_ran = True

    def min_score(self, loc: Optional[Location] = None) -> int:
        self.dijkstra()
        if loc is None:
            loc = self.end
        if loc not in self.min_scores:
            raise ValueError(f"{loc} was not scored within this maze")
        return self.min_scores[loc][0]


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        maze = Maze(line.strip() for line in f)
    maze.dijkstra()
    print(maze.min_score())


if __name__ == "__main__":
    main()
