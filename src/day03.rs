use std::fs::read_to_string;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
}

struct Computer {
    memory: String,
}

impl Computer {
    fn new(input: &str) -> Self {
        let memory = input.to_string();
        Self { memory }
    }

    fn sum_multiplications(&self) -> i64 {
        RE.captures_iter(&self.memory)
            .map(|x| {
                let (_, [a, b]) = x.extract();
                let (a, b) = (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap());
                a * b
            })
            .sum()
    }
}

pub fn run() {
    let input = read_to_string("inputs/day03.txt").unwrap();
    let computer = Computer::new(&input);
    println!("Multiplications sum: {}", computer.sum_multiplications())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_multiplications() {
        let computer = Computer::new(&read_to_string("inputs/day03_small.txt").unwrap());
        assert_eq!(161, computer.sum_multiplications())
    }
}
