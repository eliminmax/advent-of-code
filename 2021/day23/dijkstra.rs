// SPDX-FileCopyrightText: 2025 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

use core::cmp::Reverse;
use core::hash::Hash;
use core::ops::Add;
use std::collections::{BinaryHeap, HashMap};

#[allow(dead_code)]
/// A generic implementation of Dijkstra's Algorithm
/// `start` is the node to start from, and `neighbor_fn` returns a IntoIterator of neighbors and
/// the costs to go to them from the current node
pub fn dijkstra<Node, NeighborIter, Cost>(
    start: Node,
    mut neighbor_fn: impl for<'a> FnMut(Node) -> NeighborIter,
) -> HashMap<Node, Cost>
where
    Node: Hash + Ord + Clone,
    Cost: Ord + Copy + Eq + Default + Add<Cost, Output = Cost>,
    NeighborIter: IntoIterator<Item = (Node, Cost)>,
{
    let mut costs: HashMap<Node, Cost> = HashMap::from([(start.clone(), Cost::default())]);
    let mut queue: BinaryHeap<Reverse<(Cost, Node)>> =
        BinaryHeap::from([Reverse((Cost::default(), start))]);

    while let Some(Reverse((cost, node))) = queue.pop() {
        if costs[&node] < cost {
            continue;
        }

        for (neighbor, conn_cost) in neighbor_fn(node) {
            let next_cost = cost + conn_cost;
            if costs.get(&neighbor).is_none_or(|prev| *prev > next_cost) {
                costs.insert(neighbor.clone(), next_cost);
                queue.push(Reverse((next_cost, neighbor)));
            }
        }
    }

    costs
}

#[allow(dead_code)]
/// a slightly specialized generic implementation of Dijkstra's Algorithm, which will return the
/// cost of a specific node, and will not continue down paths that are more expensive than the
/// currently-known lowest cost to reach that node.
pub fn targeted_dijkstra<Node, NeighborIter, Cost>(
    start: Node,
    target: Node,
    mut neighbor_fn: impl for<'a> FnMut(Node) -> NeighborIter,
) -> Cost
where
    Node: Hash + Ord + Copy,
    Cost: Ord + Copy + Default + Add<Cost, Output = Cost>,
    NeighborIter: IntoIterator<Item = (Node, Cost)>,
{
    let mut costs: HashMap<Node, Cost> = HashMap::from([(start, Cost::default())]);
    let mut queue: BinaryHeap<Reverse<(Cost, Node)>> =
        BinaryHeap::from([Reverse((Cost::default(), start))]);
    let mut max_cost = None;

    while let Some(Reverse((cost, node))) = queue.pop() {
        if costs[&node] < cost {
            continue;
        }

        if let Some(mc) = max_cost
            && mc < cost
        {
            continue;
        }

        for (neighbor, conn_cost) in neighbor_fn(node) {
            let next_cost = cost + conn_cost;
            if costs.get(&neighbor).is_none_or(|prev| *prev > next_cost) {
                costs.insert(neighbor, next_cost);
                queue.push(Reverse((next_cost, neighbor)));
                if neighbor == target {
                    max_cost = Some(next_cost);
                    queue.retain(|&Reverse((cost, _))| cost <= next_cost);
                }
            }
        }
    }
    costs[&target]
}
