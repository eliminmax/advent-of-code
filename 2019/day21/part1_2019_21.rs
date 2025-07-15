// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 21 Part 1

// In my cargo-based dev environment, `intcode` is a separate crate, but in the in-tree version,
// it's not.
#[cfg(aoc_direct)]
mod intcode;

use intcode::Interpreter;
use std::process::ExitCode;

macro_rules! springscript {
    {} => {
        "WALK\n"
    };

    {[$op:tt $a:tt $b:tt] $($remaining:tt)*} => {
        concat!(stringify!($op), " ", stringify!($a), " ", stringify!($b), "\n", springscript!{$($remaining)*})
    };
}

// trying a bunch of arbitrary programs, it seems to always fall into holes with one of the following
// arrangements:
//
// 1: #####.##########
// 2: #####.#..#######
// 3: #####..#.#######
// 4: #####...########
//
//
// jumping at `1101` handles patterns 1 and 2, and jumping at 00.1 handles patterns 3 and 4
//
// 
// Sympy's `simplify_logic(((A & B & ~C) | (~A & ~B)))` resolves to
// `D & (A | ~B) & (B | ~A) & (~B | ~C)`
//
//
// In terms of converting to springscript, [AND D J] should be the final instruction, so the
// remaining task is to encode (A | ~B) & (B | ~A) & (~B | ~C), or something equivalent to it.
//
// Plugging that expression into https://www.boolean-algebra.com/, it simpifies to
// `(A & B & ~C) | (~A & ~B)
//
// (~A & ~B) can be stored in T with the following:
//
// NOT A J
// NOT B T
// AND J T
//
// (A & B & ~C) can be stored in J with the following
// NOT C J
// AND A J
// AND B J
//
// so the following should encode D & ((A & B & ~C) | (~A & ~B))
//
// NOT A J
// NOT B T
// AND J T
// NOT C J
// AND A J
// AND B J
// OR T J
// AND D J
//
// I validated that the progam matched the expression with a springscript emulator of mine, but it
// fell in the first hole in arrangement 3 nonetheless
//  
// Back to the drawing board:
//
// I decided to try to arrange the holes based on where they actually need to jump
//          ABCD
//    #####|.###|#######    (setup 1, option 1)
//     ####|#.##|########   (setup 1, option 2)
//      ###|##.#|#########  (setup 1, option 3)
//      ###|##.#|..#######  (setup 2, 1st hole)
//  #####.#|..##|#####      (setup 2, 2nd hole)
//     ####|#..#|.#######   (setup 3, 1st hole)
// #####..#|.###|####       (setup 3, 2nd hole)
//    #####|...#|#######    (setup 4)
// 
// Of the 3 options for setup 1, the 1st and 3rd would look identical to other inputs - (setup 3, 2nd
// hole and setup 2, 1st hole respectively)
//
// 0001
// 0011
// 0111
// 1001
// 1101
//
// As a truth table:
// ABCD|J
// 0000|0
// 0001|1
// 0010|0
// 0011|1
// 0100|0
// 0101|0
// 0110|0
// 0111|1
// 1000|0
// 1001|1
// 1010|0
// 1011|0
// 1100|0
// 1101|1
// 1110|0
// 1111|0
//
// Plugging those into https://www.boolean-algebra.com/kmap/, it resolves the K-Map to the boolean
// expression `(A & ~C & D) | (~B & ~C & D) | (~A & C & D)`
//
// Plugging that into sympy's `simplify_logic` results in
// D & (~A | ~C) & (A | C | ~B)


const PROG0: &str = springscript! {
    [NOT A T]
    [NOT C J]
    [OR T J]
    [NOT B T]
    [OR A T]
    [OR C T]
    [AND T J]
    [AND D J]
};

const _: () = {
    let mut i = 0;
    let mut lines = 0;
    while i < PROG0.len() - 1 {
        if PROG0.as_bytes()[i] == b'\n' {
            lines += 1;
        }
        i += 1;
    }
    assert!(lines <= 16);
};

fn main() -> ExitCode {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut interpreter = Interpreter::new(input.trim().split(",").map(|s| s.parse().unwrap()));

    let (output, status) = interpreter
        .run_through_inputs(PROG0.bytes().map(i64::from))
        .unwrap();

    assert_eq!(status, intcode::State::Halted, "still awaiting input");

    let mut s = String::new();
    for n in output {
        if n > 127 {
            println!("{n}");
            return ExitCode::SUCCESS;
        }
        s.push(n as u8 as char);
    }
    eprintln!("{s}");
    ExitCode::FAILURE
}
