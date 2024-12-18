#!/bin/sh

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# generate part2 file from part1
set -e

# set year and day arguments by parsing current directory name
eval "$(pwd | sed 's#.*/\([0-9]\{4\}\)/day0\?\([1-9]\?[0-9]\)$#year=\1 day=\2#')"

c_year="$(date +%Y)"
c_name="$(git config user.name)"

case "${1:-rust}" in
    rs|rust) extension="rs" ;;
    awk) extension="awk" ;;
    c) extension="c" ;;
    py|python) extension="py" ;;
esac

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

reuse annotate -l 0BSD -y "$c_year" -c "$c_name" "$outname" --merge-copyrights

if [ "$(head -c2 "$outname")" = '#!' ]; then
    chmod +x "$outname"
fi
