/*
 * SPDX-FileCopyrightText: 2024 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD
 */

#include <ctype.h>
#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Solution to AoC 2018 Day 5 Part 2 */

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

size_t size_without(
    char removee,
    const char *restrict polymer,
    char *restrict copy_buf,
    size_t len
) {
    memcpy(copy_buf, polymer, len);
    for (long i = len - 1; i >= 0; i--) {
        if (tolower(copy_buf[i]) == removee) {
            memmove(&copy_buf[i], &copy_buf[i + 1], (len - i) + 1);
        }
    }
    process_polymer(copy_buf, len);
    size_t ret = strlen(copy_buf);
    return ret;
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
    char *copy_buf = malloc(len);
    if (copy_buf == NULL) {
        fputs("Failed to obtain memory for copy_buf.\n", stderr);
        free(polymer);
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

    const char alphabet[27] = "abcdefghijklmnopqrstuvwxyz";
    size_t smallest = SIZE_MAX;
    for (int i = 0; i < 26; i++) {
        size_t size_without_c =
            size_without(alphabet[i], polymer, copy_buf, len);
        if (size_without_c < smallest) smallest = size_without_c;
    }
    printf("%ju\n", (uintmax_t)smallest);
    free(copy_buf);
    free(polymer);
    fclose(fp);
    return EXIT_SUCCESS;

failure_post_malloc:
    free(polymer);
    free(copy_buf);
failure_post_open:
    fclose(fp);
    return EXIT_FAILURE;
}
