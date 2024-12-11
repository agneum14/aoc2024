#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

struct TopographicMap {
    map: HashMap<(isize, isize), u32>,
    trailheads: Vec<(isize, isize)>,
}

impl TopographicMap {
    fn new(input: &str) -> Self {
        let mut map = HashMap::new();
        let mut trailheads = Vec::new();

        for (y, line) in input.lines().enumerate() {
            for (x, d) in line.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
                let pos = (y as isize, x as isize);
                if d == 0 {
                    trailheads.push(pos);
                }
                map.insert(pos, d);
            }
        }

        Self { map, trailheads }
    }

    fn trailhead_score_sum(&self) -> usize {
        self.trailheads
            .iter()
            .map(|x| self.trailhead_score(*x))
            .sum()
    }

    fn trailhead_score(&self, trailhead: (isize, isize)) -> usize {
        let mut nines = HashSet::<(isize, isize)>::new();
        self.trailhead_score_helper(&mut nines, trailhead, 0);
        nines.len()
    }

    fn trailhead_score_helper(
        &self,
        nines: &mut HashSet<(isize, isize)>,
        pos: (isize, isize),
        target: u32,
    ) {
        if self.map.get(&pos) != Some(&target) {
            return;
        }
        if target == 9 {
            nines.insert(pos);
            return;
        }

        let (y, x) = pos;
        let target = target + 1;
        self.trailhead_score_helper(nines, (y + 1, x), target);
        self.trailhead_score_helper(nines, (y - 1, x), target);
        self.trailhead_score_helper(nines, (y, x + 1), target);
        self.trailhead_score_helper(nines, (y, x - 1), target);
    }

    fn trailhead_rating_sum(&self) -> usize {
        self.trailheads
            .iter()
            .map(|x| self.trailhead_rating(*x))
            .sum()
    }

    fn trailhead_rating(&self, trailhead: (isize, isize)) -> usize {
        self.trailhead_rating_helper(trailhead, 0)
    }

    fn trailhead_rating_helper(&self, pos: (isize, isize), target: u32) -> usize {
        if self.map.get(&pos) != Some(&target) {
            return 0;
        }
        if target == 9 {
            return 1;
        }

        let (y, x) = pos;
        let target = target + 1;
        self.trailhead_rating_helper((y + 1, x), target)
            + self.trailhead_rating_helper((y - 1, x), target)
            + self.trailhead_rating_helper((y, x + 1), target)
            + self.trailhead_rating_helper((y, x - 1), target)
    }
}

pub fn run() {
    let input = read_to_string("inputs/day10.txt").unwrap();
    let tm = TopographicMap::new(&input);
    println!("Trailhead score sum: {}", tm.trailhead_score_sum());
    println!("Trailhead rating sum: {}", tm.trailhead_rating_sum());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trailhead_score_sum() {
        let tm = TopographicMap::new(&read_to_string("inputs/day10_small.txt").unwrap());
        assert_eq!(36, tm.trailhead_score_sum())
    }

    #[test]
    fn trailhead_rating_sum() {
        let tm = TopographicMap::new(&read_to_string("inputs/day10_small.txt").unwrap());
        assert_eq!(81, tm.trailhead_rating_sum())
    }
}
