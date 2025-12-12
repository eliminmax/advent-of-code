#!/bin/sh

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

set -e

year="$1"
day="$2"
# double realpath to resolve relative path
aoc_dir="$(realpath "$(dirname "$(realpath "$0")")/../..")"
cookie="session=$(cat "$aoc_dir/.cookie")"

if [ -z "$year" ] || [ -z "$day" ]; then
    printf 'usage: %s year day\n' "$0" >&2
    exit 2
fi

target_path="$(printf '%s/%04d/day%02d/input' "$aoc_dir" "$year" "$day")"

if [ -e "$target_path" ]; then exit 0; fi

url="$(printf 'https://adventofcode.com/%04d/day/%d/input' "$year" "$day")"

email="$(git config get user.email)"
if [ -z "$email" ]; then
    printf 'Could not get email from git config.\n' >&2
    printf 'Must include contact information in request header.\n' >&2
    printf 'See https://reddit.com/r/adventofcode/wiki/faqs/automation\n' >&2
    exit 1
fi
user="$(git config get user.name)"

user_id="${user}${user:+ }<$email>"
ua="get_input.sh from github.com/eliminmax/advent-of-code, user: $user_id"

curl --user-agent "$ua" --output "$target_path" --cookie "$cookie" "$url"
