#![allow(dead_code)]
use itertools::Itertools;

use crate::utils::get_input;

const INVALID_HEIGHT: i32 = 6;
const WIDTH: usize = 5;
const HEIGHT: usize = 7;

struct Office {
    keys: Vec<Vec<i32>>,
    locks: Vec<Vec<i32>>,
}

impl Office {
    fn new(input: &str) -> Self {
        let mut keys = Vec::new();
        let mut locks = Vec::new();
        for block in input.split("\n\n") {
            let mut grid = (0..WIDTH)
                .map(|_| (0..HEIGHT).map(|_| '0').collect_vec())
                .collect_vec();
            for (y, line) in block.lines().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    grid[x][y] = c;
                }
            }
            let heights = grid
                .iter()
                .map(|line| line.iter().filter(|c| **c == '#').count() as i32 - 1)
                .collect_vec();
            if grid[0][0] == '#' {
                locks.push(heights);
            } else {
                keys.push(heights);
            }
        }
        Self { keys, locks }
    }

    fn unique_pairs(&self) -> usize {
        let mut count = 0;
        for lock in self.locks.iter() {
            for key in self.keys.iter() {
                if lock
                    .iter()
                    .zip(key.iter())
                    .map(|(l, k)| *l + *k)
                    .all(|height| height < INVALID_HEIGHT)
                {
                    count += 1;
                }
            }
        }
        count
    }
}

pub fn run() {
    let office = Office::new(&get_input(25));
    println!("Unique pairs: {}", office.unique_pairs())
}

#[cfg(test)]
mod tests {
    use crate::utils::get_small;

    use super::Office;

    #[test]
    fn unique_pairs() {
        let office = Office::new(&get_small(25));
        assert_eq!(3, office.unique_pairs())
    }
}
