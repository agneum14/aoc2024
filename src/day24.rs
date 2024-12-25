#![allow(dead_code)]
use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::get_input;

struct Grove {
    values: HashMap<String, bool>,
    gates: HashMap<String, Gate>,
}

impl Grove {
    fn new(input: &str) -> Self {
        let (wires, gates) = input.split_once("\n\n").unwrap();
        let values = wires
            .lines()
            .map(|line| line.split_once(": ").unwrap())
            .map(|(id, val)| {
                let val = match val {
                    "1" => true,
                    "0" => false,
                    _ => unreachable!(),
                };
                (id.to_owned(), val)
            })
            .collect::<HashMap<_, _>>();
        let gates = gates
            .lines()
            .map(|line| {
                let data = line.split_whitespace().collect_vec();
                let op = match data[1] {
                    "AND" => Operator::And,
                    "OR" => Operator::Or,
                    "XOR" => Operator::Xor,
                    _ => unreachable!(),
                };
                let gate = Gate {
                    left: data[0].to_string(),
                    right: data[2].to_string(),
                    op,
                };
                (data[4].to_owned(), gate)
            })
            .collect::<HashMap<_, _>>();
        Self { values, gates }
    }

    fn dot(&self) -> String {
        let inner = self
            .gates
            .iter()
            .flat_map(|(id, gate)| {
                let mut inner = Vec::new();
                for id2 in [&gate.left, &gate.right] {
                    inner.push(format!("{} -> {}", id2, id));
                    if id2.starts_with("x") {
                        inner.push(format!("{} [color=\"yellow\"]", id2));
                    } else if id2.starts_with("y") {
                        inner.push(format!("{} [color=\"green\"]", id2));
                    }
                }
                let color = match gate.op {
                    Operator::And => "red",
                    Operator::Or => "blue",
                    Operator::Xor => "orange",
                };
                inner.push(format!("{} [color=\"{}\"]", id, color));
                inner
            })
            .join("\n");
        format!("digraph {{\nnode[style=filled]\n{}\n}}", inner)
    }

    fn resolve_gate(&self, id: &str, values: &mut HashMap<String, bool>) -> bool {
        if let Some(v) = values.get(id) {
            return *v;
        }
        let gate = &self.gates[id];
        let left = self.resolve_gate(&gate.left, values);
        let right = self.resolve_gate(&gate.right, values);
        let res = match gate.op {
            Operator::And => left && right,
            Operator::Or => left || right,
            Operator::Xor => left ^ right,
        };
        values.insert(id.to_owned(), res);
        res
    }

    fn z_decimal(&self) -> u64 {
        let mut values = self.values.clone();
        let binary = self
            .gates
            .keys()
            .filter(|id| id.starts_with("z"))
            .sorted()
            .rev()
            .map(|id| {
                if self.resolve_gate(id, &mut values) {
                    "1"
                } else {
                    "0"
                }
            })
            .join("");
        u64::from_str_radix(&binary, 2).unwrap()
    }
}

#[derive(Debug)]
struct Gate {
    left: String,
    right: String,
    op: Operator,
}

#[derive(Debug)]
enum Operator {
    And,
    Or,
    Xor,
}

pub fn run() {
    let grove = Grove::new(&get_input(24));
    // println!("Z decimal: {}", grove.z_decimal());
    println!("{}", grove.dot())
}

#[cfg(test)]
mod tests {
    use crate::utils::get_small;

    use super::Grove;

    #[test]
    fn z_decimal() {
        let grove = Grove::new(&get_small(24));
        assert_eq!(2024, grove.z_decimal())
    }
}
