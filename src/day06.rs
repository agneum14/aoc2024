use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

struct Map {
    grid: HashMap<(isize, isize), Space>,
    start: (isize, isize),
}

impl Map {
    fn new(input: &str) -> Self {
        let mut start = (0, 0);
        let mut grid = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let space = match c {
                    '#' => Space::Obstacle,
                    '^' => {
                        start = (y as isize, x as isize);
                        Space::Ground
                    }
                    _ => Space::Ground,
                };
                grid.insert((y as isize, x as isize), space);
            }
        }
        Self { grid, start }
    }

    fn distinct_positions(&self) -> usize {
        let mut visited = HashSet::from([self.start]);
        let (y, x) = self.start;
        self.walk(&mut visited, Direction::North, y, x)
    }

    fn walk(
        &self,
        visited: &mut HashSet<(isize, isize)>,
        direction: Direction,
        y: isize,
        x: isize,
    ) -> usize {
        if self.grid.get(&(y, x)).is_none() {
            return visited.len();
        }
        visited.insert((y, x));

        let facing = match direction {
            Direction::North => (y - 1, x),
            Direction::East => (y, x + 1),
            Direction::South => (y + 1, x),
            Direction::West => (y, x - 1),
        };
        if self.grid.get(&facing) == Some(&Space::Obstacle) {
            return self.walk(visited, direction.rotate(), y, x);
        }
        let (y, x) = facing;
        return self.walk(visited, direction, y, x);
    }
}

#[derive(PartialEq, Eq)]
enum Space {
    Ground,
    Obstacle,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self) -> Direction {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

pub fn run() {
    let input = read_to_string("inputs/day06.txt").unwrap();
    let map = Map::new(&input);
    println!("Distinct positions: {}", map.distinct_positions())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_positions() {
        let map = Map::new(&read_to_string("inputs/day06_small.txt").unwrap());
        assert_eq!(41, map.distinct_positions())
    }
}
