#!/bin/sh

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# generate python script skeleton

if [ "$#" -ne 4 ]; then
    printf 'Unexpected number of arguments: expected 4, not %d!\n' "$#" >&2
    exit 1
fi

scriptname="$1.py"; year="$2"; day="$3"; part="$4"

cat > "$scriptname" <<EOF
#!/usr/bin/env python3

# SPDX-FileCopyrightText: $(date +%Y) $(git config user.name)
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC $year Day $day Part $part

import sys


def main() -> None:
    with open(sys.argv[1], "r") as f:
        pass


if __name__ == "__main__":
    main()
EOF

chmod +x "$scriptname"
