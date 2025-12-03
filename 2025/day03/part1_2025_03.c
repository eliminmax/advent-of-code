/*
 * SPDX-FileCopyrightText: 2025 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

/* Solution to AoC 2025 Day 03 Part 1 */

static char *max_digit(const char *s) {
    char *ret;
    for (char *p = "9876543210"; *p; ++p) {
        if ((ret = strchr(s, *p))) return ret;
    }
    return NULL;
}

static int max_joltage(char *bank) {
    char *m = max_digit(bank);
    if (m == NULL) {
        fprintf(stderr, "Failed to find a digit in \"%s\".\n", bank);
        exit(EXIT_FAILURE);
    }
    char max_d = *m - '0';
    *m = 0;
    char *max_l = max_digit(bank), *max_r = max_digit(m + 1);
    int l = 0, r = 0;
    if (max_r != NULL) {
        r = ((int)max_d * 10) + (*max_r - '0');
    }
    if (max_l != NULL) {
        l = ((int)(*max_l - '0') * 10) + (max_d);
    }
    return l > r ? l : r;
}

int main(int argc, char *argv[]) {
    const char *filename = argc > 1? argv[1] : "input"; 
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
    int total = 0;

    while(fgets(line_buf, 102, fp)) {
        char *ptr = strchr(line_buf, '\n');
        if (ptr) *ptr = '\0';
        total += max_joltage(line_buf);
    }

    if (ferror(fp)) {
        fputs("Error flag on function pointer\n", stderr);
        fclose(fp);
        return 1;
    }

    fclose(fp);
    printf("%d\n", total);
    return EXIT_SUCCESS;
}
