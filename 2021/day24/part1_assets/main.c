/*
 * SPDX-FileCopyrightText: 2025 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD
 */

// Brute-force Solution to AoC 2021 Day 24 Part 1

#ifndef _GNU_SOURCE
#define _GNU_SOURCE
#endif
#include <ctype.h>
#include <inttypes.h>
#include <signal.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

#include "part1_assets/monad.h"

static bool decrement(char digits[14]) {
    for (int i = 13; i >= 0; --i) {
        if (--(digits[i])) return true;
        digits[i] = 9;
    }
    return false;
}

static char DIGITS[14] = {9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9};

static void usr1_handler(int) {
#if __has_c_attribute(gnu::nonstring)
    [[gnu::nonstring]]
#endif
    char buf[29] = "Currently at ______________.\n";
    for (int i = 0; i < 14; ++i) buf[i + 13] = DIGITS[i] + '0';
    write(STDERR_FILENO, buf, 29);
}

int main(int argc, char *argv[]) {
    if (argc == 2) {
        if (strlen(argv[1]) != 14) {
            write(STDERR_FILENO, "Input was not 14 digits long\n", 29);
            return 3;
        }
        for (int i = 0; i < 14; ++i) {
            if (!isdigit(argv[1][i])) {
                write(STDERR_FILENO, "Input was not numeric\n", 22);
                return 3;
            } else if (argv[1][i] == '0') {
                write(STDERR_FILENO, "Input has a zero digit\n", 23);
                return 3;
            }
            DIGITS[i] = argv[1][i] - '0';
        }
    }

    int s = sigaction(
        SIGUSR1, &(struct sigaction){.sa_handler = usr1_handler}, NULL
    );
    if (s == -1) {
        perror("sigaction");
        return 2;
    }

    do {
        if (monad(DIGITS) == 0) {
            char *p = DIGITS;

            while (*p) {
                *p += '0';
                ++p;
            }
            write(STDOUT_FILENO, DIGITS, 14);
            write(STDOUT_FILENO, "\n", 1);
            return 0;
        }
    } while (decrement(DIGITS));
    write(STDERR_FILENO, "No valid value found\n", 21);
    return 1;
}
