#![allow(dead_code)]
use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

use crate::utils::get_input;

struct MonkeyMarket {
    secrets: Vec<i64>,
}

impl MonkeyMarket {
    fn new(input: &str) -> Self {
        let secrets = input
            .lines()
            .map(|line| line.parse().unwrap())
            .collect_vec();
        Self { secrets }
    }

    fn sum_2000s(&self) -> i64 {
        self.secrets
            .iter()
            .map(|secret| {
                let mut secret = *secret;
                for _ in 0..2000 {
                    secret = MonkeyMarket::transform(secret);
                }
                secret
            })
            .sum()
    }

    fn transform(secret: i64) -> i64 {
        let mut secret = MonkeyMarket::prune(MonkeyMarket::mix(secret, secret * 64));
        secret = MonkeyMarket::mix(secret, secret / 32);
        secret = MonkeyMarket::prune(MonkeyMarket::mix(secret, secret * 2048));
        secret
    }

    fn mix(secret: i64, given: i64) -> i64 {
        secret ^ given
    }

    fn prune(secret: i64) -> i64 {
        secret % 16777216
    }

    fn most_bananas(&self) -> i64 {
        let mut differences_bananas = HashMap::new();
        for secret in self.secrets.iter() {
            let mut last_secret = *secret;
            let mut last_digit = last_secret % 10;
            let mut differences = VecDeque::new();
            let mut seen = HashSet::new();
            for _ in 0..4 {
                let new_secret = MonkeyMarket::transform(last_secret);
                let new_digit = new_secret % 10;
                differences.push_back(new_digit - last_digit);
                last_secret = new_secret;
                last_digit = new_digit;
                seen.insert(differences.to_owned());
                *differences_bananas
                    .entry(differences.to_owned())
                    .or_insert(0) += last_digit;
            }
            differences_bananas.insert(differences.to_owned(), last_digit);
            for _ in 0..1996 {
                let new_secret = MonkeyMarket::transform(last_secret);
                let new_digit = new_secret % 10;
                differences.push_back(new_digit - last_digit);
                differences.pop_front();
                last_secret = new_secret;
                last_digit = new_digit;
                if !seen.contains(&differences) {
                    *differences_bananas
                        .entry(differences.to_owned())
                        .or_insert(0) += last_digit;
                }
                seen.insert(differences.to_owned());
            }
        }
        *differences_bananas.values().max().unwrap()
    }
}

pub fn run() {
    let mm = MonkeyMarket::new(&get_input(22));
    println!("2000th secret numbers sum: {}", mm.sum_2000s());
    println!("Most bananas: {}", mm.most_bananas());
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::{get_small, get_smalln};

    #[test]
    fn sum() {
        let mm = MonkeyMarket::new(&get_small(22));
        assert_eq!(37327623, mm.sum_2000s());
    }

    #[test]
    fn most_bananas() {
        let mm = MonkeyMarket::new(&get_smalln(22, 2));
        assert_eq!(23, mm.most_bananas());
    }
}
