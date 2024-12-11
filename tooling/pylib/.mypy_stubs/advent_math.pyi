from typing import NamedTuple, Self

class Point(NamedTuple):
    row: int
    col: int
    def manhattan_distance(self, other: Self) -> int: ...
