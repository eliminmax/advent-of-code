// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2019 Day 23 Part 1

// In my cargo-based dev environment, `intcode` is a separate crate, but in the in-tree version,
// it's not.
#[cfg(aoc_direct)]
mod intcode;
use intcode::Interpreter;
use std::collections::VecDeque;
use std::num::TryFromIntError;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Packet {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Nic<'a> {
    processor: Interpreter<'a>,
    inputs: VecDeque<Packet>,
    halted: bool,
}

#[derive(Debug)]
enum NicOperationError {
    Intcode(#[allow(dead_code, reason = "used in Debug impl")] intcode::ErrorState),
    Destination(#[allow(dead_code, reason = "used in Debug impl")] TryFromIntError),
    IncompleteOutput(#[allow(dead_code, reason = "used in Debug impl")] Vec<i64>),
}

impl<'a> Nic<'a> {
    fn new(
        processor: Interpreter<'a>,
        addr: i64,
        packet_queue: &mut VecDeque<(usize, Packet)>,
    ) -> Result<Self, NicOperationError> {
        let mut new_self = Self {
            processor,
            inputs: VecDeque::new(),
            halted: false,
        };
        let state_update_data = new_self.processor.run_through_inputs(Some(addr))?;
        new_self.update_state(state_update_data, packet_queue)?;

        Ok(new_self)
    }

    fn push(&mut self, packet: Packet) {
        self.inputs.push_back(packet);
    }

    fn update_state(
        &mut self,
        (output, state): (Vec<i64>, intcode::State),
        packet_queue: &mut VecDeque<(usize, Packet)>,
    ) -> Result<(), NicOperationError> {
        if state == intcode::State::Halted {
            self.halted = true;
        }
        if output.len() % 3 != 0 {
            return Err(NicOperationError::IncompleteOutput(output));
        }

        for chunk in output.chunks_exact(3) {
            let dest = chunk[0].try_into()?;
            let packet = Packet {
                x: chunk[1],
                y: chunk[2],
            };
            packet_queue.push_back((dest, packet));
        }

        Ok(())
    }

    fn run_update(
        &mut self,
        packet_queue: &mut VecDeque<(usize, Packet)>,
    ) -> Result<(), NicOperationError> {
        if self.halted {
            Ok(())
        } else {
            let state_update_data: (Vec<i64>, intcode::State) =
                if let Some(Packet { x, y }) = self.inputs.pop_front() {
                    self.processor.run_through_inputs([x, y])
                } else {
                    self.processor.run_through_inputs(Some(-1))
                }?;
            self.update_state(state_update_data, packet_queue)
        }
    }
}

fn main() {
    use std::env::args;
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let mut template = Interpreter::new(input.trim().split(",").map(|i| i.parse().unwrap()));
    template.precompute().unwrap();

    let mut ring_network: Vec<Nic<'_>> = Vec::with_capacity(50);
    let mut packet_queue = VecDeque::new();
    for i in 0..50 {
        ring_network.push(Nic::new(template.clone(), i, &mut packet_queue).unwrap());
    }
    'main_loop: for i in (0..50).cycle() {
        while let Some((addr, packet)) = packet_queue.pop_front() {
            if addr == 255 {
                println!("{}", packet.y);
                break 'main_loop;
            }
            ring_network[addr].push(packet);
        }
        ring_network[i].run_update(&mut packet_queue).unwrap();
    }
}

impl From<intcode::ErrorState> for NicOperationError {
    fn from(e: intcode::ErrorState) -> Self {
        Self::Intcode(e)
    }
}

impl From<TryFromIntError> for NicOperationError {
    fn from(e: TryFromIntError) -> Self {
        Self::Destination(e)
    }
}
