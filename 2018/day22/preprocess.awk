#!/usr/bin/env -S awk -f

# SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

# Run on input to create constants.rs
$1 == "depth:" { printf "pub const DEPTH: u64 = %s;\n", $2 }
$1 == "target:" { printf "pub const TARGET: (u64, u64) = (%s);\n", $2 }
