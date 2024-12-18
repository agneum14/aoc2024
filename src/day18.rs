use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

struct Computer {
    bytes: Vec<Point>,
    width: isize,
    start: Point,
    exit: Point,
}

impl Computer {
    fn new(input: &str, width: isize) -> Self {
        let bytes = input
            .lines()
            .map(|line| {
                let (x, y) = line
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect_tuple()
                    .unwrap();
                (y, x)
            })
            .collect_vec();
        Self {
            bytes,
            width,
            start: (0, 0),
            exit: (width - 1, width - 1),
        }
    }

    fn drop_bytes(&self, count: usize) -> HashMap<Point, Space> {
        let bytes = &self.bytes[0..count]
            .iter()
            .copied()
            .collect::<HashSet<Point>>();
        (0..self.width)
            .flat_map(|y| {
                (0..self.width).map(move |x| {
                    let p = (y, x);
                    let space = if bytes.contains(&p) {
                        Space::Corrupted
                    } else {
                        Space::Safe
                    };
                    (p, space)
                })
            })
            .collect::<HashMap<_, _>>()
    }

    fn steps(&self, count: usize) -> u32 {
        let grid = self.drop_bytes(count);
        let mut cache = HashMap::new();
        self.steps_inner(self.start, 0, &mut cache, count, &grid);
        *cache.get(&self.exit).unwrap()
    }

    fn steps_inner(
        &self,
        p: Point,
        steps: u32,
        cache: &mut HashMap<Point, u32>,
        count: usize,
        grid: &HashMap<Point, Space>,
    ) {
        let space = grid.get(&p);
        if space == None || space == Some(&Space::Corrupted) {
            return;
        }
        if let Some(v) = cache.get(&p) {
            if v <= &steps {
                return;
            }
        }
        cache.insert(p, steps);
        let (y, x) = p;
        for p in [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)] {
            self.steps_inner(p, steps + 1, cache, count, grid);
        }
    }
}

type Point = (isize, isize);

#[derive(PartialEq, Eq)]
enum Space {
    Safe,
    Corrupted,
}

pub fn run() {
    let computer = Computer::new(&read_to_string("inputs/day18.txt").unwrap(), 71);
    println!("Minimum steps: {}", computer.steps(1024))
}

#[cfg(test)]
mod tests {
    use crate::day18::*;

    #[test]
    fn minimum_steps() {
        let computer = Computer::new(&read_to_string("inputs/day18_small.txt").unwrap(), 7);
        assert_eq!(22, computer.steps(12))
    }
}
