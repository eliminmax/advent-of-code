#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2023 Day 2 Part 1

import sys
from typing import NamedTuple


class GameInfo(NamedTuple):
    num: int
    r: int
    g: int
    b: int


def parse_game(game_string: str) -> GameInfo:
    num_str, outcome = game_string.split(": ")
    game_num = int(num_str.split(" ")[1])
    max_r = 0
    max_g = 0
    max_b = 0
    for pick in outcome.split("; "):
        for part in pick.split(', '):
            num_str, color = part.split()
            match color:
                case 'red':
                    max_r = max(max_r, int(num_str))
                case 'green':
                    max_g = max(max_g, int(num_str))
                case 'blue':
                    max_b = max(max_b, int(num_str))
    return GameInfo(game_num, max_r, max_g, max_b)


def main():
    with open(sys.argv[1], "r") as f:
        games: list[GameInfo] = [parse_game(line) for line in f]
    possible = filter(lambda a: a.r <= 12 and a.g <= 13 and a.b <= 14, games)
    print(sum(game.num for game in possible))


if __name__ == "__main__":
    main()
