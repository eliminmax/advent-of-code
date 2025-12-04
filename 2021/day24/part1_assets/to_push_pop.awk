#!/usr/bin/env -S awk -f
## SPDX-FileCopyrightText: 2025 Eli Array Minkoff
#
# SPDX-License-Identifier: 0BSD

BEGIN { block = 0 }
# don't increment the starting block 
/inp/  && NR > 1{ ++block }
NR % 18 == 5 { push_pop = ($3 != 1) }
NR % 18 == 6 {
    if(push_pop) printf "if (POP() - %d != digits[%d]) ", -$3, block
}
NR % 18 == 16 {
    print "PUSH(digits[" block "] + "$3")"
    print ""
}
