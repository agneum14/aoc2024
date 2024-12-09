#![allow(dead_code)]
use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

struct Map {
    antennas: HashMap<char, Vec<(isize, isize)>>,
    y_max: isize,
    x_max: isize,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut antennas = HashMap::<_, Vec<_>>::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let pos = (y as isize, x as isize);
                    antennas.entry(c).or_default().push(pos);
                }
            }
        }
        let y_max = input.lines().count() as isize;
        let x_max = input.lines().next().unwrap().chars().count() as isize;
        Self {
            antennas,
            y_max,
            x_max,
        }
    }

    fn exists(&self, pos: &(isize, isize)) -> bool {
        let (y, x) = pos;
        *y >= 0 && *x >= 0 && *y < self.y_max && *x < self.x_max
    }

    fn antinodes(&self) -> usize {
        let mut antinodes = HashSet::new();
        for nodes in self.antennas.values() {
            for (a, b) in nodes.iter().tuple_combinations() {
                let vba = (a.0 - b.0, a.1 - b.1);
                for p in [(a.0 + vba.0, a.1 + vba.1), (b.0 - vba.0, b.1 - vba.1)]
                    .into_iter()
                    .filter(|pos| self.exists(pos))
                {
                    antinodes.insert(p);
                }
            }
        }
        antinodes.len()
    }

    fn harmonic_antinodes(&self) -> usize {
        let mut antinodes = HashSet::new();
        for nodes in self.antennas.values() {
            for (a, b) in nodes.iter().tuple_combinations() {
                let vba = (a.0 - b.0, a.1 - b.1);
                self.harmonic_half(*a, vba)
                    .into_iter()
                    .chain(self.harmonic_half(*b, (vba.0 * -1, vba.1 * -1)))
                    .for_each(|p| {
                        antinodes.insert(p);
                    });
            }
        }
        antinodes.len()
    }

    fn harmonic_half(&self, p: (isize, isize), v: (isize, isize)) -> HashSet<(isize, isize)> {
        let mut points = HashSet::from([p]);
        let mut p = p;
        loop {
            p = (p.0 + v.0, p.1 + v.1);
            if !self.exists(&p) {
                break;
            }
            points.insert(p);
        }
        points
    }
}

pub fn run() {
    let input = read_to_string("inputs/day08.txt").unwrap();
    let map = Map::new(&input);
    println!("Unique antinode locations: {}", map.antinodes());
    println!(
        "Harmonic unique antinode locations: {}",
        map.harmonic_antinodes()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn antinodes() {
        let map = Map::new(&read_to_string("inputs/day08_small.txt").unwrap());
        assert_eq!(14, map.antinodes())
    }

    #[test]
    fn harmonic_antinodes() {
        let map = Map::new(&read_to_string("inputs/day08_small.txt").unwrap());
        assert_eq!(34, map.harmonic_antinodes())
    }
}
