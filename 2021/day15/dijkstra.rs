// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

use core::cmp::Reverse;
use core::hash::Hash;
use core::ops::Add;
use std::collections::{BinaryHeap, HashMap};

/// A generic implementation of Dijkstra's Algorithm
/// `start` is the node to start from, and `neighbor_fn` returns a IntoIterator of neighbors and
/// the costs to go to them from an the location
pub fn dijkstra<Node, NeighborIter, Cost, NeighborFn>(
    start: Node,
    neighbor_fn: NeighborFn,
) -> HashMap<Node, Cost>
where
    Node: Hash + Ord + Copy + Eq + std::fmt::Debug,
    Cost: Ord + Copy + Eq + std::fmt::Debug + Default + Add<Cost, Output = Cost>,
    NeighborIter: IntoIterator<Item = (Node, Cost)>,
    NeighborFn: Fn(Node) -> NeighborIter,
{
    let mut costs: HashMap<Node, Cost> = HashMap::from([(start, Cost::default())]);
    let mut queue: BinaryHeap<Reverse<(Cost, Node)>> =
        BinaryHeap::from([Reverse((Cost::default(), start))]);

    while let Some(Reverse((cost, node))) = queue.pop() {
        if costs[&node] < cost {
            continue;
        }

        for (neighbor, conn_cost) in neighbor_fn(node) {
            let next_cost = cost + conn_cost;
            if costs.get(&neighbor).is_none_or(|prev| *prev > next_cost) {
                costs.insert(neighbor, next_cost);
                queue.push(Reverse((next_cost, neighbor)));
            }
        }
    }

    costs
}
