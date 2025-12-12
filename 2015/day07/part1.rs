// SPDX-FileCopyrightText: 2024 Eli Array Minkoff
//
// SPDX-License-Identifier: 0BSD

// Solution to AoC 2015 Day 7 Part 1

use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
enum OpParam {
    Value(u16),
    Wire(String),
}

impl OpParam {
    fn parse(token: &str) -> OpParam {
        match token.parse::<u16>() {
            Ok(val) => OpParam::Value(val),
            Err(_) => OpParam::Wire(String::from(token)),
        }
    }
}

#[derive(Debug, Clone)]
enum WireOp {
    Binary(String, OpParam, OpParam),
    Unary(String, OpParam),
}

#[derive(Debug)]
enum Wire {
    FromOp(WireOp),
    WithValue(u16),
}

#[derive(Debug)]
enum WireKitError {
    Ordering,
    Parse,
    Read,
}

trait WireKit {
    fn parse_rule<'s>(&mut self, rule: &'s str) -> Result<(), &'s str>;
    fn dep_order(&self) -> Result<Vec<String>, WireKitError>;
    fn resolve_op(&self, op: WireOp) -> Result<Wire, WireKitError>;
    fn resolve(&mut self) -> Result<(), WireKitError>;
}

impl WireKit for HashMap<String, Wire> {
    fn parse_rule<'s>(&mut self, rule: &'s str) -> Result<(), &'s str> {
        let words: Vec<&str> = rule.split_whitespace().collect();
        match words.as_slice() {
            [a, op, b, "->", dst] => self.insert(
                String::from(*dst),
                Wire::FromOp(WireOp::Binary(
                    String::from(*op),
                    OpParam::parse(a),
                    OpParam::parse(b),
                )),
            ),
            ["NOT", a, "->", dst] => self.insert(
                String::from(*dst),
                Wire::FromOp(WireOp::Unary(String::from("NOT"), OpParam::parse(a))),
            ),
            [a, "->", dst] => {
                let op_param = OpParam::parse(a);
                self.insert(
                    String::from(*dst),
                    match op_param {
                        OpParam::Value(v) => Wire::WithValue(v),
                        wire_param => Wire::FromOp(WireOp::Unary(String::from("SET"), wire_param)),
                    },
                )
            }
            _ => return Err(rule),
        };
        Ok(())
    }

    fn dep_order(&self) -> Result<Vec<String>, WireKitError> {
        let mut order_map: HashMap<String, Vec<String>> = HashMap::new();
        let mut order_vec: Vec<String> = Vec::new();
        macro_rules! insert_dep {
            ($wire_id: ident) => {{
                order_map.entry(String::from($wire_id)).or_insert(vec![]);
            }};
            ($wire_id: ident, $a: ident) => {{
                order_map
                    .entry(String::from($wire_id))
                    .and_modify(|v| v.push(String::from($a)))
                    .or_insert(vec![String::from($a)]);
            }};
            ($wire_id: ident, $a: ident, $b: ident) => {{
                order_map
                    .entry(String::from($wire_id))
                    .and_modify(|v| {
                        v.push(String::from($a));
                        v.push(String::from($b))
                    })
                    .or_insert(vec![String::from($a), String::from($b)]);
            }};
        }
        for (wire_id, wire_rule) in self.iter() {
            match wire_rule {
                // the following cases have no dependencies on other wires
                Wire::WithValue(_) => insert_dep!(wire_id),
                Wire::FromOp(WireOp::Unary(_, OpParam::Value(_))) => insert_dep!(wire_id),
                Wire::FromOp(WireOp::Binary(_, OpParam::Value(_), OpParam::Value(_))) => {
                    insert_dep!(wire_id)
                }
                // the following cases have dependencies on other wires that must be handled
                Wire::FromOp(WireOp::Unary(_, OpParam::Wire(a))) => insert_dep!(wire_id, a),
                Wire::FromOp(WireOp::Binary(_, OpParam::Wire(a), OpParam::Wire(b))) => {
                    insert_dep!(wire_id, a, b)
                }
                Wire::FromOp(WireOp::Binary(_, OpParam::Wire(a), OpParam::Value(_))) => {
                    insert_dep!(wire_id, a)
                }
                Wire::FromOp(WireOp::Binary(_, OpParam::Value(_), OpParam::Wire(b))) => {
                    insert_dep!(wire_id, b)
                }
            }
        }

        let mut resolved: Vec<String> = Vec::new();
        while !order_map.is_empty() {
            resolved.extend(order_map.iter().filter_map(|(wire_id, wire_deps)| {
                if wire_deps.is_empty() {
                    Some(wire_id.clone())
                } else {
                    None
                }
            }));
            if resolved.is_empty() {
                return Err(WireKitError::Ordering);
            };
            order_map
                .values_mut()
                .for_each(|deps| deps.retain(|id| !resolved.contains(id)));
            resolved.iter().for_each(|id| {
                order_map.remove(id);
            });
            order_vec.append(&mut resolved);
            assert!(resolved.is_empty());
        }
        Ok(order_vec)
    }

    fn resolve_op(&self, op: WireOp) -> Result<Wire, WireKitError> {
        macro_rules! expand_ops {
            ($op_str: expr, $a: expr, $b: expr) => {{
                match $op_str {
                    "AND" => Ok(Wire::WithValue($a & $b)),
                    "OR" => Ok(Wire::WithValue($a | $b)),
                    "LSHIFT" => Ok(Wire::WithValue($a << $b)),
                    "RSHIFT" => Ok(Wire::WithValue($a >> $b)),
                    _ => Err(WireKitError::Parse),
                }
            }};
            ($op_str: expr, $a: expr) => {{
                match $op_str {
                    "SET" => Ok(Wire::WithValue($a)),
                    "NOT" => Ok(Wire::WithValue(!$a)),
                    _ => Err(WireKitError::Parse),
                }
            }};
        }

        match op {
            WireOp::Binary(ref op_string, OpParam::Value(lit_a), OpParam::Value(lit_b)) => {
                expand_ops!(op_string.as_str(), lit_a, lit_b)
            }
            WireOp::Binary(ref op_string, OpParam::Wire(ref a), OpParam::Value(lit_b))
            | WireOp::Binary(ref op_string, OpParam::Value(lit_b), OpParam::Wire(ref a)) => {
                if let Some(Wire::WithValue(lit_a)) = self.get(a) {
                    expand_ops!(op_string.as_str(), *lit_a, lit_b)
                } else {
                    Err(WireKitError::Read)
                }
            }
            WireOp::Binary(ref op_string, OpParam::Wire(ref a), OpParam::Wire(ref b)) => {
                if let (Some(Wire::WithValue(lit_a)), Some(Wire::WithValue(lit_b))) =
                    (self.get(a), self.get(b))
                {
                    expand_ops!(op_string.as_str(), *lit_a, *lit_b)
                } else {
                    Err(WireKitError::Read)
                }
            }
            WireOp::Unary(ref op_string, OpParam::Value(lit_a)) => {
                expand_ops!(op_string.as_str(), lit_a)
            }
            WireOp::Unary(ref op_string, OpParam::Wire(ref a)) => {
                if let Some(Wire::WithValue(lit_a)) = self.get(a) {
                    expand_ops!(op_string.as_str(), *lit_a)
                } else {
                    Err(WireKitError::Read)
                }
            }
        }
    }

    fn resolve(&mut self) -> Result<(), WireKitError> {
        let order = self.dep_order()?;
        for key in order.into_iter() {
            let wire_state = self
                .get(&key)
                .expect("WireKit won't pass itself an invalid key");
            match wire_state {
                Wire::WithValue(_) => continue,
                Wire::FromOp(op) => {
                    let new_state = self.resolve_op(op.clone())?;
                    self.entry(key).insert_entry(new_state);
                }
            };
        }
        Ok(())
    }
}

fn main() {
    let input = read_to_string(args().nth(1).unwrap_or(String::from("input")))
        .expect("Failed to read file!");

    let mut kit: HashMap<String, Wire> = HashMap::new();

    input.lines().for_each(|line| {
        kit.parse_rule(line)
            .unwrap_or_else(|err| panic!("{} could not be parsed as an instruction", err))
    });

    kit.resolve().expect("Failed to resolve values in wire kit");

    match kit.get("a").expect("Key \"a\" missing at end of run.") {
        Wire::WithValue(v) => println!("{v}"),
        e => panic!("\"a\" resolved to {:?}, rather than a Wire::WithValue", e),
    };
}
