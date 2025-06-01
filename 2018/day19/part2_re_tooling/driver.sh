#!/bin/sh

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

awk -f pseudo-opcodes.awk ../input | awk -f convert-to-c.awk | clang-format
