/*
 * SPDX-FileCopyrightText: 2024 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD
 */

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Solution to AoC 2022 Day 4 Part 1 */

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
    unsigned char elf_a_start;
    unsigned char elf_a_end;
    unsigned char elf_b_start;
    unsigned char elf_b_end;
    unsigned int contained = 0;
    // when I checked, the longest line in the input was 11 non-newlines.
    // include room for the newline and the null terminator
    char line_buf[13];
    int scan_ct;
    const char *scan_pat = "%hhu-%hhu,%hhu-%hhu";
    while (fgets(line_buf, 13, fp) != NULL) {
        scan_ct = sscanf(
            line_buf,
            scan_pat,
            &elf_a_start,
            &elf_a_end,
            &elf_b_start,
            &elf_b_end
        );
        if (scan_ct != 4) {
            if (scan_ct == EOF) {
                fprintf(
                    stderr,
                    "Failed to fully parse \"%s\" with format \"%s\": %s\n",
                    line_buf,
                    scan_pat,
                    strerror(errno)
                );
            } else {
                fprintf(
                    stderr,
                    "Failed to fully parse \"%s\" with format \"%s\": "
                    "expected 4 items, but read %d.\n",
                    line_buf,
                    scan_pat,
                    scan_ct
                );
            }
            fclose(fp);
            return EXIT_FAILURE;
        }
        if ((elf_a_start <= elf_b_start && elf_a_end >= elf_b_end) ||
            (elf_b_start <= elf_a_start && elf_b_end >= elf_a_end)) {
            contained++;
        }
    }
    if (ferror(fp)) {
        fprintf(stderr, "Issue occurred reading %s.\n", filename);
        fclose(fp);
        return EXIT_FAILURE;
    }
    fclose(fp);
    printf("%u\n", contained);
    return EXIT_SUCCESS;
}
