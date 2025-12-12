/*
 * SPDX-FileCopyrightText: 2024 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD
 */

#include <errno.h>
#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Solution to AoC 2015 Day 6 Part 2 */
typedef unsigned int uint;
typedef unsigned short light;

static void inc_range(
    uint start_row, uint start_col, uint end_row, uint end_col, light *lights
) {
    for (uint row = start_row; row <= end_row; row++) {
        for (uint col = start_col; col <= end_col; col++) {
            uint index = (row * 1000) + col;
            if (lights[index] == USHRT_MAX) abort();
            lights[index]++;
        }
    }
}

static void dec_range(
    uint start_row, uint start_col, uint end_row, uint end_col, light *lights
) {
    for (uint row = start_row; row <= end_row; row++) {
        for (uint col = start_col; col <= end_col; col++) {
            uint index = (row * 1000) + col;
            if (lights[index]) lights[index]--;
        }
    }
}

static void add2_range(
    uint start_row, uint start_col, uint end_row, uint end_col, light *lights
) {
    for (uint row = start_row; row <= end_row; row++) {
        for (uint col = start_col; col <= end_col; col++) {
            uint index = (row * 1000) + col;
            if (lights[index] >= USHRT_MAX - 1) abort();
            lights[index] += 2;
        }
    }
}

typedef void (*light_range_op)(uint, uint, uint, uint, light *);

#define TURN_ON 0
#define TURN_OFF 1
#define TOGGLE 2

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

    /* I'm glad I didn't do something clever to simulate a bit array, as
     * switching to unsigned int (typedefed as light) was much easier */
    light *lights = calloc(sizeof(light) * 1000, 1000);
    if (lights == NULL) {
        fputs("Failed to allocate memory for light array\n", stderr);
        fclose(fp);
        return EXIT_FAILURE;
    }

    const char *scan_pat = "%u,%u through %u,%u";
    char line_buf[40];
    int scan_ct;
    uint start_row, start_col, end_row, end_col;
    char *scan_point;
    light_range_op operation;

    while (fgets(line_buf, 40, fp) != NULL) {
        if (line_buf[1] == 'o') { // only true of lines starting with "toggle"
            operation = add2_range;
            scan_point = &line_buf[7];
        } else if (line_buf[6] == 'n') { // only lines starting with "turn on"
            operation = inc_range;
            scan_point = &line_buf[8];
        } else { // process of elimination
            operation = dec_range;
            scan_point = &line_buf[9];
        }

        scan_ct = sscanf(
            scan_point, scan_pat, &start_row, &start_col, &end_row, &end_col
        );

        if (scan_ct != 4) {
            if (scan_ct == EOF) {
                fprintf(
                    stderr,
                    "Failed to fully parse \"%s\" with format \"%s\": %s\n",
                    scan_point,
                    scan_pat,
                    strerror(errno)
                );
            } else {
                fprintf(
                    stderr,
                    "Failed to fully parse \"%s\" with format \"%s\": "
                    "expected 4 items, but read %d.\n",
                    scan_point,
                    scan_pat,
                    scan_ct
                );
            }
            free(lights);
            fclose(fp);
            return EXIT_FAILURE;
        }

        operation(start_row, start_col, end_row, end_col, lights);
    }
    if (ferror(fp)) {
        fprintf(stderr, "Issue occurred reading %s.\n", filename);
        free(lights);
        fclose(fp);
        return EXIT_FAILURE;
    }

    uint total = 0;
    for (uint row = 0; row < 1000; row++) {
        for (uint col = 0; col < 1000; col++) {
            total += lights[(row * 1000) + col];
        }
    }
    printf("%u\n", total);

    free(lights);
    fclose(fp);
    return EXIT_SUCCESS;
}
