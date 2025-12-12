/*
 * SPDX-FileCopyrightText: 2024 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD
 */

#include <errno.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Solution to AoC 2015 Day 6 Part 1 */
typedef unsigned int uint;

static void set_range(
    uint start_row,
    uint start_col,
    uint end_row,
    uint end_col,
    bool *grid,
    bool target
) {
    for (uint row = start_row; row <= end_row; row++) {
        memset(
            &grid[(row * 1000) + start_col], target, (end_col - start_col + 1)
        );
    }
}

static void toggle_range(
    uint start_row, uint start_col, uint end_row, uint end_col, bool *grid
) {
    for (uint row = start_row; row <= end_row; row++) {
        for (uint col = start_col; col <= end_col; col++) {
            uint index = (row * 1000) + col;
            grid[index] = !grid[index];
        }
    }
}

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

    /* I want to do something clever to simulate a bit array, but this is much
     * faster to implement, as much as it pains me. */
    bool *lights = calloc(sizeof(bool) * 1000, 1000);
    if (lights == NULL) {
        fputs("Failed to allocate memory for light array\n", stderr);
        fclose(fp);
        return EXIT_FAILURE;
    }

    const char *scan_pat = "%u,%u through %u,%u";
    char line_buf[40];
    int scan_ct;
    uint start_row, start_col, end_row, end_col;
    int action;
    char *scan_point;

    while (fgets(line_buf, 40, fp) != NULL) {
        if (line_buf[1] == 'o') { // only true of lines starting with "toggle"
            action = TOGGLE;
            scan_point = &line_buf[7];
        } else if (line_buf[6] == 'n') { // only lines starting with "turn on"
            action = TURN_ON;
            scan_point = &line_buf[8];
        } else { // process of elimination
            action = TURN_OFF;
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

        switch (action) {
            case TURN_ON:
                set_range(start_row, start_col, end_row, end_col, lights, true);
                break;
            case TURN_OFF:
                set_range(
                    start_row, start_col, end_row, end_col, lights, false
                );
                break;
            case TOGGLE:
                toggle_range(start_row, start_col, end_row, end_col, lights);
                break;
        }
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
