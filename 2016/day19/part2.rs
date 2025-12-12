// SPDX-FileCopyrightText: 2024 - 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2016 Day 19 Part 2

const CIRCLE_SIZE: usize = include!("input");

fn main() {
    use std::collections::VecDeque;
    // My first approach used a single VecDeque gift_circle and would remove the middle element
    // then rotate, repeating until one element is left. That works, but takes over 20 minutes on
    // my system (compiled with rustc -O).
    //
    // Because it always drops towards the midpoint, I had the idea to use 2 VecDeques, and instead
    // of rotating, pop the first element of one onto the end of the other. It took a lot of
    // thinking and trial-and-error to figure out the order to do things in, but the runtime is
    // down to only around 0.040 seconds on my system (also compiled with rustc -O).

    // setting the cutoff here sets it so that the first element of gift_circle_b is the first to
    // be dropped.
    const CUTOFF: usize = CIRCLE_SIZE / 2 + 1;

    let mut gift_circle_a: VecDeque<_> = (1..CUTOFF).collect();
    let mut gift_circle_b: VecDeque<_> = (CUTOFF..=CIRCLE_SIZE).collect();

    // keep going until only one element is left.
    while gift_circle_a.len() + gift_circle_b.len() > 1 {
        let _ = gift_circle_b.pop_front();
        // move the elf that just went to the back of the line.
        gift_circle_b.push_back(gift_circle_a.pop_front().unwrap());

        // this only runs every other time, which ensures that the bias towards the lower-value
        // elements is preserved.
        //
        // A side-effect of this approach means that gift_circle_b will always be at least as long
        // as gift_circle_a, so the last elf standing will be in gift_circle_b.
        if gift_circle_a.len() + 1 < gift_circle_b.len() {
            gift_circle_a.push_back(gift_circle_b.pop_front().unwrap());
        }
    }
    println!("{}", gift_circle_b[0]);
}
