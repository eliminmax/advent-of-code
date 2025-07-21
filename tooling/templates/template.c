#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

/* --solution-comment-- */

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
}
