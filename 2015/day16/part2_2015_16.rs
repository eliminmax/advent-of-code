// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 16 Part 2

use std::env::args;
use std::fs::read_to_string;
use std::str::FromStr;

#[derive(Debug, Default)]
struct AuntSue {
    sue_number: u16,
    children: Option<u8>,
    cats: Option<u8>,
    samoyeds: Option<u8>,
    pomeranians: Option<u8>,
    akitas: Option<u8>,
    vizslas: Option<u8>,
    goldfish: Option<u8>,
    trees: Option<u8>,
    cars: Option<u8>,
    perfumes: Option<u8>,
}

#[derive(Debug)]
struct MFCSAMReport {
    children: u8,
    cats: u8,
    samoyeds: u8,
    pomeranians: u8,
    akitas: u8,
    vizslas: u8,
    goldfish: u8,
    trees: u8,
    cars: u8,
    perfumes: u8,
}

const REPORT: MFCSAMReport = MFCSAMReport {
    children: 3,
    cats: 7,
    samoyeds: 2,
    pomeranians: 3,
    akitas: 0,
    vizslas: 0,
    goldfish: 5,
    trees: 3,
    cars: 2,
    perfumes: 1,
};

#[derive(Debug)]
enum AuntSueParseError {
    BadFormat,
    NotNumeric,
}

impl FromStr for AuntSue {
    type Err = AuntSueParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::collections::HashMap;
        let (sue_id, sue_info) = s.split_once(": ").ok_or(AuntSueParseError::BadFormat)?;
        let (sue_name, sue_number) = sue_id.split_once(' ').ok_or(AuntSueParseError::BadFormat)?;
        if sue_name != "Sue" {
            return Err(AuntSueParseError::BadFormat);
        }
        let sue_number: u16 = sue_number
            .parse()
            .map_err(|_| AuntSueParseError::NotNumeric)?;

        let mut sue_info = sue_info.replace(": ", ":");
        sue_info.retain(|c| c != ',');

        let sue_info: HashMap<&str, u8> = sue_info
            .split_whitespace()
            .map(|s| {
                s.split_once(':')
                    .ok_or(AuntSueParseError::BadFormat)
                    .and_then(|(prop, n)| {
                        Ok((
                            prop,
                            n.parse::<u8>().map_err(|_| AuntSueParseError::NotNumeric)?,
                        ))
                    })
            })
            .collect::<Result<_, AuntSueParseError>>()?;
        Ok(AuntSue {
            sue_number,
            children: sue_info.get("children").copied(),
            cats: sue_info.get("cats").copied(),
            samoyeds: sue_info.get("samoyeds").copied(),
            pomeranians: sue_info.get("pomeranians").copied(),
            akitas: sue_info.get("akitas").copied(),
            vizslas: sue_info.get("vizslas").copied(),
            goldfish: sue_info.get("goldfish").copied(),
            trees: sue_info.get("trees").copied(),
            cars: sue_info.get("cars").copied(),
            perfumes: sue_info.get("perfumes").copied(),
        })
    }
}

impl AuntSue {
    fn fits_report(&self) -> bool {
        macro_rules! check_prop {
            (cats) => {
                check_prop!(cats, >)
            };
            (trees) => {
                check_prop!(trees, >)
            };
            (pomeranians) => {
                check_prop!(pomeranians, <)
            };
            (goldfish) => {
                check_prop!(goldfish, <)
            };
            ($prop: ident) => {
                check_prop!($prop, ==)
            };
            ($prop0: ident, $($props: ident),+) => {
                check_prop!($prop0) && check_prop!($($props), +)
            };
            ($prop: ident, $cmp_op: tt) =>  {
                self.$prop.is_none_or(|v| v $cmp_op REPORT.$prop)
            };
        }
        check_prop!(
            children,
            cats,
            samoyeds,
            pomeranians,
            akitas,
            vizslas,
            goldfish,
            trees,
            cars,
            perfumes
        )
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    for line in input.lines() {
        let current_aunt = AuntSue::from_str(line).expect("Failed to parse Aunt Sue");
        if current_aunt.fits_report() {
            println!("{}", current_aunt.sue_number);
            break;
        }
    }
}
