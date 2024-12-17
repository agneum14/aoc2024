#![allow(dead_code)]
use std::{collections::HashMap, fs::read_to_string};

type Point = (isize, isize);

struct ReindeerMaze {
    grid: HashMap<Point, Space>,
    start: Point,
    end: Point,
}

impl ReindeerMaze {
    fn new(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = (y as isize, x as isize);
                if c == '#' {
                    grid.insert(p, Space::Wall);
                } else if c == 'E' {
                    grid.insert(p, Space::End);
                    end = p;
                } else if c == 'S' {
                    start = p;
                }
            }
        }
        Self { grid, start, end }
    }

    fn lowest_score(&self) -> u32 {
        let mut cache = HashMap::new();
        self.lowest_score_helper(self.start, Direction::East, &mut cache, 0);
        *[
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
        .flat_map(|d| cache.get(&(self.end, d)))
        .min()
        .unwrap()
    }

    fn lowest_score_helper(
        &self,
        p: Point,
        facing: Direction,
        cache: &mut HashMap<(Point, Direction), u32>,
        score: u32,
    ) {
        let space = self.grid.get(&p);
        if space == Some(&Space::Wall) {
            return;
        }

        let lowest = *cache.entry((p, facing)).or_insert(u32::MAX);
        if score < lowest {
            cache.insert((p, facing), score);
        }
        if space == Some(&Space::End) || score >= lowest {
            return;
        }

        let (y, x) = p;
        let next_p = match facing {
            Direction::North => (y - 1, x),
            Direction::East => (y, x + 1),
            Direction::South => (y + 1, x),
            Direction::West => (y, x - 1),
        };
        self.lowest_score_helper(next_p, facing, cache, score + 1);
        self.lowest_score_helper(p, facing.rotate_clockwise(), cache, score + 1000);
        self.lowest_score_helper(p, facing.rotate_counterclockwise(), cache, score + 1000);
    }
}

#[derive(PartialEq, Eq)]
enum Space {
    Wall,
    End,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_clockwise(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn rotate_counterclockwise(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

pub fn run() {
    let input = read_to_string("inputs/day16.txt").unwrap();
    let rm = ReindeerMaze::new(&input);
    println!("Lowest score: {}", rm.lowest_score());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_score() {
        let rm = ReindeerMaze::new(&read_to_string("inputs/day16_small.txt").unwrap());
        assert_eq!(7036, rm.lowest_score())
    }

    #[test]
    fn lowest_score2() {
        let rm = ReindeerMaze::new(&read_to_string("inputs/day16_small2.txt").unwrap());
        assert_eq!(11048, rm.lowest_score())
    }
}
