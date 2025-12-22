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

/* Solution to AoC 2022 Day 20 Part 2 */

// Clang 4 is the oldest version I could find with documented support for the
// _Nonnull pointer annotation. Older versions don't have attributes documented
// with a URL with the format
// https://releases.llvm.org/$VERSION/tools/clang/docs/AttributeReference.html,
// so older versions could still support it - I wouldn't know
#ifndef __clang_major__
#define _Nonnull
#elif __clang_major__ < 4
#define _Nonnull
#endif

// a doubly-linked list node
typedef struct node {
    struct node *_Nonnull prev;
    struct node *_Nonnull next;
    long long value;
} Node;

typedef struct {
    long long size;
    Node *_Nonnull nodes [[clang::counted_by(size)]];
} List;

static List build_list(FILE *_Nonnull fp) {
    // actual input is 5000 lines, so this can fit it exactly, and is more than
    // enough for the much smaller sample input.
    Node *nodes = malloc(sizeof(Node[5000]));
    char line[8];
    if (!nodes) {
        perror("malloc(sizeof(nodes[5000]))");
        abort();
    }
    long long size = 0;
    for (int i = 0; i < 5000; ++size, ++i) {
        if (!fgets(line, 8, fp)) break;

        nodes[i].next = &nodes[i + 1];
        nodes[i].prev = &nodes[i > 0 ? i - 1 : 4999];
        long long value;
        int matched = sscanf(line, "%lld", &value);
        if (matched != 1) {
            char *nl = strchr(line, '\n');
            if (nl) *nl = '\0';
            fprintf(stderr, "failed to parse line %d (\"%s\")\n", i + 1, line);
            free(nodes);
            abort();
        }
        nodes[i].value = value * 811589153LL;
    }
    if (ferror(fp)) {
        fputs("error reading file\n", stderr);
        abort();
    }

    nodes[0].prev = &nodes[size - 1];
    nodes[size - 1].next = nodes;

    return (List){size, nodes};
}

static void mix(Node *_Nonnull node, long long size) {
    Node *prev = node->prev;
    Node *next = node->next;
    next->prev = prev;
    prev->next = next;
    // this took an embarassing amount of time, because I forgot about the order
    // of operations, and was reading `llabs(node->value) % size - 1` as though
    // it were equivalent to the following, leading to quite the wild goose
    // chase.
    long long moves = llabs(node->value) % (size - 1);
    if (moves == 0) {
        next->prev = node;
        prev->next = node;
        return;
    }

    if (node->value < 0) {
        for (long long i = 0; i < moves; ++i) {
            prev = prev->prev;
            next = next->prev;
        }
    } else {
        for (long long i = 0; i < moves; ++i) {
            prev = prev->next;
            next = next->next;
        }
    }
    node->prev = prev;
    node->next = next;
    prev->next = node;
    next->prev = node;
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

    List list = build_list(fp);

    fclose(fp);
    Node *node = list.nodes;

    for (int mix_rounds = 1; mix_rounds <= 10; ++mix_rounds) {
        for (long long i = 0; i < list.size; ++i) {
            mix(&list.nodes[i], list.size);
        }
    }

    long long total = 0;

    while (node->value != 0) node = node->next;
    for (int n = 1; n <= 3000; ++n) {
        node = node->next;
        if (n % 1000 == 0) {
            total += node->value;
        }
    }
    printf("%lld\n", total);

    free(list.nodes);
    return EXIT_SUCCESS;
}
