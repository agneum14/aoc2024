#![allow(dead_code)]
use std::{collections::HashSet, fs::read_to_string};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"-{0,1}\d+").unwrap();
}

struct Bathroom {
    robots: Vec<Robot>,
    x_max: isize,
    y_max: isize,
}

impl Bathroom {
    fn new(input: &str, x_max: isize, y_max: isize) -> Self {
        let robots = input.lines().map(|line| Robot::new(line)).collect_vec();
        Self {
            robots,
            x_max,
            y_max,
        }
    }

    fn pace(&self) -> Vec<Robot> {
        let mut robots = self.robots.clone();
        for _ in 0..100 {
            robots = robots
                .iter()
                .map(|r| r.pace(self.x_max, self.y_max))
                .collect()
        }
        robots
    }

    fn safety_factor(&self) -> usize {
        let robots = self.pace();
        let xs = self.x_max / 2;
        let xl = xs + 1;
        let ys = self.y_max / 2;
        let yl = ys + 1;
        let (q1, q2, q3, q4) =
            robots
                .iter()
                .fold((0, 0, 0, 0), |(mut q1, mut q2, mut q3, mut q4), r| {
                    let (x, y) = r.pos;
                    if (0..ys).contains(&y) {
                        if (xl..self.x_max).contains(&x) {
                            q1 += 1;
                        } else if (0..xs).contains(&x) {
                            q2 += 1;
                        }
                    } else if (yl..self.y_max).contains(&y) {
                        if (xl..self.x_max).contains(&x) {
                            q4 += 1;
                        } else if (0..xs).contains(&x) {
                            q3 += 1;
                        }
                    }
                    (q1, q2, q3, q4)
                });
        q1 * q2 * q3 * q4
    }

    fn find_tree(&self) -> usize {
        let mut robots = self.robots.clone();
        (0..)
            .find(|_| {
                let cond =
                    robots.iter().map(|r| r.pos).collect::<HashSet<_>>().len() == robots.len();
                robots = robots
                    .iter()
                    .map(|r| r.pace(self.x_max, self.y_max))
                    .collect();
                cond
            })
            .unwrap()
    }

    fn display(&self, seconds: usize) {
        let mut robots = self.robots.clone();
        for _ in 0..seconds {
            robots = robots
                .iter()
                .map(|r| r.pace(self.x_max, self.y_max))
                .collect();
        }
        let ps = robots.iter().map(|r| r.pos).collect::<HashSet<_>>();
        for y in 0..self.y_max {
            for x in 0..self.x_max {
                if ps.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }
}

#[derive(Clone, Copy)]
struct Robot {
    pos: (isize, isize),
    vel: (isize, isize),
}

impl Robot {
    fn new(input: &str) -> Self {
        let ns = RE
            .find_iter(input)
            .map(|n| n.as_str().parse().unwrap())
            .collect_vec();
        let pos = (ns[0], ns[1]);
        let vel = (ns[2], ns[3]);
        Self { pos, vel }
    }

    fn pace(&self, x_max: isize, y_max: isize) -> Self {
        let wrap = |cur: isize, vel: isize, max: isize| -> isize {
            let mut r = cur + vel;
            if r < 0 {
                r += max;
            }
            if r >= max {
                r -= max;
            }
            r
        };
        let pos = (
            wrap(self.pos.0, self.vel.0, x_max),
            wrap(self.pos.1, self.vel.1, y_max),
        );
        Self { pos, vel: self.vel }
    }
}

pub fn run() {
    let bathroom = Bathroom::new(&read_to_string("inputs/day14.txt").unwrap(), 101, 103);
    println!("Safety factor: {}", bathroom.safety_factor());
    let seconds = bathroom.find_tree();
    println!("Tree time: {}", seconds);
    bathroom.display(seconds);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safety_factor() {
        let bathroom = Bathroom::new(&read_to_string("inputs/day14_small.txt").unwrap(), 11, 7);
        assert_eq!(12, bathroom.safety_factor())
    }
}
