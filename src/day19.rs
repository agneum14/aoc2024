#![allow(dead_code)]
use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

struct HotSprings {
    patterns: Vec<String>,
    designs: Vec<String>,
}

impl HotSprings {
    fn new(input: &str) -> Self {
        let data = input.split_once("\n\n").unwrap();
        let patterns = data.0.split(", ").map(|x| x.to_string()).collect_vec();
        let designs = data.1.lines().map(|x| x.to_string()).collect_vec();
        Self { patterns, designs }
    }

    fn design_possible(&self, partial_design: &str) -> bool {
        if partial_design.len() == 0 {
            return true;
        }
        let patterns = self
            .patterns
            .iter()
            .filter(|pat| partial_design.starts_with(*pat))
            .collect_vec();
        if patterns.len() == 0 {
            false
        } else {
            patterns
                .iter()
                .any(|pat| self.design_possible(&partial_design[pat.len()..]))
        }
    }

    fn count_possible_designs(&self) -> usize {
        self.designs
            .iter()
            .filter(|x| self.design_possible(x))
            .count()
    }

    fn different_ways(&self, partial_design: &str, ways: &mut HashMap<String, usize>) -> usize {
        if partial_design.len() == 0 {
            return 1;
        }
        if let Some(count) = ways.get(partial_design) {
            return *count;
        }
        let count = self
            .patterns
            .iter()
            .filter(|pat| partial_design.starts_with(*pat))
            .map(|pat| self.different_ways(&partial_design[pat.len()..], ways))
            .sum();
        ways.insert(partial_design.to_string(), count);
        count
    }

    fn sum_different_ways(&self) -> usize {
        self.designs
            .iter()
            .map(|design| self.different_ways(design, &mut HashMap::new()))
            .sum()
    }
}

pub fn run() {
    let hs = HotSprings::new(&read_to_string("inputs/day19.txt").unwrap());
    println!("Possible designs: {}", hs.count_possible_designs());
    println!("Sum different ways: {}", hs.sum_different_ways());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_possible() {
        let hs = HotSprings::new(&read_to_string("inputs/day19_small.txt").unwrap());
        assert_eq!(6, hs.count_possible_designs());
    }

    #[test]
    fn sum_different_ways() {
        let hs = HotSprings::new(&read_to_string("inputs/day19_small.txt").unwrap());
        assert_eq!(16, hs.sum_different_ways());
    }
}
