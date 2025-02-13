#!/usr/bin/env python3

# SPDX-FileCopyrightText: 2024 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Solution to AoC 2018 Day 4 Part 2

import sys
import re
from typing import TypedDict, NamedTuple, Optional
from enum import Enum
from collections import defaultdict


class Event(Enum):
    FallAsleep = 0
    WakeUp = 1
    StartShift = 2


class TimeStamp(NamedTuple):
    year: int
    month: int
    day: int
    hour: int
    minute: int


class LogEntry(TypedDict):
    timestamp: TimeStamp
    event: Event
    guard_id: int


ts_pat = re.compile(
    r"^\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) "
    r"(?P<hour>\d{2}):(?P<minute>\d{2})\] (?P<action>wakes up|falls asleep|"
    "Guard #(?P<guard_id>[0-9]+) begins shift)$"
)


def parse_entry(entry_text: str, prev_guard: Optional[int]) -> LogEntry:
    m = ts_pat.match(entry_text)
    if m is None:
        raise ValueError(f"{repr(ts_pat)} did not match {repr(entry_text)}")
    parsed_info = m.groupdict()
    match m["action"][0]:
        case "w":
            event = Event.WakeUp
        case "f":
            event = Event.FallAsleep
        case "G":
            event = Event.StartShift
    if parsed_info["guard_id"] is None:
        guard_id = prev_guard
    else:
        guard_id = int(parsed_info["guard_id"])
    if guard_id is None:
        raise ValueError(
            "Guard ID must be specified either in log event or function args"
        )

    return LogEntry(
        guard_id=guard_id,
        event=event,
        timestamp=TimeStamp(
            **{
                k: int(parsed_info[k])
                for k in ("year", "month", "day", "hour", "minute")
            },
        ),
    )


def main() -> None:
    with open(sys.argv[1] if sys.argv[1:] else "input", "r") as f:
        log_lines: list[str] = sorted(f)
    prev_guard: Optional[int] = None
    entries: defaultdict[int, list[LogEntry]] = defaultdict(list)
    for line in log_lines:
        log_entry = parse_entry(line, prev_guard)
        prev_guard = log_entry["guard_id"]
        entries[prev_guard].append(log_entry)

    max_sleep_count: int = 0
    chosen_guard: int = 0
    most_common_minute: int = 0

    for guard in entries:
        sleep_times: defaultdict[int, int] = defaultdict(int)
        time_asleep: TimeStamp = TimeStamp(0, 0, 0, 0, 0)
        for entry in entries[guard]:
            match entry["event"]:
                case Event.FallAsleep:
                    time_asleep = entry["timestamp"]
                case Event.WakeUp:
                    for minute in range(
                        time_asleep.minute, entry["timestamp"].minute
                    ):
                        sleep_times[minute] += 1
        sleep_freqs = sorted(
            sleep_times, key=lambda m: sleep_times[m], reverse=True
        )
        if sleep_times:  # one guard never slept on the job, so need to check
            sleep_count = sleep_times[sleep_freqs[0]]
            if max_sleep_count < sleep_count:
                most_common_minute = sleep_freqs[0]
                max_sleep_count = sleep_count
                chosen_guard = guard

    print(chosen_guard * most_common_minute)


if __name__ == "__main__":
    main()
