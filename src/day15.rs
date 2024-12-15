use std::{collections::HashMap, fs::read_to_string};

use itertools::Itertools;

#[derive(Clone)]
struct Warehouse {
    grid: HashMap<(isize, isize), Square>,
    robot: (isize, isize),
    moves: Vec<Direction>,
}

impl Warehouse {
    fn new(input: &str) -> Self {
        let mut data = input.split("\n\n");
        let mut grid = HashMap::new();
        let mut robot = (0, 0);
        for (y, line) in data.next().unwrap().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (y as isize, x as isize);
                if c == '#' {
                    grid.insert(pos, Square::Wall);
                } else if c == 'O' {
                    grid.insert(pos, Square::Box);
                } else if c == '@' {
                    robot = pos;
                }
            }
        }
        let moves = data
            .next()
            .unwrap()
            .replace("\n", "")
            .chars()
            .map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => unreachable!(),
            })
            .collect_vec();
        Self { grid, robot, moves }
    }

    fn simulate(&mut self) {
        for direction in self.moves.clone().iter() {
            self.shift(*direction);
        }
    }

    fn shift(&mut self, direction: Direction) {
        let target = direction.target(self.robot);
        if self.shift_inner(target, direction) {
            self.robot = target;
        }
    }

    fn shift_inner(&mut self, pos: (isize, isize), direction: Direction) -> bool {
        let square = self.grid.get(&pos);
        match square {
            None => true,
            Some(&Square::Wall) => false,
            Some(&Square::Box) => {
                let target = direction.target(pos);
                let foo = self.shift_inner(target, direction);
                if foo {
                    self.grid.remove(&pos);
                    self.grid.insert(target, Square::Box);
                }
                foo
            }
        }
    }

    fn gps_sum(&mut self) -> isize {
        self.simulate();
        self.grid
            .iter()
            .filter(|(_, square)| **square == Square::Box)
            .map(|(pos, _)| pos.0 * 100 + pos.1)
            .sum()
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Square {
    Wall,
    Box,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn target(&self, pos: (isize, isize)) -> (isize, isize) {
        let (y, x) = pos;
        match self {
            Self::Up => (y - 1, x),
            Self::Down => (y + 1, x),
            Self::Left => (y, x - 1),
            Self::Right => (y, x + 1),
        }
    }
}

pub fn run() {
    let input = read_to_string("inputs/day15.txt").unwrap();
    let warehouse = Warehouse::new(&input);
    println!("GPS sum: {}", warehouse.clone().gps_sum());
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::Warehouse;

    #[test]
    fn gps_sum() {
        let mut warehouse = Warehouse::new(&read_to_string("inputs/day15_small.txt").unwrap());
        assert_eq!(10092, warehouse.gps_sum())
    }
}
