<!--
SPDX-FileCopyrightText: 2025 Eli Array Minkoff

SPDX-License-Identifier: 0BSD
-->

<!-- Solution to AoC 2017 Day 23 Part 2 -->

# 2017 Day 23 Part 2

This is the first time where I decided that "works on my machine" is good enough for at least part of solution, and allowed myself to go beyond a single language and its standard library.

First step is to convert the input into C. I wrote an AWK script for that. Its output is not code that any human would write, but it's valid C, which defines a function `long long coprocess(void)` which returns the ending value of `h`.

Every line is preceded with a label, so that jumps work:

```awk
{ printf "L%02d:\n    ", NR }
```

Jumps are handled with the following:

```awk
$1 == "jnz" { printf "if (%s) goto L%02d;\n", $2, NR + $3 }
```

So `jnz e 3` on line 3 of the input would become the following:

```c
L03:
    if (e) goto L06;
```

Most labels are unused, so I then use a Python script (`relabel.py`) to filter out unused labels. This makes me wish I knew Perl.

I then manually rewrote the output to remove dead code paths and `if (true)`-equivalents, then worked on further simplifying the structure, and figuring out loops and conditionals with equivalent control flow to the program.

Given the request that people don't share their inputs, I'm not actually going to share the 3 stages, in their original form but I'll share the changes I made during each stage of it:

## Stage 0:

I created the `convert-to-c.awk` and `relabel.py` scripts, and a shell script (`driver.sh`) to connect them. The output of `driver.sh` is C source code that defines the `coprocess` function, declares a local variable for each register, then contains the direct line-by-line translation of the input.

## Stage 1:

I removed the debug mode code path. With it gone, the beginning was simply setting `b` to a constant value through multiple steps, and setting `c` to a value with a constant offset from `b`. With the debug mode code path eliminated, `a` was no longer read, so I removed it entirely.

I replaced `if(N) goto LXY;` with `goto LXY` for all constant `N` values, and replaced all instances of `R -= -1;` with `R++;` and the one instance of `R -= -N;` with `R += N;`.

## Stage 2:

Because `c` has a constant starting value with `b`, and `b` itself has a constant starting value, I simply set c to a constant. I then saw that `c` was only ever read once, and never changed, so I replaced `c` with its value in that one location. I saw that `g` was only ever used in `if (g) goto LXY;`, and was always set right before that construct, so I replaced instances of `g` with whatever it had just been defined as. I saw that there was an unconditional jump back to the start of the function (right after `b` and `c` had been defined), and right before it, `b` was incremented, and right before that, there was a conditional jump that would skip past the unconditional one. I rewrote the code as a `while (1)` loop, but then immediately noticed that the condition check made it more similar to a hybrid of a `for` loop and a `do while` loop, and given that the condition was clearly true on the first rung, it could be rewritten as a `for` loop. I decided to call it a day there, even though I suspected the remaining process would largely consist of further conversions to for loops, in order to avoid too drastic a jump from one stage to another.

## Stage 3:

In this stage, I finished resolving the control flow, by first replacing constructs that would conditionally skip the next line. For example,

```c
    if (a - b) goto L3;
    c += 3;
  L3:
```

would become

```c
    if (a == b) c += 3
```

(If `(a - b)` is true (i.e. nonzero), then `a` must not equal `b`)



That cleaned things up enough that I was able to identify cases of nested for loops, and rewrite it into the following (with actual constants redacted):
```c
long long coprocess(void) {
    long long b = 0, d = 0, e = 0, f = 0, g = 0, h = 0;
    for (b = START; b != END; b += INCREMENT) {
        f = 1;
        for (d = 2; d != b; d++) {
            for (e = 2; e != b; e++) {
                if ((d * e) == b) f = 0;
            }
        }
        if (!f) h++;
    }
    return h;
}
```

## Stage 4:

I renamed variables based on their apparent purpose, and narrowed their scope down to where they were actually used, and adjusted their types based on their actual ranges. While doing so, I noticed some mistakes in my logic (some checks were flipped), and I went back to correct them, both in stage 3 and this writeup.

```c
#include <stdbool.h>

long coprocess(void) {
    long composites = 0;
    for (long num = START; num != END; num += INCREMENT) {
        bool prime = true;
        for (long i = 2; i != num; i++) {
            for (long j = 2; j != num; j++) {
                if ((i * j) == num) prime = false;
            }
        }
        if (!prime) composites++;
    }
    return composites;
}
```

Now, there are some clear flaws with that - namely, it should not keep checking after it's established that `num` is composite, and the modulo operator can check if `num` is a multiple of `i` without needing the `j` loop. Additionally, a number `N` cannot be a multiple of `M` if `M` is greater than `N / 2`, so the upper bound can easily be cut in half.

## Stage 5: Not quite there.

I created `.gitignore`d files called `start`, `end`, and `increment`, and wrote `part2_2017_23.rs`.
I tried to run it, but the site said my answer was too low, so I had to rethink it. As it turns out the weird hybrid  `do while`/`for` loop does not have identical semantics to a `for` loop, and it runs one more time when the condition is true. Changing `(START..END)` to `(START..=END)` got me the right answer.
