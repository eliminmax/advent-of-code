/* SPDX-FileCopyrightText: 2025 Eli Array Minkoff
 *
 * SPDX-License-Identifier: 0BSD */

/* Solution to AoC 2020 Day 23 Part 2 */

/* I believe that the Rust VecDeque-based approach is as slow as it is because
 * it needs to cycle through until it finds the target element, so it's moving
 * memory around a lot. The problem is that a faster approach would need to be
 * able to efficiently get and modify other elements, but also insert elements
 * at arbitrary positions efficiently.
 *
 * None of Rust's standard collection types seem like they'd do the trick.
 *
 * The closest is a LinkedList, but its use is heavily discouraged in Rust's
 * documentation, and it's slow to go through and find the destination, so it's
 * got both the cache inefficiency of linked lists, and the same problem as the
 * VecDeque when re-inserting the removed nodes.
 *
 * I was thinking of using a sequence of nodes that work as a doubly-linked
 * list, but are stored consecutively in memory, in a single allocation, in
 * ascending numeric order, so that they can also be accessed as an array - the
 * index would be one less than the cup number.
 *
 * Building that in Rust seemed like a daunting task that I'd learn a lot from,
 * and would require quite a bit of `unsafe`, but I would rather implement it in
 * C, as I've never worked with linked lists in either language as-is */

#define NDEBUG

#include <assert.h>
#include <inttypes.h>
#include <limits.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INPUT_LEN 9

// Embed the input within the binary, but not the source code, using a C23
// addition.
static const char INPUT[INPUT_LEN] = {
#ifndef AOC_TEST
#embed "input"
#else
    "389125467"
#endif
};

#define INPUT_IDX(i) ((INPUT[i]) - '1')

typedef struct cup_list_node {
    struct cup_list_node *next;
    uint32_t num;
} Cup;

#ifndef NDEBUG
static uint32_t cycle_len(const Cup *head) {
    uint32_t len = 1;
    const Cup *current = head->next;

    while (current != head) {
        ++len;
        current = current->next;
    }
    return len;
}
#endif /* NDEBUG */

[[gnu::returns_nonnull]]
// Return a pointer to 1000000 heap-allocated Cups, with pointers initialized to
// work as a cycling doubly-linked list.
//
// The first INPUT_LEN cups are linked in the order specified by INPUT, and the
// rest are linked in sequential order
//
// aborts if `malloc(sizeof(Cup[1000000]))` returns NULL
static Cup *initial_cups() {
    // input validation
    // Does nothing if NDEBUG is defined.
    //
    // Otherwise, checks INPUT contains all digits 1 through 9 with no
    // duplicates.
    //
    // Prints an error then aborts in case of failure.
#ifndef NDEBUG
    bool nums_seen[INPUT_LEN] = {};
    // Check that all digits 1 through 9 are present in inputs
    for (uint32_t i = 0; i < INPUT_LEN; ++i) {
        if (nums_seen[INPUT_IDX(i)]) {
            fprintf(stderr, "Duplicate entry for %c.\n", INPUT[i]);
            abort();
        }
        nums_seen[INPUT[i] - '1'] = true;
    }

    // this should be covered by the above loop, but no harm in extra caution in
    // debug builds, I suppose.
    for (uint32_t i = 0; i < INPUT_LEN; ++i) {
        if (!nums_seen[i]) {
            fprintf(stderr, "Missing entry for %d.\n", i + 1);
            abort();
        }
    }
#endif /* NDEBUG */
    Cup *cups = malloc(sizeof(Cup[1000000]));
    if (cups == NULL) {
        fputs("Failed to allocate memory for 1 million cups!\n", stderr);
        abort();
    }

    // start with the cups that start out sequential on both sides - so none of
    // 0 through INPUT_LEN (inclusive), and not the last element. Initialize
    // them fully.

    for (int i = 0; i < INPUT_LEN - 1; ++i) {
        cups[INPUT_IDX(i)] = (Cup){
            .next = &cups[INPUT_IDX(i + 1)],
            .num = INPUT[i] - '0',
        };
    }

    cups[INPUT_IDX(INPUT_LEN - 1)] = (Cup){
        .next = &cups[INPUT_LEN],
        .num = INPUT[INPUT_LEN - 1] - '0',
    };

    for (uint32_t i = INPUT_LEN; i < 999999; ++i) {
        cups[i] = (Cup){
            .next = &cups[i + 1],
            .num = i + 1,
        };
    }

    // the final element points back to the first
    cups[999999] = (Cup){
        .next = &cups[INPUT_IDX(0)],
        .num = 1000000,
    };
#ifndef NDEBUG
    uint32_t cl = cycle_len(&cups[INPUT_IDX(0)]);
    if (cl != 1000000) {
        fprintf(stderr, "Cycle length is %" PRIu32 ", not 1000000.\n", cl);
        abort();
    }
#endif /* NDEBUG */

    return cups;
}

[[gnu::nonnull]]
// shuffle the cups, returning the next head cup
static Cup *shuffle(Cup *head, Cup cups[1000000]) {
    Cup *picked_up[3];
    picked_up[0] = head->next;
    picked_up[1] = picked_up[0]->next;
    picked_up[2] = picked_up[1]->next;

    uint32_t target = head->num - 1;
    if (!target) target = 1000000;

    // keep going until target is resolved
    while (target == picked_up[0]->num || target == picked_up[1]->num ||
           target == picked_up[2]->num) {
        --target;
        if (target == 0) target = 1000000;
    }

    assert(cups[target - 1].num == target);

    // skip over the picked up elements at the original location
    head->next = picked_up[2]->next;
    // insert the skipped elements after destination
    picked_up[2]->next = cups[target - 1].next;
    cups[target - 1].next = picked_up[0];

    return head->next;
}

int main() {
    Cup *cups = initial_cups();
    Cup *head = &cups[INPUT[0] - '1'];
    for (uint32_t i = 0; i < 10000000; ++i) {
        head = shuffle(head, cups);
#ifndef NDEBUG
        uint32_t cl = cycle_len(head);
        if (cl != 1000000) {
            fprintf(stderr, "Cycle length %" PRIu32 "!\n", cl);
            abort();
        }
#endif /* NDEBUG */
    }
    uint64_t a = cups[0].next->num;
    uint64_t b = cups[0].next->next->num;
    free(cups);

    printf("%" PRIu64 "\n", a * b);
    return EXIT_SUCCESS;
}

// vi: tw=80
