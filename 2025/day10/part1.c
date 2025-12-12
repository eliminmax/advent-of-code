/*
 * SPDX-FileCopyrightText: 2025 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD
 */

#include <assert.h>
#include <errno.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef __clang__
#pragma gcc diagnostic push
#pragma gcc diagnostic ignored "-Wreserved-macro-identifier"
#define _Nonnull
#pragma gcc pop
#endif

/* Solution to AoC 2025 Day 10 Part 1 */

typedef uint16_t u16;
typedef uint8_t u8;

typedef struct queue_entry {
    alignas(4) u16 state;
    u16 cost;
} QueueEntry;

typedef struct queue {
    size_t capacity;
    size_t len;
    QueueEntry *_Nonnull entries [[clang::counted_by(len)]];
} Queue;

// sorts from highest to lowest cost, as that makes queue_pop more efficient
static int queue_sorter(const void *a, const void *b) {
    assert(a != NULL && b != NULL);
    int a_cost = ((QueueEntry *)a)->cost;
    int b_cost = ((QueueEntry *)b)->cost;
    return b_cost - a_cost;
}

[[gnu::nonnull]]
static QueueEntry queue_pop(Queue *_Nonnull queue) {
    qsort(queue->entries, queue->len, sizeof(QueueEntry), queue_sorter);
    // the previous last entry has the lowest cost (possibly tied with others)
    return queue->entries[--queue->len];
}

[[gnu::nonnull]]
void queue_push(Queue *_Nonnull queue, QueueEntry entry) {
    assert(queue->len <= queue->capacity);
    if (queue->len == queue->capacity) {
        void *ptr = reallocarray(
            queue->entries, queue->capacity * 2, sizeof(QueueEntry)
        );
        if (!ptr) {
            fprintf(
                stderr,
                "Failed to reallocate queue capacity from %zu to %zu\n",
                queue->capacity,
                queue->capacity * 2
            );
            abort();
        }
        queue->entries = ptr;
        queue->capacity *= 2;
    }
    queue->entries[queue->len++] = entry;
}

[[gnu::nonnull]]
static u16 shortest_sequence(
    u8 nbuttons, u16 buttons[const _Nonnull nbuttons], u16 target_state
) {
    assert(nbuttons > 0);
    u16 costs[1024] = {0, [1 ... 1023] = 0xff};
    QueueEntry *entries = malloc(sizeof(QueueEntry[32]));
    if (entries == NULL) {
        fputs("Failed to allocate entries\n", stderr);
        abort();
    }
    Queue queue = {32, 1, entries};
    queue.entries[0] = (QueueEntry){.state = 0, .cost = 0};

    while (queue.len) {
        QueueEntry entry = queue_pop(&queue);
        if (costs[entry.state] < entry.cost) continue;
        u16 new_cost = entry.cost + 1;
        if (new_cost >= costs[target_state]) continue;
        for (u8 i = 0; i < nbuttons; ++i) {
            u16 next_state = entry.state ^ buttons[i];
            if (costs[next_state] > new_cost) {
                costs[next_state] = new_cost;
                queue_push(
                    &queue, (QueueEntry){.state = next_state, .cost = new_cost}
                );
            }
        }
    }

    free(queue.entries);

    u16 answer = costs[target_state];
    return answer;
}

[[gnu::nonnull]]
static u16 check_machine(char *_Nonnull line_start) {
    assert(*line_start == '[');
    char *ts_end = strchr(line_start, ']');
    char *cursor = line_start;
    u16 target_state = 0;
    u16 bit = 1;
    while (*++cursor != ']') {
        switch (*cursor) {
            case '.':
                break;
            case '#':
                target_state |= bit;
                break;
            default: {
                fprintf(
                    stderr, "invalid target state character: '%c'\n", *cursor
                );
                exit(EXIT_FAILURE);
            }
        }
        bit <<= 1;
    }

    u8 nbuttons = 0;
    u16 buttons[13] = {};
    cursor = ts_end;
    while ((cursor = strchr(cursor, '('))) {
        while (*++cursor != ')') {
            switch (*cursor) {
                case '0' ... '9':
                    buttons[nbuttons] |= 1 << (*cursor - '0');
                case ',':
                    break;
                default: {
                    fprintf(
                        stderr, "invalid button character: '%c'\n", *cursor
                    );
                    exit(EXIT_FAILURE);
                }
            }
        }
        ++nbuttons;
    }
    return shortest_sequence(nbuttons, buttons, target_state);
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
    char line[256]; // more than large enough to hold any line in the input file

    unsigned long long total = 0;
    while (fgets(line, 256, fp)) total += check_machine(line);

    printf("%llu\n", total);
    fclose(fp);
}
