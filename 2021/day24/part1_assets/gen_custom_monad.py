#!/usr/bin/env python3
"""
Generate C source that works like the direct translation, but with more of an
understanding of the actual structure of the ALU

SPDX-FileCopyrightText: 2025 Eli Array Minkoff

SPDX-License-Identifier: 0BSD
"""

from sys import argv

with open(argv[1] if argv[1:] else "input") as f:
    lines = f.readlines()

block_nums: list[tuple[str, str, str]] = [
    (
        lines[i + 4].split()[2],
        lines[i + 5].split()[2],
        lines[i + 15].split()[2],
    )
    for i in range(0, len(lines), 18)
]

block_fn_template = """int64_t block{i}(char d, int64_t z) {{
    if ((z % 26) + {p1} == d) {{
        return z / {p0};
    }} else {{
        return (z / {p0}) * 26 + d + {p2};
    }}
}}
"""

block_fns = "\n".join(
    block_fn_template.format(i=i, p0=p0, p1=p1, p2=p2)
    for i, (p0, p1, p2) in enumerate(block_nums)
)

calls = "\n".join(f"        z = block{i}(digits[{i}], z);" for i in range(14))

print(
    f"""
#include <stdint.h>
#include "block_cache.h"

{block_fns}

int64_t custom_monad(char digits[14]) {{
    int64_t z = 0;
{calls}
    return z;
}}
"""
)
