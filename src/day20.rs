#![allow(dead_code)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

// This method also works for inputs with multiple paths and dead ends. This is
// apparently unnecessary per the problem description but I already solved it
// this way so whatever.
struct Racetrack {
    grid: HashSet<Point>,
    edges: HashMap<Point, Vec<Point>>,
    start: Point,
    end: Point,
}

impl Racetrack {
    fn new(input: &str) -> Self {
        let mut grid = HashSet::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let point = (y as isize, x as isize);
                if c == 'S' {
                    start = point;
                } else if c == 'E' {
                    end = point;
                }
                if c != '#' {
                    grid.insert(point);
                }
            }
        }
        let edges = grid
            .iter()
            .map(|point| {
                let surrounding = point
                    .surrounding()
                    .into_iter()
                    .filter(|sp| grid.contains(&sp))
                    .collect_vec();
                (*point, surrounding)
            })
            .collect();
        Self {
            grid,
            edges,
            start,
            end,
        }
    }

    fn dijkstra(&self, start: Point) -> HashMap<Point, u32> {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut q = BinaryHeap::<(Reverse<u32>, Point)>::new();
        distances.insert(start, 0);
        q.push((Reverse(0), start));

        while let Some((Reverse(src_dist), src_node)) = q.pop() {
            if visited.contains(&src_node) {
                continue;
            }
            visited.insert(src_node);
            for dst_node in self.edges.get(&src_node).unwrap() {
                let new_dist = src_dist + 1;
                if new_dist < *distances.entry(*dst_node).or_insert(u32::MAX) {
                    distances.insert(*dst_node, new_dist);
                    q.push((Reverse(new_dist), *dst_node));
                }
            }
        }

        distances
    }

    fn count_cheats(&self, threshold: u32) -> usize {
        let from_start = self.dijkstra(self.start);
        let from_end = self.dijkstra(self.end);
        let record = from_start[&self.end];

        self.grid
            .iter()
            .filter(|point| from_start.contains_key(point))
            .map(|point| {
                point
                    .cheated()
                    .iter()
                    .filter(|sp| {
                        if let Some(end_dist) = from_end.get(sp) {
                            let dist = from_start[point] + 2 + end_dist;
                            if dist < record && record - dist >= threshold {
                                return true;
                            }
                        }
                        false
                    })
                    .count()
            })
            .sum()
    }

    fn count_big_cheats(&self, threshold: u32) -> usize {
        let from_start = self.dijkstra(self.start);
        let from_end = self.dijkstra(self.end);
        let record = from_start[&self.end];

        self.grid
            .iter()
            .filter(|point| from_start.contains_key(point))
            .map(|point| {
                point
                    .big_cheated()
                    .iter()
                    .filter(|(sp, cheated_steps)| {
                        if let Some(end_dist) = from_end.get(sp) {
                            let dist = from_start[point] + cheated_steps + end_dist;
                            if dist < record && record - dist >= threshold {
                                return true;
                            }
                        }
                        false
                    })
                    .count()
            })
            .sum()
    }
}

type Point = (isize, isize);

trait PointStuff: Sized {
    fn surrounding(&self) -> [Self; 4];
    fn cheated(&self) -> [Self; 8];
    fn big_cheated(&self) -> Vec<(Self, u32)>;
}

impl PointStuff for Point {
    fn surrounding(&self) -> [Self; 4] {
        let (y, x) = *self;
        [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)]
    }

    fn cheated(&self) -> [Self; 8] {
        let (y, x) = *self;
        [
            (y, x - 2),
            (y - 1, x - 1),
            (y - 2, x),
            (y - 1, x + 1),
            (y, x + 2),
            (y + 1, x + 1),
            (y + 2, x),
            (y + 1, x - 1),
        ]
    }

    fn big_cheated(&self) -> Vec<(Self, u32)> {
        let (y, x) = *self;
        (-20_isize..=20)
            .flat_map(|dy| {
                (-20_isize..=20).map(move |dx| {
                    let dist = dy.abs() as u32 + dx.abs() as u32;
                    ((y + dy, x + dx), dist)
                })
            })
            .filter(|(_, dist)| dist > &1 && dist <= &20)
            .collect_vec()
    }
}

pub fn run() {
    let racetrack = Racetrack::new(&read_to_string("inputs/day20.txt").unwrap());
    println!("Cheats: {}", racetrack.count_cheats(100));
    println!("Big cheats: {}", racetrack.count_big_cheats(100));
    (3 as isize, 2 as isize).big_cheated();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_cheats() {
        let racetrack = Racetrack::new(&read_to_string("inputs/day20_small.txt").unwrap());
        assert_eq!(44, racetrack.count_cheats(1))
    }

    #[test]
    fn count_big_cheats() {
        let racetrack = Racetrack::new(&read_to_string("inputs/day20_small.txt").unwrap());
        assert_eq!(285, racetrack.count_big_cheats(50))
    }
}
