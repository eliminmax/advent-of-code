#!/bin/sh
set -e

year="$1"
day="$2"
# double realpath to resolve relative path
aoc_dir="$(realpath "$(dirname "$(realpath "$0")")/../..")"
cookie="session=$(cat "$aoc_dir/.cookie")"

if [ -z "$year" ] || [ -z "$day" ]; then
    printf 'usage: %s year day\n' >&2
    exit 2
fi

target_path="$(printf '%s/%04d/day%02d/input' "$aoc_dir" "$year" "$day")"

if [ -e "$target_path" ]; then
    printf '%s already exists.\n' "$target_path" >&2
    exit 0
fi

url="$(printf 'https://adventofcode.com/%04d/day/%d/input' "$year" "$day")"

curl --output "$target_path" --cookie "$cookie" "$url"
