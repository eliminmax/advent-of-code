// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum NonZeroDigit {
    D1 = 1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
}

#[derive(Debug, Clone, Copy)]
pub struct DigitBlock(i64, i64, i64);

#[link(name = "monad", kind = "static")]
unsafe extern "C" {
    safe fn monad(digits: *const [core::ffi::c_char; 14]) -> i64;
}

impl DigitBlock {
    pub fn into_fn(self) -> impl Fn(NonZeroDigit, i64) -> i64 {
        let DigitBlock(p0, p1, p2) = self;
        move |d: NonZeroDigit, z: i64| {
            let d = d as i64;
            let n = z / p0;
            if d == (z % 26) + p1 {
                n
            } else {
                n * 26 + d + p2
            }
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let lines: Vec<&str> = input.lines().collect();
    let mut blocks: Vec<DigitBlock> = Vec::with_capacity(14);
    for block in lines.chunks(18) {
        let p0 = block[4].strip_prefix("div z ").unwrap().parse().unwrap();
        let p1 = block[5].strip_prefix("add x ").unwrap().parse().unwrap();
        let p2 = block[15].strip_prefix("add y ").unwrap().parse().unwrap();
        blocks.push(DigitBlock(p0, p1, p2));
    }

    let blocks = blocks.as_slice();
    macro_rules! switch_test {
        ($evens: ident, $odds: ident) => {{
            core::array::from_fn(|i| if i % 2 == 0 { NonZeroDigit::$evens } else {NonZeroDigit::$odds})
        }}
    }

    let tests = [
        [NonZeroDigit::D1; 14],
        [NonZeroDigit::D2; 14],
        [NonZeroDigit::D3; 14],
        [NonZeroDigit::D4; 14],
        [NonZeroDigit::D5; 14],
        [NonZeroDigit::D6; 14],
        [NonZeroDigit::D7; 14],
        [NonZeroDigit::D8; 14],
        [NonZeroDigit::D9; 14],
        switch_test!(D1, D2),
        switch_test!(D2, D3),
        switch_test!(D3, D4),
        switch_test!(D4, D5),
        switch_test!(D5, D6),
        switch_test!(D6, D7),
        switch_test!(D7, D8),
        switch_test!(D8, D9),
    ];

    for test in tests {
        let c_answer = dbg!(monad(test.as_ptr() as _));
        let rs_answer = dbg!(test
            .iter()
            .zip(blocks)
            .fold(0, |z, (&d, &block)| block.into_fn()(d, z)));
        assert_eq!(c_answer, rs_answer);
    }
}
