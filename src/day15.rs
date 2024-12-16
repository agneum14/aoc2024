#![allow(dead_code)]
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
        if self.shift_inner(target, direction, false, false) {
            let _ = self.shift_inner(target, direction, false, true);
            self.robot = target;
        }
    }

    fn shift_inner(
        &mut self,
        pos: (isize, isize),
        direction: Direction,
        inner: bool,
        shift: bool,
    ) -> bool {
        let square = self.grid.get(&pos);
        match square {
            None => true,
            Some(&Square::Wall) => false,
            Some(&Square::Box) => {
                let target = direction.target(pos);
                let foo = self.shift_inner(target, direction, false, shift);
                if foo && shift {
                    self.grid.remove(&pos);
                    self.grid.insert(target, Square::Box);
                }
                foo
            }
            Some(&Square::LeftBox) | Some(&Square::RightBox) => {
                let square = *square.unwrap();
                let target = direction.target(pos);
                let foo;
                if inner {
                    foo = self.shift_inner(target, direction, false, shift);
                } else {
                    let opposite = square.opposite();
                    if direction == opposite {
                        foo = self.shift_inner(target, direction, true, shift)
                    } else {
                        foo = self.shift_inner(target, direction, false, shift)
                            && self.shift_inner(opposite.target(pos), direction, true, shift)
                    }
                }
                if foo && shift {
                    self.grid.remove(&pos);
                    self.grid.insert(target, square);
                }
                foo
            }
        }
    }

    fn gps_sum(&self) -> isize {
        self.grid
            .iter()
            .filter(|(_, square)| **square == Square::Box || **square == Square::LeftBox)
            .map(|(pos, _)| pos.0 * 100 + pos.1)
            .sum()
    }

    fn widen(&mut self) {
        let mut new = HashMap::new();
        for (pos, space) in self.grid.iter() {
            let (fst, snd) = match space {
                Square::Wall => (Square::Wall, Square::Wall),
                Square::Box => (Square::LeftBox, Square::RightBox),
                _ => unreachable!(),
            };
            let pos = (pos.0, pos.1 * 2);
            new.insert(pos, fst);
            new.insert((pos.0, pos.1 + 1), snd);
        }
        self.grid = new;
        self.robot = (self.robot.0, self.robot.1 * 2)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Square {
    Wall,
    Box,
    LeftBox,
    RightBox,
}

impl Square {
    fn opposite(&self) -> Direction {
        match self {
            Self::LeftBox => Direction::Right,
            Self::RightBox => Direction::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    let mut warehouse = Warehouse::new(&input);
    let mut wide = warehouse.clone();
    wide.widen();
    wide.simulate();
    warehouse.simulate();
    println!("GPS sum: {}", warehouse.gps_sum());
    println!("Wide GPS sum: {}", wide.gps_sum());
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::Warehouse;

    #[test]
    fn gps_sum() {
        let mut warehouse = Warehouse::new(&read_to_string("inputs/day15_small.txt").unwrap());
        warehouse.simulate();
        assert_eq!(10092, warehouse.gps_sum())
    }

    #[test]
    fn wide_gps_sum() {
        let mut warehouse = Warehouse::new(&read_to_string("inputs/day15_small.txt").unwrap());
        warehouse.widen();
        warehouse.simulate();
        assert_eq!(9021, warehouse.gps_sum())
    }
}
