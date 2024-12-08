use std::fs::read_to_string;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

struct Bridge {
    equations: Vec<Equation>,
}

impl Bridge {
    fn new(input: &str) -> Self {
        let equations = input.lines().map(|x| Equation::new(x)).collect_vec();
        Self { equations }
    }

    fn total_calibration(&self) -> i64 {
        self.equations
            .iter()
            .filter(|x| x.possible())
            .map(|x| x.target)
            .sum()
    }
}

struct Equation {
    target: i64,
    values: Vec<i64>,
}

impl Equation {
    fn new(input: &str) -> Self {
        let mut nums = RE
            .find_iter(input)
            .map(|x| x.as_str().parse::<i64>().unwrap());
        let target = nums.next().unwrap();
        let values = nums.collect_vec();
        Self { target, values }
    }

    fn possible(&self) -> bool {
        self.inner_possible(self.values[0], 1)
    }

    fn inner_possible(&self, cur: i64, idx: usize) -> bool {
        match self.values.get(idx) {
            None => cur == self.target,
            Some(v) => {
                if *v > self.target {
                    true
                } else {
                    let idx = idx + 1;
                    self.inner_possible(cur * v, idx) || self.inner_possible(cur + v, idx)
                }
            }
        }
    }
}

pub fn run() {
    let input = read_to_string("inputs/day07.txt").unwrap();
    let bridge = Bridge::new(&input);
    println!("Total calibration result: {}", bridge.total_calibration())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_calibration() {
        let bridge = Bridge::new(&read_to_string("inputs/day07_small.txt").unwrap());
        assert_eq!(3749, bridge.total_calibration())
    }
}
