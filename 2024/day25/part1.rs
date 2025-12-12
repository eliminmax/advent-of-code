// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2024 Day 25 Part 1

use std::convert::TryInto;
use std::env::args;
use std::fs::read_to_string;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum ClusterType {
    Key,
    Lock,
}

#[derive(Debug, PartialEq)]
struct PinCluster(ClusterType, [u8; 5]);

impl Index<usize> for PinCluster {
    type Output = u8;
    fn index(&self, i: usize) -> &u8 {
        &self.1[i]
    }
}

#[derive(Debug)]
struct ClusterFormatError;

impl FromStr for PinCluster {
    type Err = ClusterFormatError;
    /// Does not fully validate input:
    ///
    /// While it does validate that the cluster has the right number of rows and columns, and does
    /// check what I think of as the "control rows" - the top and bottom rows - to determine the
    /// ClusterType, other than that, it just counts the number of `b'#'` bytes in each column.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const LOCK_CONTROL: [u8; 5] = [b'#'; 5];
        const KEY_CONTROL: [u8; 5] = [b'.'; 5];
        let rows: [[u8; 5]; 7] = s
            .lines()
            .map(|line| line.bytes().collect::<Vec<_>>().try_into())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| ClusterFormatError)?
            .try_into()
            .map_err(|_| ClusterFormatError)?;
        let cluster_type: ClusterType = match (rows[0], rows[6]) {
            (LOCK_CONTROL, KEY_CONTROL) => Ok(ClusterType::Key),
            (KEY_CONTROL, LOCK_CONTROL) => Ok(ClusterType::Lock),
            _ => Err(ClusterFormatError),
        }?;

        let pin_sizes: [u8; 5] =
            core::array::from_fn(|i| rows[1..=5].iter().filter(|row| row[i] == b'#').count() as u8);
        Ok(PinCluster(cluster_type, pin_sizes))
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");
    let mut keys_n_locks: Vec<PinCluster> = Vec::new();
    for cluster in input.split("\n\n") {
        keys_n_locks.push(PinCluster::from_str(cluster).expect("Failed to parse pin cluster"));
    }
    let (keys, locks): (Vec<_>, Vec<_>) = keys_n_locks
        .into_iter()
        .partition(|a| a.0 == ClusterType::Key);

    let mut fit_count = 0u16;

    for key in keys.into_iter() {
        for lock in locks.iter() {
            if (0usize..5).all(|i| key[i] + lock[i] <= 5) {
                fit_count += 1;
            }
        }
    }
    println!("{fit_count}");
}
