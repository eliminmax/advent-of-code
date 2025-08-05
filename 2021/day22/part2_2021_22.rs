// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2021 Day 22 Part 2

use std::ops::RangeInclusive;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Copy)]
struct Span {
    start: i32,
    end: i32,
}

impl Span {
    const fn len(&self) -> u64 {
        self.end.abs_diff(self.start) as u64
    }
    const fn contains(&self, val: i32) -> bool {
        self.start <= val && val < self.end
    }
}

struct Step {
    x: Span,
    y: Span,
    z: Span,
    is_active: bool,
}

impl From<RangeInclusive<i32>> for Span {
    fn from(range: RangeInclusive<i32>) -> Self {
        Self {
            start: *range.start(),
            end: range.end() + 1,
        }
    }
}

impl Step {
    fn panicky_parse(s: &str) -> Self {
        let (state, coords) = s.split_once(' ').unwrap();
        let (x, y, z) = coords
            .split_once(',')
            .and_then(|(x, yz)| yz.split_once(',').map(|(y, z)| (x, y, z)))
            .unwrap();
        let x = x
            .strip_prefix("x=")
            .and_then(|x| x.split_once(".."))
            .map(|(lo, hi)| lo.parse().unwrap()..=(hi.parse::<i32>().unwrap()))
            .unwrap();
        let y = y
            .strip_prefix("y=")
            .and_then(|y| y.split_once(".."))
            .map(|(lo, hi)| lo.parse().unwrap()..=(hi.parse::<i32>().unwrap()))
            .unwrap();
        let z = z
            .strip_prefix("z=")
            .and_then(|z| z.split_once(".."))
            .map(|(lo, hi)| lo.parse().unwrap()..=(hi.parse::<i32>().unwrap()))
            .unwrap();
        let state = match state {
            "on" => true,
            "off" => false,
            bad => panic!("bad state: {bad:?}"),
        };
        Self {
            is_active: state,
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    fn apply(&self, x: i32, y: i32, z: i32) -> Option<bool> {
        if self.x.contains(x) && self.y.contains(y) && self.z.contains(z) {
            Some(self.is_active)
        } else {
            None
        }
    }
}

fn is_active(steps: &[Step], x: i32, y: i32, z: i32) -> bool {
    steps
        .iter()
        .filter_map(|step| step.apply(x, y, z))
        .next()
        .unwrap_or(false)
}

struct PointOfInterestIter {
    x: Vec<Span>,
    y: Vec<Span>,
    z: Vec<Span>,
    x_index: usize,
    y_index: usize,
    z_index: usize,
}

impl PointOfInterestIter {
    fn bump_index(&mut self) {
        self.z_index += 1;
        if self.z_index >= self.z.len() {
            self.z_index = 0;
            self.y_index += 1;
            if self.y_index >= self.y.len() {
                self.y_index = 0;
                self.x_index += 1;
            }
        }
    }

    fn new(x: Vec<Span>, y: Vec<Span>, z: Vec<Span>) -> Self {
        Self {
            x,
            y,
            z,
            x_index: 0,
            y_index: 0,
            z_index: 0,
        }
    }
}

struct GroupIter<const S: usize, T, I: Iterator<Item = T>> {
    inner: I,
}

impl<const S: usize, T, I: Iterator<Item = T>> Iterator for GroupIter<S, T, I> {
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut v = Vec::with_capacity(S);
        for _ in 0..S {
            if let Some(i) = self.inner.next() {
                v.push(i);
            } else {
                break;
            }
        }

        if v.is_empty() {
            None
        } else {
            Some(v)
        }
    }
}

impl<const S: usize, I: Iterator<Item = T>, T> GroupIter<S, T, I> {
    fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl Iterator for PointOfInterestIter {
    type Item = (Span, Span, Span);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.x.get(self.x_index).cloned() {
            if let Some(y) = self.y.get(self.y_index).cloned() {
                if let Some(z) = self.z.get(self.z_index).cloned() {
                    self.bump_index();
                    return Some((x, y, z));
                }
            }
        }
        None
    }
}

fn main() {
    use std::env::{args, var as env_var};
    use std::fs::read_to_string;
    let input =
        read_to_string(args().nth(1).as_deref().unwrap_or("input")).expect("Failed to read file!");
    let steps: Vec<Step> = input.lines().map(Step::panicky_parse).rev().collect();
    let steps: Arc<[Step]> = steps.into();

    let mut x_of_interest: Vec<i32> = Vec::with_capacity(steps.len() * 2);
    let mut y_of_interest: Vec<i32> = Vec::with_capacity(steps.len() * 2);
    let mut z_of_interest: Vec<i32> = Vec::with_capacity(steps.len() * 2);

    for &Step { x, y, z, .. } in steps.iter() {
        x_of_interest.extend([x.start, x.end]);
        y_of_interest.extend([y.start, y.end]);
        z_of_interest.extend([z.start, z.end]);
    }
    x_of_interest.sort();
    x_of_interest.dedup();
    y_of_interest.sort();
    y_of_interest.dedup();
    z_of_interest.sort();
    z_of_interest.dedup();

    let points_to_check = PointOfInterestIter::new(
        x_of_interest
            .windows(2)
            .map(|w| Span {
                start: w[0],
                end: w[1],
            })
            .collect(),
        y_of_interest
            .windows(2)
            .map(|w| Span {
                start: w[0],
                end: w[1],
            })
            .collect(),
        z_of_interest
            .windows(2)
            .map(|w| Span {
                start: w[0],
                end: w[1],
            })
            .collect(),
    );

    let num_child_threads = if let Ok(nthreads) = env_var("AOC_NUM_THREADS") {
        nthreads.parse::<usize>().unwrap().checked_sub(1).unwrap()
    } else {
        match thread::available_parallelism() {
            Ok(n) => n.get() - 1,
            Err(_) => 0,
        }
    };

    type TaskBatcher = GroupIter<1024, (Span, Span, Span), PointOfInterestIter>;

    let batches = Arc::new(Mutex::new(GroupIter::<1024, _, _>::new(points_to_check)));

    let mut threads = Vec::with_capacity(num_child_threads);


    let run_task = |steps: Arc<[Step]>, batches: Arc<Mutex<TaskBatcher>>| -> u64 {
        let mut total = 0;
        while let Some(batch) = { batches.lock().unwrap().next() } {
            for (x, y, z) in batch {
                if is_active(&steps, x.start, y.start, z.start) {
                    total += x.len() * y.len() * z.len();
                }
            }
        }
        total
    };

    for _ in 0..num_child_threads {
        let batches = Arc::clone(&batches);
        let steps = Arc::clone(&steps);
        threads.push(thread::spawn(move || run_task(steps, batches)));
    }

    let mut total = run_task(steps, batches);
    for handle in threads {
        total += handle.join().unwrap();
    }

    println!("{total}");
}
