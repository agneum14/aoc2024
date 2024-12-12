#![allow(dead_code)]
use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

struct Garden {
    grid: HashMap<(isize, isize), char>,
}

impl Garden {
    fn new(input: &str) -> Self {
        let mut grid = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (y as isize, x as isize);
                grid.insert(pos, c);
            }
        }
        Self { grid }
    }

    fn price(&self) -> u32 {
        let mut grid = self.grid.clone();
        let mut price = 0;
        while grid.len() > 0 {
            let pos = *grid.iter().next().unwrap().0;
            let c = grid[&pos];
            let (perimeter, area) = self.price_inner(&mut grid, pos, c);
            price += perimeter * area;
        }
        price
    }

    fn price_inner(
        &self,
        grid: &mut HashMap<(isize, isize), char>,
        pos: (isize, isize),
        target: char,
    ) -> (u32, u32) {
        if grid.get(&pos) != Some(&target) {
            return (0, 0);
        }
        grid.remove(&pos);

        let (y, x) = pos;
        let directions = [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];

        let initial_perimeter = directions
            .iter()
            .filter(|x| self.grid.get(x) != Some(&target))
            .count() as u32;
        let results = directions
            .iter()
            .map(|x| self.price_inner(grid, *x, target))
            .collect_vec();
        let perimeter = initial_perimeter + results.iter().map(|x| x.0).sum::<u32>();
        let area = 1 + results.iter().map(|x| x.1).sum::<u32>();
        (perimeter, area)
    }

    fn price_sides(&self) -> u32 {
        let mut grid = self.grid.clone();
        let mut price = 0;
        while grid.len() > 0 {
            let pos = *grid.iter().next().unwrap().0;
            let c = grid[&pos];
            let (sides, area) = self.price_sides_inner(&mut grid, pos, c);
            price += sides as u32 * area;
        }
        price
    }

    fn price_sides_inner(
        &self,
        grid: &mut HashMap<(isize, isize), char>,
        pos: (isize, isize),
        target: char,
    ) -> (usize, u32) {
        if grid.get(&pos) != Some(&target) {
            return (0, 0);
        }
        grid.remove(&pos);

        let (y, x) = pos;
        let directions = [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)];

        let initial_corners = self.corners(pos, target);
        let results = directions
            .iter()
            .map(|x| self.price_sides_inner(grid, *x, target))
            .collect_vec();
        let corners = initial_corners + results.iter().map(|x| x.0).sum::<usize>();
        let area = 1 + results.iter().map(|x| x.1).sum::<u32>();
        (corners, area)
    }

    fn corners(&self, pos: (isize, isize), target: char) -> usize {
        let (y, x) = pos;
        let surrounding = [
            ((y, x - 1), (y - 1, x)),
            ((y - 1, x), (y, x + 1)),
            ((y, x + 1), (y + 1, x)),
            ((y + 1, x), (y, x - 1)),
        ];

        let outer = surrounding
            .iter()
            .filter(|x| {
                self.grid.get(&x.0) != Some(&target) && self.grid.get(&x.1) != Some(&target)
            })
            .count();
        let inner = surrounding
            .iter()
            .filter(|a| {
                let y_diag = [a.0 .0, a.1 .0].into_iter().find(|b| *b != y).unwrap();
                let x_diag = [a.0 .1, a.1 .1].into_iter().find(|b| *b != x).unwrap();
                let diag = (y_diag, x_diag);
                self.grid.get(&a.0) == Some(&target)
                    && self.grid.get(&a.1) == Some(&target)
                    && self.grid.get(&diag) != Some(&target)
            })
            .count();
        outer + inner
    }
}

pub fn run() {
    let input = read_to_string("inputs/day12.txt").unwrap();
    let g = Garden::new(&input);
    println!("Total price: {}", g.price());
    println!("Total price counting sides: {}", g.price_sides());
}

#[cfg(test)]
mod tests {
    use crate::day12::*;

    #[test]
    fn total_price() {
        let g = Garden::new(&read_to_string("inputs/day12_small.txt").unwrap());
        assert_eq!(1930, g.price())
    }

    #[test]
    fn total_price_sides() {
        let g = Garden::new(&read_to_string("inputs/day12_small.txt").unwrap());
        assert_eq!(1206, g.price_sides())
    }
}
