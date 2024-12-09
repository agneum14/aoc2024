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

    fn antinodes(&self) -> usize {
        let mut antinodes = HashSet::new();
        for nodes in self.antennas.values() {
            for (fst, snd) in nodes.iter().tuple_combinations() {
                let y_diff = fst.0.abs_diff(snd.0) as isize;
                let ys = if fst.0 <= snd.0 {
                    (fst.0 - y_diff, snd.0 + y_diff)
                } else {
                    (fst.0 + y_diff, snd.0 - y_diff)
                };
                let x_diff = fst.1.abs_diff(snd.1) as isize;
                let xs = if fst.1 <= snd.1 {
                    (fst.1 - x_diff, snd.1 + x_diff)
                } else {
                    (fst.1 + x_diff, snd.1 - x_diff)
                };
                [(ys.0, xs.0), (ys.1, xs.1)]
                    .into_iter()
                    .filter(|(y, x)| *y >= 0 && *x >= 0 && *y < self.y_max && *x < self.x_max)
                    .for_each(|pos| {
                        antinodes.insert(pos);
                    });
            }
        }
        antinodes.len()
    }
}

pub fn run() {
    let input = read_to_string("inputs/day08.txt").unwrap();
    let map = Map::new(&input);
    println!("Unique antinode locations: {}", map.antinodes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn antinodes() {
        let map = Map::new(&read_to_string("inputs/day08_small.txt").unwrap());
        assert_eq!(14, map.antinodes())
    }
}
