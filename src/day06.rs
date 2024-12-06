use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    isize,
};

struct Map {
    grid: HashMap<(isize, isize), Space>,
    start: (isize, isize),
    y_max: isize,
    x_max: isize,
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
        let y_max = input.lines().count() as isize;
        let x_max = input.lines().next().unwrap().chars().count() as isize;
        Self {
            grid,
            start,
            y_max,
            x_max,
        }
    }

    fn distinct_positions(&self) -> usize {
        let mut visited = HashSet::from([self.start]);
        self.walk(&mut visited, Direction::North, self.start)
    }

    fn different_obstructions(&self) -> usize {
        (0..self.y_max)
            .flat_map(|y| (0..self.x_max).map(move |x| (y, x)))
            .filter(|pos| *pos != self.start && self.grid.get(pos) != Some(&Space::Obstacle))
            .map(|pos| self.cycles(pos))
            .filter(|x| *x)
            .count()
    }

    fn walk(
        &self,
        visited: &mut HashSet<(isize, isize)>,
        direction: Direction,
        position: (isize, isize),
    ) -> usize {
        if self.grid.get(&position).is_none() {
            return visited.len();
        }
        visited.insert(position);

        let facing = Map::facing(direction, position);
        if self.grid.get(&facing) == Some(&Space::Obstacle) {
            return self.walk(visited, direction.rotate(), position);
        }
        return self.walk(visited, direction, facing);
    }

    fn cycles(&self, obstacle: (isize, isize)) -> bool {
        return self.inner_cycles(&mut HashSet::new(), obstacle, Direction::North, self.start);
    }

    fn inner_cycles(
        &self,
        visited: &mut HashSet<((isize, isize), Direction)>,
        obstacle: (isize, isize),
        direction: Direction,
        position: (isize, isize),
    ) -> bool {
        if self.grid.get(&position).is_none() {
            return false;
        }
        if visited.contains(&(position, direction)) {
            return true;
        }
        visited.insert((position, direction));

        let facing = Map::facing(direction, position);
        if self.grid.get(&facing) == Some(&Space::Obstacle) || facing == obstacle {
            return self.inner_cycles(visited, obstacle, direction.rotate(), position);
        }
        self.inner_cycles(visited, obstacle, direction, facing)
    }

    fn facing(direction: Direction, position: (isize, isize)) -> (isize, isize) {
        let (y, x) = position;
        match direction {
            Direction::North => (y - 1, x),
            Direction::East => (y, x + 1),
            Direction::South => (y + 1, x),
            Direction::West => (y, x - 1),
        }
    }
}

#[derive(PartialEq, Eq)]
enum Space {
    Ground,
    Obstacle,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
    println!("Distinct positions: {}", map.distinct_positions());
    println!("Different obstructions: {}", map.different_obstructions())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinct_positions() {
        let map = Map::new(&read_to_string("inputs/day06_small.txt").unwrap());
        assert_eq!(41, map.distinct_positions())
    }

    #[test]
    fn positive_cycle() {
        let map = Map::new(&read_to_string("inputs/day06_small.txt").unwrap());
        assert_eq!(true, map.cycles((6, 3)))
    }

    #[test]
    fn negative_cycle() {
        let map = Map::new(&read_to_string("inputs/day06_small.txt").unwrap());
        assert_eq!(false, map.cycles((6, 2)))
    }

    #[test]
    fn different_obstructions() {
        let map = Map::new(&read_to_string("inputs/day06_small.txt").unwrap());
        assert_eq!(6, map.different_obstructions())
    }
}
