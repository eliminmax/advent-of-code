// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 21 Part 2

// In my cargo-based dev environment, `intcode` is a separate crate, but in the in-tree version,
// it's not.
#[cfg(aoc_direct)]
mod intcode;

use intcode::Interpreter;
use std::fmt;
use std::process::ExitCode;

#[derive(PartialEq, Clone, Copy)]
pub enum SpringScriptOp {
    Not(Register, WritableReg),
    And(Register, WritableReg),
    Or(Register, WritableReg),
}

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
pub enum WritableReg {
    T = 9,
    J = 10,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    T = 9,
    J = 10,
}

// PART 1 NOTES, KEPT FOR REFERENCE
//
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

// Reusing the program from part 1, it falls into the second hole in the following arrangement:
//
// `#####.##.#.#.####`
//
// Flipping `0101` in the truth table should cover that. On the K-map solver, it becomes
// `(~C & D) | (~A & D)`, and per the Distributive law*, that is equivalent to `D & (~A | ~C)`,
// which, in turn, per De Morgan's law*, is equivalent to D & ~(A & C)`
//
// *https://www.electronics-tutorials.ws/wp-content/uploads/2022/09/boolean-algebra-table.jpg
//
// Now, that leads to it falling in the second hole in the following arrangement:
//
// `#####.##.##..####`
//
// Adding `1011` to the table handles that case, but is probably nearing the limit of what can be
// done while ignoring the new inputs.
//
// The truth table solver spits out `(~C & D) | (~B & D) | (~A & D)`, which sympy simplifies to
// `D & (~A | ~B | ~C)`. That, per De Morgan's law, is equivalent to D & ~(A & B & C).
//
// Sure enough, that fails on the following arrangement, with the bot falling into the second hole:
//
// `#####.#.##..#.###`
//
// So seemed to happen, is that it landed on the space surrounded on either side by holes, and
// couldn't continue as both walking and jumping would land in holes.
//
// Only jumping if H is also landable would fix this case, but it may also cause other problems
// Still, let's try it
//
// Sure enough, it falls into `#####...##.#.####` at the first hole
//
// Maybe instead of appending `AND H J`, I could try appending logic to only jump if `H|(E&I)`, so
// that it can either jump right upon landing, or step once then jump, if there are any holes in
// the future.
//
// The part that deals with A through D doesn't use T at all, so nothing needs to be done to clear
// it out.
//
// part 1 start:
//  `[OR A J]`  J = A
//  `[AND B J]` J = A & B
//  `[AND C J]` J = A & B & C
//  `[NOT J J]` J = ~(A & B & C) = (~A | ~B | ~C)
//  `[AND D J]` J = D & ~(A & B & C) = D & (~A | ~B | ~C)
//
// `[OR E T]`   T = E
// `[AND I T]`  T = E & I
// `[OR H T]`   T = (E & I) | H
// `[AND T J]`  J = D & ~(A & B & C) & ((E & I) | H)
//
// Nope. Falls into `#####...###...###` at the first hole. Let's see why:
// `ABCDEFGHI`
// `...###...`
//
// I guess adding `[OR F T]` after `[OR H T]` would fix that, but it might open up other problems.
// Still worth trying...
//
// And it fails on `#####.#.##..#.###` again, in the same place as before, as the check to allow
// `F` doesn't actually make sure it's reachable.
//
//
// Trying ot lay out all of the failure patterns so far, aligned by the first time to jump:
//     #####.###########
//     #####.#..########
//    #####..#.########
//     #####.#..########
//     #####.##.#.#.####
//   #####.#.##..#.###
//   #####...###...###
//
// An expression that should work for all of them is `D & ~(A & C) & (E | H)`.
// That results in a new failure, in the middle hole of the following:
//
// `#####.##.##..####`
//
// Replacing `~(A & C)` with `(~(A & C) | ~(B | E | F))` should fix that, and that is equvialent to
// `~(A & C & (B | E | F))`
//
// Now, it fails on `#####.##.##.#.###`
//
// All failing patterns up to this point:
//
//     #####.###########
//     #####.#..########
//    #####..#.########
//     #####.#..########
//     #####.##.#.#.####
//   #####.#.##..#.###
//   #####...###...###
//     #####.##.##..####
//     #####.##.##.#.###
//
// The last one was added because the rule for ~B didn't trigger, as F had a platform, but at a
// glance, I didn't see any holes that would be problems if `F` was removed entirely, so I gave it
// a try, and finally succeeded.
macro_rules! spring_op {
    [NOT $in: ident $out: ident] => {{ SpringScriptOp::Not(Register::$in, WritableReg::$out) }};
    [AND $in: ident $out: ident] => {{ SpringScriptOp::And(Register::$in, WritableReg::$out) }};
    [OR $in: ident $out: ident] => {{ SpringScriptOp::Or(Register::$in, WritableReg::$out) }};
}

const PROGRAM: &[SpringScriptOp] = &[
    // ~((B | E | F) & A & C)
    spring_op![OR B J],
    spring_op![OR E J],
    spring_op![AND A J],
    spring_op![AND C J],
    spring_op![NOT J J],
    // & (E | H) & D
    spring_op![OR E T], // hopefully the former value of T is a non-issue
    spring_op![OR H T],
    spring_op![AND T J],
    spring_op![AND D J],
];

const _: () = assert!(PROGRAM.len() <= 15);

fn main() -> ExitCode {
    use std::env::args;
    use std::fmt::Write;
    use std::fs::read_to_string;
    assert!(PROGRAM.len() <= 15, "{}", PROGRAM.len());

    let mut prog_text = String::with_capacity(8 * PROGRAM.len() + 4);
    for op in PROGRAM {
        writeln!(&mut prog_text, "{op}").expect("can always write to String");
    }
    prog_text.push_str("RUN\n");

    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut interpreter = Interpreter::new(input.trim().split(",").map(|s| s.parse().unwrap()));
    interpreter.precompute().unwrap();

    let (output, status) = interpreter
        .run_through_inputs(prog_text.bytes().map(i64::from))
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

impl fmt::Display for SpringScriptOp {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpringScriptOp::And(reg, outreg) => write!(fmt, "AND {reg} {outreg}"),
            SpringScriptOp::Or(reg, outreg) => write!(fmt, "OR {reg} {outreg}"),
            SpringScriptOp::Not(reg, outreg) => write!(fmt, "NOT {reg} {outreg}"),
        }
    }
}

impl fmt::Display for WritableReg {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match *self {
                WritableReg::T => 'T',
                WritableReg::J => 'J',
            }
        )
    }
}

impl fmt::Display for Register {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "{}",
            match *self {
                Register::A => 'A',
                Register::B => 'B',
                Register::C => 'C',
                Register::D => 'D',
                Register::E => 'E',
                Register::F => 'F',
                Register::G => 'G',
                Register::H => 'H',
                Register::I => 'I',
                Register::T => 'T',
                Register::J => 'J',
            }
        )
    }
}
