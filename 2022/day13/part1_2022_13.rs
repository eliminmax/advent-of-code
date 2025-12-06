// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2022 Day 13 Part 1

use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    SingleNum(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        use Packet::{List, SingleNum};
        // [T] (where T: Ord) is sorted lexicographically, which is the exact ordering
        // defined in the puzzle description.
        match (self, other) {
            (List(a), List(b)) => a.cmp(b),
            (SingleNum(a), SingleNum(b)) => a.cmp(b),
            (SingleNum(n), List(l)) => [SingleNum(*n)][..].cmp(l),
            (List(l), SingleNum(n)) => l[..].cmp(&[SingleNum(*n)][..]),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Number(u8),
    Open,
    Close,
}

impl std::str::FromStr for Packet {
    type Err = PacketParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_tokens(tokens: &mut VecDeque<Token>) -> Result<Packet, PacketParseError> {
            match tokens.pop_front() {
                Some(Token::Close) | None => Err(PacketParseError::MismatchedBrackets),
                Some(Token::Number(i)) => Ok(Packet::SingleNum(i)),
                Some(Token::Open) => {
                    let mut list = Vec::new();
                    'inner_list: loop {
                        match tokens.front() {
                            None => return Err(PacketParseError::MismatchedBrackets),
                            Some(Token::Close) => {
                                tokens.pop_front();
                                break 'inner_list;
                            }
                            Some(Token::Open) | Some(Token::Number(_)) => {
                                list.push(parse_tokens(tokens)?);
                            }
                        }
                    }
                    Ok(Packet::List(list))
                }
            }
        }

        let mut tokens = VecDeque::new();

        let mut num = None;
        for c in s.chars() {
            if let Some(digit) = c.to_digit(10) {
                if let Some(n) = num.as_mut() {
                    *n *= 10;
                    *n += digit as u8;
                } else {
                    num = Some(digit as u8);
                }
            } else {
                if let Some(n) = num.take() {
                    tokens.push_back(Token::Number(n));
                }
                match c {
                    '0'..='9' => unreachable!("handled separately already"),
                    '[' => tokens.push_back(Token::Open),
                    ']' => tokens.push_back(Token::Close),
                    ',' => (),
                    _ => return Err(PacketParseError::InvalidChar(c)),
                }
            }
        }

        let packet = parse_tokens(&mut tokens)?;
        if tokens.is_empty() {
            Ok(packet)
        } else {
            Err(PacketParseError::ExtraTokens)
        }
    }
}

#[derive(Debug)]
enum PacketParseError {
    InvalidChar(#[allow(dead_code)] char),
    MismatchedBrackets,
    ExtraTokens,
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let pairs = input
        .split("\n\n")
        .map(|p| p.trim().split_once('\n').unwrap())
        .map(|(l, r)| (l.parse::<Packet>().unwrap(), r.parse::<Packet>().unwrap()));
    let mut total = 0;
    for (i, (l, r)) in pairs.enumerate() {
        if l <= r {
            total += i + 1;
        }
    }
    println!("{total}");
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::Packet;
    #[test]
    fn parse_empty_list() {
        assert_eq!("[]".parse::<Packet>().unwrap(), Packet::List(vec![]));
    }
}
