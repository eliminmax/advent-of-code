#!/bin/sh

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

awk -f convert-to-c.awk ../input | python3 relabel.py
