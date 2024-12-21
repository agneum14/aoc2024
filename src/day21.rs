#![allow(dead_code)]
use std::{collections::HashMap, iter::repeat};

use itertools::Itertools;
use lazy_static::lazy_static;

use crate::utils::{get_input, Point};

lazy_static! {
    static ref NUMERIC_BUTTONS: HashMap<char, Point> = HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);
    static ref DIRECTIONAL_BUTTONS: HashMap<char, Point> = HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);
}

struct Starship {
    codes: Vec<Vec<char>>,
}

impl Starship {
    fn new(input: &str) -> Self {
        let codes = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        Self { codes }
    }

    fn complexity(&self, directional_robots: usize) -> u64 {
        self.codes
            .iter()
            .map(|code| {
                let length = numeric_presses(code, directional_robots);
                let s = code.iter().collect::<String>();
                let s = s.trim_start_matches('0');
                let s = s.replace("A", "");
                let numeric = s.parse::<u64>().unwrap();
                length as u64 * numeric
            })
            .sum()
    }
}

fn numeric_presses(code: &Vec<char>, directional_robots: usize) -> usize {
    let mut arm = (3, 2);
    let mut length = 0;
    let mut cache = HashMap::new();
    for c in code {
        let target = NUMERIC_BUTTONS[c];
        length += if (arm.0 + target.0 - arm.0, arm.1) == (3, 0) {
            directional_presses(
                &presses(false, &target, &arm),
                0,
                &mut cache,
                directional_robots,
            )
        } else if (arm.0, arm.1 + target.1 - arm.1) == (3, 0) {
            directional_presses(
                &presses(true, &target, &arm),
                0,
                &mut cache,
                directional_robots,
            )
        } else {
            directional_presses(
                &presses(false, &target, &arm),
                0,
                &mut cache,
                directional_robots,
            )
            .min(directional_presses(
                &presses(true, &target, &arm),
                0,
                &mut cache,
                directional_robots,
            ))
        };
        arm = target;
    }
    length
}

fn directional_presses(
    code: &Vec<char>,
    depth: usize,
    cache: &mut HashMap<(String, usize), usize>,
    directional_robots: usize,
) -> usize {
    if depth == directional_robots {
        return code.len();
    }
    let s = code.iter().collect::<String>();
    if let Some(v) = cache.get(&(s.clone(), depth)) {
        return *v;
    }
    let mut arm = (0, 2);
    let mut length = 0;
    for c in code {
        let target = DIRECTIONAL_BUTTONS[c];
        length += if (arm.0 + target.0 - arm.0, arm.1) == (0, 0) {
            directional_presses(
                &presses(false, &target, &arm),
                depth + 1,
                cache,
                directional_robots,
            )
        } else if (arm.0, arm.1 + target.1 - arm.1) == (0, 0) {
            directional_presses(
                &presses(true, &target, &arm),
                depth + 1,
                cache,
                directional_robots,
            )
        } else {
            directional_presses(
                &presses(false, &target, &arm),
                depth + 1,
                cache,
                directional_robots,
            )
            .min(directional_presses(
                &presses(true, &target, &arm),
                depth + 1,
                cache,
                directional_robots,
            ))
        };
        arm = target
    }
    cache.insert((s, depth), length);
    length
}

fn presses(y_first: bool, target: &Point, arm: &Point) -> Vec<char> {
    let yd = target.0 - arm.0;
    let c = if yd >= 0 { 'v' } else { '^' };
    let ys = repeat(c).take(yd.abs() as usize);
    let xd = target.1 - arm.1;
    let c = if xd >= 0 { '>' } else { '<' };
    let xs = repeat(c).take(xd.abs() as usize);
    if y_first {
        ys.chain(xs).chain(['A']).collect_vec()
    } else {
        xs.chain(ys).chain(['A']).collect_vec()
    }
}

pub fn run() {
    let starship = Starship::new(&get_input(21));
    println!("Complexity: {}", starship.complexity(2));
    println!("Big Complexity: {}", starship.complexity(25));
}

#[cfg(test)]
mod tests {
    use crate::utils::get_small;

    use super::*;

    #[test]
    fn complexity() {
        let starship = Starship::new(&get_small(21));
        assert_eq!(126384, starship.complexity(2));
    }
}
