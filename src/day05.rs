#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

#[derive(Debug)]
struct Printer {
    rules: HashMap<u32, HashSet<u32>>,
    updates: Vec<Update>,
}

impl Printer {
    fn new(input: &str) -> Self {
        let data = input.split("\n\n").collect_vec();
        let rules = data[0]
            .lines()
            .map(|x| {
                let mut nums = x.split("|");
                (
                    nums.next().unwrap().parse().unwrap(),
                    nums.next().unwrap().parse().unwrap(),
                )
            })
            .fold(HashMap::<_, HashSet<_>>::new(), |mut acc, (fst, snd)| {
                acc.entry(fst).or_default().insert(snd);
                acc
            });

        let updates = data[1].lines().map(|x| Update::new(x)).collect_vec();

        Self { rules, updates }
    }

    fn correct_sum(&self) -> u32 {
        self.updates
            .iter()
            .filter(|x| x.correct(&self.rules))
            .map(|x| x.middle_page())
            .sum()
    }

    fn incorrect_sum(&self) -> u32 {
        self.updates
            .iter()
            .filter(|x| !x.correct(&self.rules))
            .map(|x| x.order(&self.rules).middle_page())
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<u32>,
}

impl Update {
    fn new(input: &str) -> Self {
        let pages = input.split(",").map(|x| x.parse().unwrap()).collect_vec();
        Self { pages }
    }

    fn correct(&self, rules: &HashMap<u32, HashSet<u32>>) -> bool {
        let page_set = self.pages.iter().collect::<HashSet<_>>();

        !rules
            .keys()
            .into_iter()
            .filter(|x| page_set.contains(x))
            .any(|x| {
                let after = &rules[x];
                self.pages
                    .iter()
                    .filter(|y| *y == x || after.contains(*y))
                    .next()
                    .unwrap()
                    != x
            })
    }

    fn middle_page(&self) -> u32 {
        self.pages[self.pages.len() / 2]
    }

    fn order(&self, rules: &HashMap<u32, HashSet<u32>>) -> Self {
        let mut pages = self.pages.clone();
        let page_set = self.pages.iter().collect::<HashSet<_>>();

        for left in rules.keys().into_iter().filter(|x| page_set.contains(x)) {
            let mut sequence = pages
                .iter()
                .enumerate()
                .filter(|(_, x)| *x == left || rules[left].contains(x));
            let fst = sequence.next().unwrap();
            if fst.1 != left {
                let old_idx = sequence.filter(|(_, x)| *x == left).next().unwrap().0 + 1;
                pages.insert(fst.0, *left);
                pages.remove(old_idx);
            }
        }

        Self { pages }
    }
}

pub fn run() {
    let input = read_to_string("inputs/day05.txt").unwrap();
    let printer = Printer::new(&input);
    println!("Correct sum: {}", printer.correct_sum());
    println!("Incorrect sum: {}", printer.incorrect_sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_sum() {
        let printer = Printer::new(&read_to_string("inputs/day05_small.txt").unwrap());
        assert_eq!(143, printer.correct_sum())
    }

    #[test]
    fn incorrect_sum() {
        let printer = Printer::new(&read_to_string("inputs/day05_small.txt").unwrap());
        assert_eq!(123, printer.incorrect_sum())
    }
}
