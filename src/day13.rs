#![allow(dead_code)]
use std::fs::read_to_string;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Clone)]
struct Lobby {
    machines: Vec<Machine>,
}

impl Lobby {
    fn new(input: &str) -> Self {
        let machines = input.split("\n\n").map(|x| Machine::new(x)).collect_vec();
        Self { machines }
    }

    fn fewest_tokens(&self) -> i64 {
        self.machines.iter().filter_map(|m| m.tokens()).sum()
    }

    fn fewest_tokens_big(&self) -> i64 {
        let mut big = self.clone();
        for m in big.machines.iter_mut() {
            m.prize = (m.prize.0 + 10000000000000, m.prize.1 + 10000000000000)
        }
        big.fewest_tokens()
    }
}

#[derive(Clone, Copy)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    prize: (i64, i64),
}

impl Machine {
    fn new(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| {
                RE.find_iter(line)
                    .map(|m| m.as_str().parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_vec();
        Self {
            a: data[0],
            b: data[1],
            prize: data[2],
        }
    }

    fn tokens(&self) -> Option<i64> {
        let d = determinant(self.a.0, self.b.0, self.a.1, self.b.1);
        let da = determinant(self.prize.0, self.b.0, self.prize.1, self.b.1);
        let db = determinant(self.a.0, self.prize.0, self.a.1, self.prize.1);
        if da % d != 0 || db % d != 0 {
            None
        } else {
            let a = da / d;
            let b = db / d;
            Some(a * 3 + b)
        }
    }
}

fn determinant(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a * d - b * c
}

pub fn run() {
    let input = read_to_string("inputs/day13.txt").unwrap();
    let lobby = Lobby::new(&input);
    println!("Fewest tokens: {}", lobby.fewest_tokens());
    println!("Fewest tokens big: {}", lobby.fewest_tokens_big());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fewest_tokens() {
        let lobby = Lobby::new(&read_to_string("inputs/day13_small.txt").unwrap());
        assert_eq!(480, lobby.fewest_tokens())
    }
}
