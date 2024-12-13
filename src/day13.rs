use std::fs::read_to_string;

use good_lp::{constraint, microlp, variable, ProblemVariables, Solution, SolverModel};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+").unwrap();
}

struct Lobby {
    machines: Vec<Machine>,
}

impl Lobby {
    fn new(input: &str) -> Self {
        let machines = input.split("\n\n").map(|x| Machine::new(x)).collect_vec();
        Self { machines }
    }

    fn fewest_tokens(&self) -> u64 {
        self.machines.iter().filter_map(|m| m.tokens()).sum()
    }
}

#[derive(Debug)]
struct Machine {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
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

    fn tokens(&self) -> Option<u64> {
        let mut problem = ProblemVariables::new();
        let a = problem.add(variable().integer().min(0).max(100));
        let b = problem.add(variable().integer().min(0).max(100));
        let solution = problem
            .minimise(3 * a + b)
            .using(microlp)
            .with(constraint!(self.a.0 * a + self.b.0 * b == self.prize.0))
            .with(constraint!(self.a.1 * a + self.b.1 * b == self.prize.1))
            .solve();
        if let Ok(s) = solution {
            let mut data = [s.value(a), s.value(b)]
                .into_iter()
                .map(|x| x.round() as u64);
            Some(data.next().unwrap() * 3 + data.next().unwrap())
        } else {
            None
        }
    }
}

pub fn run() {
    let input = read_to_string("inputs/day13.txt").unwrap();
    let lobby = Lobby::new(&input);
    println!("Fewest tokens: {}", lobby.fewest_tokens());
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
