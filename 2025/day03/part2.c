/*
 * SPDX-FileCopyrightText: 2025 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD
 */

#include <assert.h>
#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Solution to AoC 2025 Day 03 Part 2 */

// Return a pointer to the highest-value digit in s[len]. In case of multiple
// highest digits, returns the first match.
static const char *max_digit(unsigned len, const char s[len]) {
    char *ret;
    for (char *p = "9876543210"; *p; ++p) {
        if ((ret = memchr(s, *p, len))) return ret;
    }
    return NULL;
}

static unsigned long long max_joltage(const char *bank) {
    char answer[13];
    answer[12] = 0;
    // If there are N digits left, then the last N-1 digits can't be the next
    // digit, as there wouldn't be enough room. The maximum value would come
    // from getting the first maximum digit before that point, then searching
    // for the remaining digits after the matched digit.
    for (int i = 0; i < 12; ++i) {
        const char *ptr = max_digit(strlen(bank) - (11 - i), bank);
        assert(ptr != NULL);
        answer[i] = *ptr;
        bank = ptr + 1;
    }

    return strtoull(answer, NULL, 10);
}

int main(int argc, char *argv[]) {
    const char *filename = argc > 1 ? argv[1] : "input";
    FILE *fp = fopen(filename, "r");
    if (fp == NULL) {
        fprintf(
            stderr,
            "Failed to open file \"%s\": %s.\n",
            filename,
            strerror(errno)
        );
        return EXIT_FAILURE;
    }
    // actual input lines are 100 long each. This gives enough room for the
    // newline and null terminator when calling `fgets`.
    char line_buf[102];
    unsigned long long total = 0;

    while (fgets(line_buf, 102, fp)) {
        char *ptr = strchr(line_buf, '\n');
        if (ptr) *ptr = '\0';
        total += max_joltage(line_buf);
    }

    if (ferror(fp)) {
        fputs("Error flag on FILE pointer\n", stderr);
        fclose(fp);
        return 1;
    }

    fclose(fp);
    printf("%llu\n", total);
    return EXIT_SUCCESS;
}
