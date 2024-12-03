use std::fs::read_to_string;

use itertools::Itertools;

struct Lists {
    fst: Vec<u32>,
    snd: Vec<u32>,
}

impl Lists {
    fn new(input: &str) -> Self {
        let (fst, snd) = input.lines().fold(
            (Vec::<u32>::new(), Vec::<u32>::new()),
            |(mut fst, mut snd), x| {
                let data = x.split_whitespace().collect::<Vec<_>>();
                fst.push(data[0].parse().unwrap());
                snd.push(data[1].parse().unwrap());
                (fst, snd)
            },
        );
        Self { fst, snd }
    }

    fn total_distance(&self) -> u32 {
        self.fst
            .iter()
            .sorted()
            .zip(self.snd.iter().sorted())
            .map(|(x, y)| x.abs_diff(*y))
            .sum()
    }
}

pub fn run() {
    let input = read_to_string("inputs/day01.txt").unwrap();
    let lists = Lists::new(&input);
    println!("Total distance: {}", lists.total_distance());
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
}
