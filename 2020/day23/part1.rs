// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2020 Day 23 Part 1

// For an extra challenge, I decided that the only part that would be done at runtime would be
// printing the answer. Everything else, from parsing the input, to simulating the moves, to
// converting the output to a string, would be done at compile time.
//
// Part of why I did this is because it seemed like a good way to push my skills and understanding
// of how to do things in `const`, and the other part is because, with the simple problem
// description, I assumed that part 2 would involve far more numbers and/or rounds, and a simple
// solution would need to be more-or-less completely rewritten either way.
//
// Sure enough, this comment is the last thing I'm writing in Part 1, and I see that part 2 extends
// the input with the range 9..=1_000_000, and runs for 10_000_000 rounds.
struct CrabShuffler {
    state: [u8; 9],
    index: u8,
}

impl CrabShuffler {
    const EXAMPLE_START: Self = Self {
        state: [3, 8, 9, 1, 2, 5, 4, 6, 7],
        index: 0,
    };

    const fn example_test() {
        let mut crab = Self::EXAMPLE_START;
        // PartialEq for arrays isn't const-friendly, so this is an alternative
        macro_rules! expect_state {
            ($index: expr, [$val: literal]) => {{
                assert!(crab.state[$index] == $val, stringify!($index));
            }};
            ($index: expr, [$val: literal, $($vals: literal),+]) => {{
                assert!(crab.state[$index] == $val, stringify!($index));
                expect_state!($index + 1, [$($vals),+]);
            }};
            ([$($vals: literal),+]) => {{ expect_state!(0, [$($vals),+]) }};
        }

        expect_state!([3, 8, 9, 1, 2, 5, 4, 6, 7]);
        crab.shuffle();
        expect_state!([3, 2, 8, 9, 1, 5, 4, 6, 7]);
        crab.shuffle();
        expect_state!([3, 2, 5, 4, 6, 7, 8, 9, 1]);
        crab.shuffle();
        expect_state!([7, 2, 5, 8, 9, 1, 3, 4, 6]);
        crab.shuffle();
        expect_state!([3, 2, 5, 8, 4, 6, 7, 9, 1]);
        crab.shuffle();
        expect_state!([9, 2, 5, 8, 4, 1, 3, 6, 7]);
        crab.shuffle();
        expect_state!([7, 2, 5, 8, 4, 1, 9, 3, 6]);
        crab.shuffle();
        expect_state!([8, 3, 6, 7, 4, 1, 9, 2, 5]);
        crab.shuffle();
        expect_state!([7, 4, 1, 5, 8, 3, 9, 2, 6]);
        crab.shuffle();
        expect_state!([5, 7, 4, 1, 8, 3, 9, 2, 6]);
        crab.shuffle();
        expect_state!([5, 8, 3, 7, 4, 1, 9, 2, 6]);
    }

    const INPUT_START: Self = {
        #[cfg(aoc_direct)]
        let bytes = include_bytes!("input");
        #[cfg(not(aoc_direct))]
        let bytes = include_bytes!("../input");
        let mut arr = [0; 9];
        let mut i = 0;

        while i < 9 {
            let new_val = bytes[i] - b'0';
            assert!(matches!(new_val, 1..=9));
            arr[i] = new_val;
            i += 1;
        }
        // SAFETY: loop initializes all indices.
        let state = arr;
        Self { state, index: 0 }
    };

    const fn label_order(&self) -> [u8; 8] {
        let mut i = 0;
        while self.state[i] != 1 {
            i += 1;
        }
        i += 1;

        let mut out = [0; 8];
        let mut out_i = 0;
        while out_i < 8 {
            out[out_i] = self.state[i];
            out_i += 1;
            i += 1;
            i %= 9;
        }
        out
    }

    const fn shuffle(&mut self) {
        macro_rules! get_cup {
            ($cup: expr) => {{ self.state[((self.index as usize) + 9 + ($cup as usize)) % 9] }};
        }
        macro_rules! set_cup {
            ($cup: expr, $val: expr) => {
                self.state[((self.index as usize) + 9 + ($cup as usize)) % 9] = $val;
            };
        }
        let picked_up = [get_cup!(1), get_cup!(2), get_cup!(3)];
        let left_behind = [
            get_cup!(0),
            get_cup!(4),
            get_cup!(5),
            get_cup!(6),
            get_cup!(7),
            get_cup!(8),
        ];

        const fn held_val(val: u8, held: [u8; 3]) -> bool {
            held[0] == val || held[1] == val || held[2] == val
        }

        // Adding 8 is equivalent to subtracing one when only concerned with the Euclidean
        // remainder when divdied by nine.
        let mut target = (get_cup!(0) + 8) % 9;
        if target == 0 {
            target = 9;
        }

        while held_val(target, picked_up) {
            target += 8;
            target %= 9;
            if target == 0 {
                target = 9;
            }
        }

        let mut i = 0;
        while left_behind[i] != target {
            set_cup!(i, left_behind[i]);
            i += 1;
        }
        set_cup!(i, left_behind[i]);
        set_cup!(i + 1, picked_up[0]);
        set_cup!(i + 2, picked_up[1]);
        set_cup!(i + 3, picked_up[2]);
        i += 1;
        while i < 6 {
            set_cup!(i + 3, left_behind[i]);
            i += 1;
        }

        self.index += 1;
        self.index %= 9;
    }
}

const ANSWER_BYTES: [u8; 8] = {
    CrabShuffler::example_test();
    let mut crab = CrabShuffler::INPUT_START;
    let mut rounds = 0;
    while rounds < 100 {
        rounds += 1;
        crab.shuffle();
    }
    let mut arr = crab.label_order();
    let mut i = 0;
    while i < arr.len() {
        arr[i] += b'0';
        i += 1;
    }
    arr
};

const ANSWER: &str = match str::from_utf8(&ANSWER_BYTES) {
    Ok(s) => s,
    // All bytes in `ANSWER_BYTES` are valid utf-8
    Err(_) => unreachable!(),
};

fn main() {
    println!("{ANSWER}");
}
