#!/bin/sh

# SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# generate source file from template
set -e

template_dir="$(dirname "$(realpath "$0")")/../templates"

# set year and day arguments by parsing current directory name
eval "$(pwd | sed 's#.*/\([0-9]\{4\}\)/day0\?\([12]\?[0-9]\)$#year=\1 day=\2#')"

c_year="$(date +%Y)"
c_name="$(git config user.name)"

# shellcheck disable=SC2154 # variables set by eval "$(pwd | sed ...)"
get_input.sh "$year" "$day"

case "${1:-rust}" in
    rs|rust) extension="rs" ;;
    awk) extension="awk" ;;
    c) extension="c" ;;
    py|python) extension="py" ;;
    jq) extension="jq" ;;
esac

# shellcheck disable=SC2154 # variables set by eval "$(pwd | sed ...)"
outname="$(printf 'part1_%04d_%02d.%s' "$year" "$day" "$extension")"
if [ -e "$outname" ]; then
    printf 'Refusing to clobber existing file %s!\n' "$outname" >&2
    exit 1
fi

template_file="$template_dir/template.$extension"
if ! [ -e "$template_file" ]; then
    printf 'File %s does not exist!\n' "$template_file" >&2
    exit 1
fi

solution_comment="$(
    # shellcheck disable=SC2154 # variables set by eval "$(pwd | sed ...)"
    printf 'Solution to AoC %04d Day %02d Part 1' "$year" "$day"
)"

sed "s/--solution-comment--/$solution_comment/" \
    "$template_file" > "$outname"

if [ "$extension" = 'jq' ]; then
    reuse annotate --style python -l 0BSD -y "$c_year" -c "$c_name" "$outname"
else
    reuse annotate -l 0BSD -y "$c_year" -c "$c_name" "$outname"
fi

# mark scripts as exectutable, and add appropriate gitignore for
# compiled languages
if [ "$(head -c2 "$outname")" = '#!' ]; then
    chmod +x "$outname"
else
    printf '*\n!.gitignore\n!*.%s\n' "$extension" > .gitignore
    reuse annotate -l 0BSD -y "$c_year" -c "$c_name" .gitignore
fi

if [ "$extension" = 'rs' ]; then
    cargo_main="$HOME/Desktop/aoc-rs/src/main.rs"
    if ! [ -e "$cargo_main" ]; then
        printf 'Note: missing out-of-tree workspace crate.\n' >&2
    else
        # This is messy and ugly. It uses awk to extract variables from the
        # solution comment and turn them into shell variable assignments.
        # Specifically, it sets year, day, and part to the appropriate numbers,
        # and day_padded to the day 0-padded to be a 2-digit number.
        #
        # If it matches zero or more than 1, it emits a `printf` command at the
        # end to warn the user, followed by `exit 1`, so nothing past this
        # will run on error
        eval "$(parse_sol_comment "$cargo_main")"
        # shellcheck disable=SC2154 # variables set by parse_sol_comment
        expected_file="part${part}_${year}_$day_padded.rs"
        expected_file="$AOC_GIT_DIR/$year/day$day_padded/$expected_file"
        if [ ! -e "$expected_file" ]; then
            printf '%s has not been copied over to %s - not clobbering\n' >&2 \
                "$cargo_main" \
                "$expected_file"
            exit 1
        elif ! diff -q "$cargo_main" "$expected_file"; then
            printf '%s differs from %s - not clobbering.\n' >&2 \
                "$cargo_main" \
                "$expected_file"
            exit 1
        fi
        cp input "$HOME/Desktop/aoc-rs/input"
        # clear out any old sample inputs
        find "$HOME/Desktop/aoc-rs" -name 'sample_input*' -exec rm -i {} +
        (cd "$HOME/Desktop/aoc-rs" && cargo clean --quiet)
        cp "$outname" "$cargo_main"
    fi
fi
