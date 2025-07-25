#!/bin/sh

# SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# generate part2 file from part1
set -e

# set year and day arguments by parsing current directory name
eval "$(pwd | sed 's#.*/\([0-9]\{4\}\)/day\([0-2]\?[0-9]\)$#year=\1 day=\2#')"

c_year="$(date +%Y)"
c_name="$(git config user.name)"

case "${1:-rust}" in
    rs|rust) extension="rs" ;;
    awk) extension="awk" ;;
    c) extension="c" ;;
    py|python) extension="py" ;;
    jq) extension="jq" ;;
esac

# shellcheck disable=SC2154 # variables set by eval "$(pwd | sed ...)"
part1_name="part1_${year}_${day}.${extension}"
if ! [ -e "$part1_name" ]; then
    printf 'File %s does not exist!\n' "$part1_name" >&2
    exit 1
fi

outname="part2_${year}_${day}.${extension}"
if [ -e "$outname" ]; then
    printf 'Refusing to clobber existing file %s!\n' "$outname" >&2
    exit 1
fi

sed "/Solution to AoC/s/Part 1/Part 2/" "$part1_name" > "$outname"

if [ "$extension" = 'jq' ]; then
    reuse annotate --style python -l 0BSD -y "$c_year" -c "$c_name" "$outname"
else
    reuse annotate -l 0BSD -y "$c_year" -c "$c_name" "$outname"
fi

if [ "$(head -c2 "$outname")" = '#!' ]; then
    chmod +x "$outname"
fi

if [ "$extension" = 'rs' ]; then
    cargo_main="$HOME/Desktop/aoc-rs/src/main.rs"
    if [ -e "$cargo_main" ]; then
        # Because of `set -e`, if there's a difference, it will stop here.
        #
        # Because diff -q still reports when files differ, printing an error is
        # redundant.
        diff -q "$cargo_main" "$part1_name"
        cp "$outname" "$cargo_main"
    fi
fi
