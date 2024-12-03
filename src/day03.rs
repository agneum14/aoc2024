#![allow(dead_code)]
use std::fs::read_to_string;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
}

struct Computer {
    memory: Vec<Instruction>,
}

impl Computer {
    fn new(input: &str) -> Self {
        let memory = RE
            .captures_iter(input)
            .map(|x| {
                let fst = x.get(0).unwrap().as_str();
                if fst.starts_with("do(") {
                    Instruction::Do
                } else if fst.starts_with("don") {
                    Instruction::Dont
                } else {
                    let product = x.get(1).unwrap().as_str().parse::<i64>().unwrap()
                        * x.get(2).unwrap().as_str().parse::<i64>().unwrap();
                    Instruction::Mul(product)
                }
            })
            .collect_vec();
        Self { memory }
    }

    fn sum_multiplications(&self) -> i64 {
        self.memory
            .iter()
            .filter(|x| matches!(x, Instruction::Mul(_)))
            .map(|x| match x {
                Instruction::Mul(v) => v,
                _ => unreachable!(),
            })
            .sum()
    }

    fn sum_enabled_multiplications(&self) -> i64 {
        let mut sum = 0;
        let mut enabled = true;

        for x in self.memory.iter() {
            match x {
                Instruction::Do => enabled = true,
                Instruction::Dont => enabled = false,
                Instruction::Mul(v) => {
                    if enabled {
                        sum += v;
                    }
                }
            }
        }

        sum
    }
}

enum Instruction {
    Do,
    Dont,
    Mul(i64),
}

pub fn run() {
    let input = read_to_string("inputs/day03.txt").unwrap();
    let computer = Computer::new(&input);
    println!("Multiplications sum: {}", computer.sum_multiplications());
    println!("Enabled multiplications sum: {}", computer.sum_enabled_multiplications())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_multiplications() {
        let computer = Computer::new(&read_to_string("inputs/day03_small.txt").unwrap());
        assert_eq!(161, computer.sum_multiplications())
    }

    #[test]
    fn enabled_multiplications() {
        let computer = Computer::new(&read_to_string("inputs/day03_small2.txt").unwrap());
        assert_eq!(48, computer.sum_enabled_multiplications())
    }
}
