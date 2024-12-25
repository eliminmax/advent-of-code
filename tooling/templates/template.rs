// --solution-comment--

use std::env::args;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
}
