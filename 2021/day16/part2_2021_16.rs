// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 16 Part 2

#[derive(PartialEq, Eq, Clone, Copy)]
enum Bit {
    Zero = 0,
    One = 1,
}

/// A space-efficient iterator over a nibble that only takes up 1 byte.
/// uses the higher 4 bits to store the number of unconsumed bits in the nibble, and the lower 4
/// bits to store the nibble itself.
#[derive(Clone)]
struct NibbleIter(u8);

impl Iterator for NibbleIter {
    type Item = Bit;

    fn next(&mut self) -> Option<Self::Item> {
        let mut remaining = self.0 >> 4;

        if remaining == 0 {
            return None;
        }

        remaining -= 1;
        let nibble = self.0 & 0xf;
        let bit = nibble & (1 << remaining);

        self.0 = nibble | (remaining << 4);

        Some(if bit == 0 { Bit::Zero } else { Bit::One })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let size = usize::from(self.0 >> 4);
        (size, Some(size))
    }
}

#[derive(Debug, PartialEq)]
enum Packet {
    Sum(Box<[Packet]>),
    Prod(Box<[Packet]>),
    Min(Box<[Packet]>),
    Max(Box<[Packet]>),
    Literal(u64),
    GreaterThan(Box<[Packet; 2]>),
    LessThan(Box<[Packet; 2]>),
    EqualTo(Box<[Packet; 2]>),
}

impl Packet {
    fn resolve_value(&self) -> u64 {
        macro_rules! cmp_op {
            ($packets: ident, $op: tt) => {{
                if $packets[0].resolve_value() $op $packets[1].resolve_value() { 1 } else { 0 }
            }}
        }
        match &self {
            Packet::Sum(packets) => packets.iter().map(Packet::resolve_value).sum(),
            Packet::Prod(packets) => packets.iter().map(Packet::resolve_value).product(),
            Packet::Min(packets) => packets.iter().map(Packet::resolve_value).min().unwrap(),
            Packet::Max(packets) => packets.iter().map(Packet::resolve_value).max().unwrap(),
            Packet::Literal(l) => *l,
            Packet::GreaterThan(packets) => cmp_op!(packets, >),
            Packet::LessThan(packets) => cmp_op!(packets, <),
            Packet::EqualTo(packets) => cmp_op!(packets, ==),
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let packet = input
        .trim()
        .bytes()
        .flat_map(|b| NibbleIter::try_from(b).unwrap())
        .parse_packet()
        .unwrap();
    println!("{}", packet.resolve_value());
}

#[derive(Debug)]
struct MissingData;

trait PacketStream: Iterator<Item = Bit> {
    fn next_bit(&mut self) -> Result<Bit, MissingData> {
        <Self as Iterator>::next(self).ok_or(MissingData)
    }

    fn parse_packet(&mut self) -> Result<Packet, PacketParseError> {
        // a macro to take the next N bits of the iterator, and pack them into the provided type
        macro_rules! take_bits {
            ($nbits: literal, $t: ty) => {{
                let mut res: $t = 0;
                for _ in 0..$nbits {
                    res <<= 1;
                    res |= self.next_bit()? as $t;
                }
                res
            }};
        }
        let _version = take_bits!(3, u8);
        let type_id = take_bits!(3, u8);

        Ok(if type_id == 4 {
            let mut data = 0;
            loop {
                let marker = self.next_bit()?;
                data <<= 4;
                data |= take_bits!(4, u64);
                if marker == Bit::Zero {
                    break;
                }
            }
            Packet::Literal(data)
        } else {
            let subpackets = match self.next_bit()? {
                Bit::Zero => {
                    let nbits = take_bits!(15, usize);
                    let mut subpacket_bits = Vec::with_capacity(nbits);
                    for _ in 0..nbits {
                        subpacket_bits.push(self.next_bit()?);
                    }
                    let mut subpacket_bits = subpacket_bits.into_iter();
                    let mut subpackets = Vec::new();
                    while subpacket_bits.len() > 0 {
                        subpackets.push(subpacket_bits.parse_packet()?);
                    }
                    subpackets
                }
                Bit::One => {
                    let npackets = take_bits!(11, usize);
                    (0..npackets)
                        .map(|_| self.parse_packet())
                        .collect::<Result<Vec<Packet>, _>>()?
                }
            }
            .into_boxed_slice();
            match type_id {
                0 => Packet::Sum(subpackets),
                1 => Packet::Prod(subpackets),
                2 => Packet::Min(subpackets),
                3 => Packet::Max(subpackets),
                4 => unreachable!("already handled above"),
                5 => Packet::GreaterThan(
                    subpackets
                        .try_into()
                        .map_err(|_| PacketParseError::WrongItemCount)?,
                ),
                6 => Packet::LessThan(
                    subpackets
                        .try_into()
                        .map_err(|_| PacketParseError::WrongItemCount)?,
                ),
                7 => Packet::EqualTo(
                    subpackets
                        .try_into()
                        .map_err(|_| PacketParseError::WrongItemCount)?,
                ),
                t => return Err(PacketParseError::BadType(t)),
            }
        })
    }
}

impl<I: Iterator<Item = Bit>> PacketStream for I {}

#[derive(Debug)]
enum PacketParseError {
    NonHex,
    MissingData,
    BadType(#[allow(dead_code)] u8),
    WrongItemCount,
}

impl From<NonHex> for PacketParseError {
    fn from(_: NonHex) -> Self {
        Self::NonHex
    }
}

impl From<MissingData> for PacketParseError {
    fn from(_: MissingData) -> Self {
        Self::MissingData
    }
}

#[derive(Debug)]
struct NonHex;

impl TryFrom<u8> for NibbleIter {
    type Error = NonHex;
    fn try_from(b: u8) -> Result<Self, Self::Error> {
        match b {
            b'0'..=b'9' => Ok(Self(0x40 | (b - b'0'))),
            b'A'..=b'F' => Ok(Self(0x40 | (b - b'A' + 10))),
            _ => Err(NonHex),
        }
    }
}

mod fmt_impls {
    use super::*;
    use std::fmt::{self, Debug, Display, Formatter};

    impl Display for NibbleIter {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let remaining = self.0 >> 4;
            match remaining {
                0 => write!(f, "NibbleIter({:04b}@[])", self.0 & 0b1111),
                1 => write!(
                    f,
                    "NibbleIter({:03b} @[{:b}])",
                    (self.0 & 0b1110) >> 1,
                    self.0 & 1
                ),
                2 => write!(
                    f,
                    "NibbleIter({:02b} @[{:b}] {:b})",
                    (self.0 & 0b1100) >> 2,
                    (self.0 & 0b10) >> 1,
                    self.0 & 1
                ),
                3 => write!(
                    f,
                    "NibbleIter({:b} @[{:b}] {:02b})",
                    (self.0 & 0b1000) >> 3,
                    (self.0 & 0b100) >> 2,
                    self.0 & 0b11,
                ),
                4 => write!(
                    f,
                    "NibbleIter(@[{:b}] {:03b})",
                    (self.0 & 0b1000) >> 3,
                    self.0 & 0b111,
                ),
                5..=0b1111 => panic!("invalid remaining count for NibbleIter: {remaining}"),
                _ => unreachable!("Bit shift makes this impossible"),
            }
        }
    }

    impl Debug for NibbleIter {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let remaining = self.0 >> 4;
            let nibble = self.0 & 0b1111;
            f.debug_struct("NibbleIter")
                .field("remaining", &remaining)
                .field("nibble", &nibble)
                .finish()
        }
    }

    impl Display for Bit {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{:1b}", *self as u8)
        }
    }

    impl Debug for Bit {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Zero => write!(f, "Bit(1)"),
                Self::One => write!(f, "Bit(0)"),
            }
        }
    }
}

#[cfg(test)]
#[test]
fn examples() {
    macro_rules! packet_from {
        ($arr: literal) => {
            $arr.iter()
                .flat_map(|&b| NibbleIter::try_from(b).unwrap())
                .parse_packet()
                .unwrap()
        };
    }
    assert_eq!(packet_from!(b"C200B40A82").resolve_value(), 3);
    assert_eq!(packet_from!(b"04005AC33890").resolve_value(), 54);
    assert_eq!(packet_from!(b"880086C3E88112").resolve_value(), 7);
    assert_eq!(packet_from!(b"CE00C43D881120").resolve_value(), 9);
    assert_eq!(packet_from!(b"D8005AC2A8F0").resolve_value(), 1);
    assert_eq!(packet_from!(b"F600BC2D8F").resolve_value(), 0);
    assert_eq!(packet_from!(b"9C005AC2F8F0").resolve_value(), 0);
    assert_eq!(
        packet_from!(b"9C0141080250320F1802104A08").resolve_value(),
        1
    );
}
