# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD
# hard-code direct links and programatically fill in the blanks
from dataclasses import dataclass
from itertools import product

_NUMPAD_LINKS = {
    "A": {"0": {"<"}, "3": {"^"}},
    "0": {"2": {"^"}, "A": {">"}},
    "1": {"2": {">"}, "4": {"^"}},
    "2": {"0": {"v"}, "1": {"<"}, "3": {">"}, "5": {"^"}},
    "3": {"A": {"v"}, "2": {"<"}, "6": {"^"}},
    "4": {"1": {"v"}, "5": {">"}, "7": {"^"}},
    "5": {"2": {"v"}, "4": {"<"}, "6": {">"}, "8": {"^"}},
    "6": {"3": {"v"}, "5": {"<"}, "9": {"^"}},
    "7": {"4": {"v"}, "8": {">"}},
    "8": {"5": {"v"}, "7": {"<"}, "9": {">"}},
    "9": {"6": {"v"}, "8": {"<"}},
}
_CONTROL_LINKS = {
    "^": {"v": {"v"}, "A": {">"}},
    "A": {"^": {"<"}, ">": {"v"}},
    "<": {"v": {">"}},
    "v": {"<": {"<"}, "^": {"^"}, ">": {">"}},
    ">": {"v": {"<"}, "A": {"^"}},
}


def _build_links(start_links: dict[str, dict[str, set[str]]]):
    """modify start_links to add indirect links"""

    @dataclass
    class RouteGroup:
        score: int
        paths: set[str]

    def join_routes(src_paths: set[str], link_routes: set[str]) -> set[str]:
        return {a + b for a, b in product(src_paths, link_routes)}

    indirect_links: dict[str, dict[str, set[str]]] = {}
    for start_link in start_links:
        # a less overengineered adaptation of my day 16 part 2 dijkstra solver
        distances: dict[str, RouteGroup] = {start_link: RouteGroup(0, {""})}
        queue: list[tuple[int, str]] = [(0, start_link)]
        while queue:
            queue.sort(reverse=True)
            dist, node = queue.pop()
            assert node in distances
            if dist > distances[node].score:
                continue

            paths = distances[node].paths

            for link, link_paths in start_links[node].items():
                weight = dist + 1
                if link not in distances or weight < distances[link].score:
                    distances[link] = RouteGroup(
                        weight, join_routes(paths, start_links[node][link])
                    )
                    queue.append((weight, link))
                elif distances[link].score == weight:
                    distances[link].paths |= join_routes(
                        paths, start_links[node][link]
                    )

        indirect_links[start_link] = {k: v.paths for k, v in distances.items()}
    for start, links in start_links.items():
        links |= indirect_links[start]


_build_links(_CONTROL_LINKS)
_build_links(_NUMPAD_LINKS)
del _build_links

# merge the lists for easier implementation
# pop "A" out of _CONTROL_LINKS to avoid overwriting _NUMPAD_LINKS["A"]
_control_a = _CONTROL_LINKS.pop("A")
BUTTON_LINKS = _NUMPAD_LINKS | _CONTROL_LINKS
BUTTON_LINKS["A"] |= _control_a
