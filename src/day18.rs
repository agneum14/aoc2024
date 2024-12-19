#![allow(dead_code)]
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
    count: usize,
}

impl Computer {
    fn new(input: &str, width: isize, count: usize) -> Self {
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
            count,
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

    fn steps(&self) -> u32 {
        let grid = self.drop_bytes(self.count);
        let mut cache = HashMap::new();
        self.steps_inner(self.start, 0, &mut cache, &grid);
        *cache.get(&self.exit).unwrap()
    }

    fn steps_inner(
        &self,
        p: Point,
        steps: u32,
        cache: &mut HashMap<Point, u32>,
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
            self.steps_inner(p, steps + 1, cache, grid);
        }
    }

    fn reachable(&self, grid: &HashMap<Point, Space>) -> bool {
        self.reachable_inner(self.start, &mut HashSet::new(), grid)
    }

    fn reachable_inner(
        &self,
        p: Point,
        visited: &mut HashSet<Point>,
        grid: &HashMap<Point, Space>,
    ) -> bool {
        let space = grid.get(&p);
        if space == None || space == Some(&Space::Corrupted) || visited.contains(&p) {
            return false;
        }
        if p == self.exit {
            return true;
        }
        visited.insert(p);
        let (y, x) = p;
        [(y - 1, x), (y + 1, x), (y, x - 1), (y, x + 1)]
            .into_iter()
            .any(|p| self.reachable_inner(p, visited, grid))
    }

    fn first_byte(&self) -> Point {
        let mut low = self.count;
        let mut high = self.bytes.len();
        while high - low > 1 {
            let mid = (high + low) / 2;
            if self.reachable(&self.drop_bytes(mid)) {
                low = mid;
            } else {
                high = mid;
            }
        }
        self.bytes[low]
    }
}

type Point = (isize, isize);

#[derive(PartialEq, Eq)]
enum Space {
    Safe,
    Corrupted,
}

pub fn run() {
    let computer = Computer::new(&read_to_string("inputs/day18.txt").unwrap(), 71, 1024);
    println!("Minimum steps: {}", computer.steps());
    let fb = computer.first_byte();
    println!("First byte: {},{}", fb.1, fb.0);
}

#[cfg(test)]
mod tests {
    use crate::day18::*;

    #[test]
    fn minimum_steps() {
        let computer = Computer::new(&read_to_string("inputs/day18_small.txt").unwrap(), 7, 12);
        assert_eq!(22, computer.steps())
    }

    #[test]
    fn first_byte() {
        let computer = Computer::new(&read_to_string("inputs/day18_small.txt").unwrap(), 7, 12);
        assert_eq!((1, 6), computer.first_byte())
    }
}
