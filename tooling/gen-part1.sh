#!/bin/sh

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# generate source file from template
set -e

template_dir="$(dirname "$(realpath "$0")")"

# set year and day arguments by parsing current directory name
eval "$(pwd | sed 's#.*/\([0-9]\{4\}\)/day\([0-9][0-9]\)$#year=\1 day=\2#')"

c_year="$(date +%Y)"
c_name="$(git config user.name)"

case "${1:-python}" in
    rs|rust) extension="rs" ;;
    awk) extension="awk" ;;
    py|python) extension="py" ;;
esac

outname="part1_${year}_${day}.${extension}"
if [ -e "$outname" ]; then
    printf 'Refusing to clobber existing file %s!\n' "$outname" >&2
    exit 1
fi

template_file="$template_dir/template.$extension"
if ! [ -e "$template_file" ]; then
    printf 'File %f does not exist!\n' "$template_file" >&2
    exit 1
fi

sed "s/--solution-comment--/Solution to AoC $year Day $day Part 1/" \
    "$template_file" > "$outname"

reuse annotate -l 0BSD -y "$c_year" -c "$c_name" "$outname"

if [ "$(head -c2 "$outname")" = '#!' ]; then
    chmod +x "$outname"
fi