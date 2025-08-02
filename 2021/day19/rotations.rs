// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD


//! Generated with the following python script, which uses Dijkstra's algorithm alongside sympy and
//! the 3d rotation matrices from https://stackoverflow.com/a/14609567/25639450
//! ```python
//! from typing import Callable, NamedTuple
//! from collections.abc import Sequence
//! from sympy import symbols, pi, cos, sin
//! from sympy.core.symbol import Symbol
//! 
//! 
//! class Rotation(NamedTuple):
//!     x: Symbol
//!     y: Symbol
//!     z: Symbol
//! 
//! 
//! COS = cos(pi / 2)
//! SIN = sin(pi / 2)
//! 
//! rotators: Sequence[Callable[[Rotation], Rotation]] = (
//!     lambda p: Rotation(p.x, p.y * COS - p.z * SIN, p.y * SIN + p.z * COS),
//!     lambda p: Rotation(p.x * COS + p.z * SIN, p.y, -p.x * SIN + p.z * COS),
//!     lambda p: Rotation(p.x, p.y * COS - p.z * SIN, p.y * SIN + p.z * COS)
//! )
//! 
//! 
//! X, Y, Z = symbols("x y z")
//! 
//! 
//! distances: dict[Rotation, int] = {Rotation(X, Y, Z): 0}
//! queue: list[tuple[int, Rotation]] = [(0, Rotation(X, Y, Z))]
//! 
//! while queue:
//!     cost, rotation = queue.pop()
//!     if rotation in distances and distances[rotation] < cost:
//!         continue
//!     reachable = [rot(rotation) for rot in rotators]
//!     next_cost = cost + 1
//! 
//!     for next_rot in reachable:
//!         if next_rot not in distances or distances[next_rot] > next_cost:
//!             queue.append((next_cost, next_rot))
//!             distances[next_rot] = next_cost
//!     queue.sort(key=lambda e: e[0], reverse=True)
//! 
//! print("pub(super) const ROTATORS: [fn(Position) -> Position; 24] = [")
//! for x, y, z in distances.keys():
//!     c = "|Position { x, y, z }| Position { " + f"x: {x}, y: {y}, z: {z}" + " }"
//!     for v in "xyz":
//!         c = c.replace(f"{v}: {v}", f"{v}")
//!     print(f"    {c},")
//! print("];")
//! ```

use super::Position;

#[rustfmt::skip]
pub(super) const ROTATORS: [fn(Position) -> Position; 24] = [
    |Position { x, y, z }| Position { x, y, z },
    |Position { x, y, z }| Position { x, y: -z, z: y },
    |Position { x, y, z }| Position { x: z, y, z: -x },
    |Position { x, y, z }| Position { x: z, y: x, z: y },
    |Position { x, y, z }| Position { x: -x, y, z: -z },
    |Position { x, y, z }| Position { x, y: -y, z: -z },
    |Position { x, y, z }| Position { x: y, y: -z, z: -x },
    |Position { x, y, z }| Position { x: y, y: x, z: -z },
    |Position { x, y, z }| Position { x: -x, y: -z, z: -y },
    |Position { x, y, z }| Position { x, y: z, z: -y },
    |Position { x, y, z }| Position { x: -z, y: -y, z: -x },
    |Position { x, y, z }| Position { x: -x, y: z, z: y },
    |Position { x, y, z }| Position { x: -z, y, z: x },
    |Position { x, y, z }| Position { x: z, y: -y, z: x },
    |Position { x, y, z }| Position { x: z, y: -x, z: -y },
    |Position { x, y, z }| Position { x: -z, y: -x, z: y },
    |Position { x, y, z }| Position { x: -x, y: -y, z },
    |Position { x, y, z }| Position { x: y, y: z, z: x },
    |Position { x, y, z }| Position { x: -z, y: x, z: -y },
    |Position { x, y, z }| Position { x: -y, y: z, z: -x },
    |Position { x, y, z }| Position { x: -y, y: -z, z: x },
    |Position { x, y, z }| Position { x: -y, y: -x, z: -z },
    |Position { x, y, z }| Position { x: -y, y: x, z },
    |Position { x, y, z }| Position { x: y, y: -x, z },
];
