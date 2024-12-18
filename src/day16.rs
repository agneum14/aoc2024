use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
};

use itertools::Itertools;

struct ReindeerMaze {
    start: Point,
    end: Point,
    edges: HashMap<Node, Vec<(Node, i32)>>,
}

impl ReindeerMaze {
    fn new(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut edges = HashMap::new();

        let mut grid = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = (y as isize, x as isize);
                if c == 'S' {
                    start = p;
                } else if c == 'E' {
                    end = p;
                }
                let c = if c == '#' { '#' } else { '.' };
                grid.insert(p, c);
            }
        }

        // vertices are the start, end, and points with turns
        let verts = grid
            .iter()
            .filter(|(p, c)| {
                if **c != '.' {
                    return false;
                }
                let surrounding = Direction::all()
                    .into_iter()
                    .filter(|d| grid.get(&p.adjacent(*d)) == Some(&'.'))
                    .collect_vec();
                if surrounding.len() < 2 {
                    return false;
                }
                let fst = surrounding[0];
                surrounding
                    .into_iter()
                    .filter(|d| *d != fst.opposite())
                    .count()
                    > 1
            })
            .map(|(p, _)| p)
            .chain([start, end].iter())
            .copied()
            .collect::<HashSet<Point>>();

        // retain only vertices and walls to find edges
        let grid = grid
            .into_iter()
            .filter(|(p, c)| *c == '#' || verts.contains(p))
            .collect::<HashMap<Point, char>>();

        // find all edges
        for v in verts {
            for d in Direction::all().into_iter() {
                let node = (v, d);

                // edges that can be reached by turning
                let turns = [d.clockwise(), d.counterclockwise()]
                    .into_iter()
                    .map(|d2| ((v, d2), 1000_i32));

                // edges that can be reached by walking
                let nearest = [match d {
                    Direction::Up => grid
                        .iter()
                        .filter(|(p, _)| p.1 == v.1 && p.0 < v.0)
                        .max_by_key(|(p, _)| p.0)
                        .map(|x| (x, v.0 - x.0 .0)),
                    Direction::Down => grid
                        .iter()
                        .filter(|(p, _)| p.1 == v.1 && p.0 > v.0)
                        .min_by_key(|(p, _)| p.0)
                        .map(|x| (x, x.0 .0 - v.0)),
                    Direction::Left => grid
                        .iter()
                        .filter(|(p, _)| p.0 == v.0 && p.1 < v.1)
                        .max_by_key(|(p, _)| p.1)
                        .map(|x| (x, v.1 - x.0 .1)),
                    Direction::Right => grid
                        .iter()
                        .filter(|(p, _)| p.0 == v.0 && p.1 > v.1)
                        .min_by_key(|(p, _)| p.1)
                        .map(|x| (x, x.0 .1 - v.1)),
                }]
                .into_iter()
                .filter_map(|x| x)
                .map(|(x, w)| (x, w as i32))
                .filter(|((_, c), _)| **c == '.')
                .map(|((p, _), w)| ((*p, d), w));

                let node_edges = turns.chain(nearest).collect_vec();
                edges.insert(node, node_edges);
            }
        }

        Self { start, end, edges }
    }

    fn lowest_score(&self) -> i32 {
        let mut unvisited = self.edges.keys().copied().collect::<HashSet<Node>>();
        let start = (self.start, Direction::Right);
        let mut distance_heap = unvisited
            .iter()
            .map(|n| (Reverse(i32::MAX), *n))
            .collect::<BinaryHeap<_>>();
        let mut distances = unvisited
            .iter()
            .map(|n| (*n, i32::MAX))
            .collect::<HashMap<Node, i32>>();
        distance_heap.push((Reverse(0), start));
        distances.insert(start, 0);

        while !unvisited.is_empty() {
            let (Reverse(src_w), src): (Reverse<i32>, Node) = distance_heap.pop().unwrap();

            for (dst, edge_w) in self.edges.get(&src).unwrap() {
                let new_w = src_w + edge_w;
                if new_w < distances[dst] {
                    distances.insert(*dst, new_w);
                    distance_heap.push((Reverse(new_w), *dst));
                }
            }

            unvisited.remove(&src);
        }

        distances
            .iter()
            .filter(|x| x.0 .0 == self.end)
            .map(|x| *x.1)
            .min()
            .unwrap()
    }
}

type Point = (isize, isize);

trait PointStuff {
    fn adjacent(&self, d: Direction) -> Self;
}

impl PointStuff for Point {
    fn adjacent(&self, d: Direction) -> Self {
        let (y, x) = *self;
        match d {
            Direction::Up => (y - 1, x),
            Direction::Down => (y + 1, x),
            Direction::Left => (y, x - 1),
            Direction::Right => (y, x + 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn counterclockwise(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn all() -> [Self; 4] {
        [Self::Up, Self::Right, Self::Down, Self::Left]
    }
}

type Node = (Point, Direction);

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
