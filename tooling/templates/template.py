#!/usr/bin/env python3

# --solution-comment--

from collections.abc import Iterable
from sys import argv


def main(input_lines: Iterable[str]):
    ...


if __name__ == "__main__":
    with open(argv[1] if argv[1:] else "input", "r") as f:
        print(main(f))
