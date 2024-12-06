// --solution-comment--

use std::env::args;
// Choose one of the following:
use std::fs::read;
use std::fs::read_to_string;

fn main() {
    let mut input = read_to_string(args().nth(1).unwrap_or("input")).expect("Failed to read file!");
}
