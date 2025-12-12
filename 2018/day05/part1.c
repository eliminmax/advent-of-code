/*
 * SPDX-FileCopyrightText: 2024 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD
 */

#include <ctype.h>
#include <errno.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Solution to AoC 2018 Day 5 Part 1 */

// take a pointer to length
void process_polymer(char *polymer, size_t len) {
    size_t move_sz = 1;
    for (long i = len - 2; i >= 0; i--) {
        if (tolower(polymer[i]) == tolower(polymer[i + 1]) &&
            polymer[i] != polymer[i + 1]) {
            memmove(&polymer[i], &polymer[i + 2], move_sz);
            len -= (move_sz - 1);
        }
        move_sz++;
    }
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

    long len_l;
    if (fseek(fp, 0, SEEK_END) == -1 || (len_l = ftell(fp)) == -1) {
        fprintf(
            stderr,
            "Failed to find end of file \"%s\": %s.\n",
            filename,
            strerror(errno)
        );
        goto failure_post_open;
    }
    if (fseek(fp, 0, SEEK_SET) == -1) {
        fprintf(
            stderr,
            "Failed to return to start of file \"%s\": %s.\n",
            filename,
            strerror(errno)
        );
        goto failure_post_open;
    }
    size_t len = len_l + 1;
    char *polymer = malloc(len);
    if (polymer == NULL) {
        fputs("Failed to obtain memory for polymer.\n", stderr);
        goto failure_post_open;
    }

    size_t read_sz = fread(polymer, 1, len - 1, fp);
    if (read_sz != len - 1) {
        fprintf(
            stderr,
            "Only read %lu out of expected %ju bytes from %s.\n",
            (uintmax_t)read_sz,
            len,
            filename
        );
        goto failure_post_malloc;
    }
    polymer[len] = '\0';

    process_polymer(polymer, len);
    printf("%ju\n", (uintmax_t)strlen(polymer));
    free(polymer);
    fclose(fp);
    return EXIT_SUCCESS;

failure_post_malloc:
    free(polymer);
failure_post_open:
    fclose(fp);
    return EXIT_FAILURE;
}
