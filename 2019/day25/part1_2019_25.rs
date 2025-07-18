// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 25 Part 1

// The program is a classic text adventure game. Through trial-and-error, I was able to determine
// that for my input, the following input will take the bot 1 space north of the checkpoint, with
// every item that's possible to pick up in hand, except for the boulder, which is too heavy on its
// own.

const STARTING_INPUT: &str = r#"west
take hypercube
west
take space law space brochure
west
north
take shell
west
take mug
south
take festive hat
north
east
south
east
east
east
east
north
west
north
take whirled peas
west
west
take astronaut ice cream
south
"#;

// In my cargo-based dev environment, `intcode` is a separate crate, but in the in-tree version,
// it's not.
#[cfg(aoc_direct)]
mod intcode;
use intcode::Interpreter;

fn send_input(s: &str) -> impl IntoIterator<Item = i64> {
    s.bytes().map(i64::from)
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut template = Interpreter::new(input.trim().split(",").map(|i| i.parse().unwrap()));
    let (_, state) = template
        .run_through_inputs(send_input(STARTING_INPUT))
        .unwrap();
    assert_eq!(state, intcode::State::Awaiting);

    // There are 7 items, so clone the interpreter, and for each possible combination of items,
    // drop the items not included in the combinations, then, if going south doesn't result in an
    // error message, print the numeric characters that are in the output then break out of the
    // loop.

    for i in 0..=127 {
        let mut interpreter = template.clone();
        macro_rules! drop_item {
            ($shl: literal, $item: literal) => {
                if i & (1 << $shl) == 0 {
                    drop_item!($item);
                }
            };
            ($item: literal) => {
                let (_, intcode::State::Awaiting) = interpreter
                    .run_through_inputs(send_input(concat!("drop ", $item, "\n")))
                    .unwrap()
                else {
                    panic!("not awaiting input")
                };
            };
        }
        drop_item!(0, "hypercube");
        drop_item!(1, "space law space brochure");
        drop_item!(2, "shell");
        drop_item!(3, "mug");
        drop_item!(4, "festive hat");
        drop_item!(5, "whirled peas");
        drop_item!(6, "astronaut ice cream");

        let (output, _) = interpreter
            .run_through_inputs(send_input("south\n"))
            .unwrap();
        let output = String::from_utf8(
            output
                .into_iter()
                .map(|b| u8::try_from(b).unwrap())
                .collect::<Vec<_>>(),
        )
        .unwrap();
        if !output.contains("Alert! Droids on this ship are") {
            println!("{}", output.chars().filter(|c| c.is_numeric()).collect::<String>() );
            break;
        }
    }
}
