// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 16 Part 1

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
enum PacketPayload {
    Data(u64),
    SubPackets(Box<[Packet]>),
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    type_id: u8,
    payload: PacketPayload,
}

impl Packet {
    fn tally_version(&self) -> u32 {
        match &self.payload {
            PacketPayload::Data(_) => self.version.into(),
            PacketPayload::SubPackets(sp) => {
                u32::from(self.version) + sp.iter().map(|i| i.tally_version()).sum::<u32>()
            }
        }
    }
}

#[derive(Debug)]
struct MissingData;

trait PacketStream: Iterator<Item = Bit> {
    fn next_bit(&mut self) -> Result<Bit, MissingData> {
        <Self as Iterator>::next(self).ok_or(MissingData)
    }

    fn parse_packet(&mut self) -> Result<Packet, MissingData> {
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
        let version = take_bits!(3, u8);
        let type_id = take_bits!(3, u8);

        let payload = if type_id == 4 {
            let mut data = 0;
            loop {
                let marker = self.next_bit()?;
                data <<= 4;
                data |= take_bits!(4, u64);
                if marker == Bit::Zero {
                    break;
                }
            }
            PacketPayload::Data(data)
        } else {
            PacketPayload::SubPackets(
                match self.next_bit()? {
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
                .into_boxed_slice(),
            )
        };

        Ok(Packet {
            version,
            type_id,
            payload,
        })
    }
}

impl<I: Iterator<Item = Bit>> PacketStream for I {}

struct PacketIter<PS: PacketStream> {
    inner: PS,
}

impl<PS: PacketStream> PacketIter<PS> {
    fn new(inner: PS) -> Self {
        Self { inner }
    }
}

impl<PS: PacketStream> Iterator for PacketIter<PS> {
    type Item = Packet;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.parse_packet().ok()
    }
}

#[derive(Debug)]
enum PacketParseError {
    NonHex,
    MissingData,
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let packets: Vec<Packet> = PacketIter::new(
        input
            .trim()
            .bytes()
            .flat_map(|b| NibbleIter::try_from(b).unwrap()),
    )
    .collect();
    println!("{}", packets.iter().map(Packet::tally_version).sum::<u32>());
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
mod tests {
    use super::*;

    fn test_iter(hex_str: &[u8]) -> PacketIter<impl Iterator<Item = Bit>> {
        PacketIter {
            inner: hex_str
                .iter()
                .copied()
                .flat_map(|b| NibbleIter::try_from(b).unwrap()),
        }
    }

    macro_rules! packet {
        {$version: literal, 4 => $value: literal} => {{
            Packet {
                version: $version,
                type_id: 4,
                payload: PacketPayload::Data($value)
            }
        }};
        {$version: literal, $type_id: literal => $packets: expr} => {{
            Packet {
                version: $version,
                type_id: $type_id,
                payload: PacketPayload::SubPackets(Box::new($packets)),
            }
        }}
    }

    #[test]
    fn test_parse_2021() {
        let test_packet = test_iter(b"D2FE28").next().unwrap();
        assert_eq!(test_packet, packet! {6, 4 => 2021});
    }

    #[test]
    fn test_parse_operators() {
        let test_packet = test_iter(b"38006F45291200").next().unwrap();
        let expected = packet! {
            1, 6 => [
                packet! { 6, 4 => 10 },
                packet! { 2, 4 => 20 },
            ]
        };
        assert_eq!(test_packet, expected);

        let test_packet = test_iter(b"EE00D40C823060").next().unwrap();
        let expected = packet! {
            7, 3 => [
                packet! { 2, 4 => 1 },
                packet! { 4, 4 => 2 },
                packet! { 1, 4 => 3 },
            ]
        };
        assert_eq!(test_packet, expected);
    }

    #[test]
    fn test_packet_tally() {
        let test_packet = test_iter(b"8A004A801A8002F478").next().unwrap();
        assert_eq!(test_packet.tally_version(), 16);
        let test_packet = test_iter(b"620080001611562C8802118E34").next().unwrap();
        assert_eq!(test_packet.tally_version(), 12);
        let test_packet = test_iter(b"C0015000016115A2E0802F182340").next().unwrap();
        assert_eq!(test_packet.tally_version(), 23);
        let test_packet = test_iter(b"A0016C880162017C3686B18A3D4780").next().unwrap();
        assert_eq!(test_packet.tally_version(), 31);
    }
}
