use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

struct Lists {
    fst: Vec<u64>,
    snd: Vec<u64>,
}

impl Lists {
    fn new(input: &str) -> Self {
        let (fst, snd) = input.lines().fold(
            (Vec::<u64>::new(), Vec::<u64>::new()),
            |(mut fst, mut snd), x| {
                let data = x.split_whitespace().collect::<Vec<_>>();
                fst.push(data[0].parse().unwrap());
                snd.push(data[1].parse().unwrap());
                (fst, snd)
            },
        );
        Self { fst, snd }
    }

    fn total_distance(&self) -> u64 {
        self.fst
            .iter()
            .sorted()
            .zip(self.snd.iter().sorted())
            .map(|(x, y)| x.abs_diff(*y))
            .sum()
    }

    fn similarity_score(&self) -> u64 {
        let frequencies = self
            .snd
            .iter()
            .fold(HashMap::<u64, usize>::new(), |mut freqs, x| {
                *freqs.entry(*x).or_default() += 1;
                freqs
            });
        self.fst
            .iter()
            .map(|x| x * *frequencies.get(x).unwrap_or(&0) as u64)
            .sum()
    }
}

pub fn run() {
    let input = read_to_string("inputs/day01.txt").unwrap();
    let lists = Lists::new(&input);
    println!("Total distance: {}", lists.total_distance());
    println!("Similarity score: {}", lists.similarity_score());
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::Lists;

    #[test]
    fn total_distance() {
        let lists = Lists::new(&read_to_string("inputs/day01_small.txt").unwrap());
        assert_eq!(11, lists.total_distance())
    }

    #[test]
    fn similarity_score() {
        let lists = Lists::new(&read_to_string("inputs/day01_small.txt").unwrap());
        assert_eq!(31, lists.similarity_score())
    }
}
