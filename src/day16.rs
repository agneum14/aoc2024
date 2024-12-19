#![allow(dead_code)]
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
};

struct ReindeerMaze {
    start: Point,
    end: Point,
    edges: HashMap<Node, Vec<(Node, u32)>>,
}

impl ReindeerMaze {
    fn new(input: &str) -> Self {
        let mut start = (0, 0);
        let mut end = (0, 0);

        let mut points = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let p = (y as isize, x as isize);
                if c == 'S' {
                    start = p;
                } else if c == 'E' {
                    end = p;
                }
                if c != '#' {
                    points.insert(p);
                }
            }
        }

        let mut edges = HashMap::<_, Vec<_>>::new();
        for point in points.iter() {
            for d in Direction::all() {
                let node = (*point, d);
                let adj = point.adjacent(d);
                if points.contains(&adj) {
                    let new_node = (adj, d);
                    edges.entry(node).or_default().push((new_node, 1));
                }
                for d2 in [d.counterclockwise(), d.clockwise()] {
                    let new_node = (*point, d2);
                    edges.entry(node).or_default().push((new_node, 1000));
                }
            }
        }
        Self { start, end, edges }
    }

    fn dijkstra(&self) -> (HashMap<Node, Vec<Node>>, HashMap<Node, u32>, u32) {
        let mut previous_nodes = HashMap::<_, Vec<_>>::new();
        let mut distances = HashMap::new();
        let mut unvisited = self.edges.keys().collect::<HashSet<&Node>>();
        let mut q = BinaryHeap::new();
        let start = (self.start, Direction::Right);
        q.push((Reverse(0), start));

        while !unvisited.is_empty() {
            let (Reverse(src_score), src_node) = q.pop().unwrap();
            if !unvisited.contains(&src_node) {
                continue;
            }
            for (dst_node, dst_score) in self.edges.get(&src_node).unwrap() {
                let new_score = src_score + dst_score;
                q.push((Reverse(new_score), *dst_node));
                if new_score == *distances.entry(*dst_node).or_insert(u32::MAX) {
                    previous_nodes.entry(*dst_node).or_default().push(src_node);
                }
                if new_score < distances[dst_node] {
                    distances.insert(*dst_node, new_score);
                    previous_nodes.insert(*dst_node, [src_node].into());
                }
            }
            unvisited.remove(&src_node);
        }

        let lowest_score = *distances
            .iter()
            .filter(|x| x.0 .0 == self.end)
            .map(|x| x.1)
            .min()
            .unwrap();
        (previous_nodes, distances, lowest_score)
    }

    fn count_tiles(
        &self,
        previous_nodes: &HashMap<Node, Vec<Node>>,
        distances: &HashMap<Node, u32>,
        lowest_score: u32,
    ) -> usize {
        distances
            .iter()
            .filter(|x| x.0 .0 == self.end && *x.1 == lowest_score)
            .flat_map(|x| {
                let mut visited = Vec::new();
                ReindeerMaze::count_tiles_helper(*x.0, previous_nodes, &mut visited);
                visited
            })
            .map(|x| x.0)
            .collect::<HashSet<_>>()
            .len()
    }

    fn count_tiles_helper(
        cur: Node,
        previous_nodes: &HashMap<Node, Vec<Node>>,
        visited: &mut Vec<Node>,
    ) {
        visited.push(cur);
        if let Some(nodes) = previous_nodes.get(&cur) {
            for node in nodes {
                if !visited.contains(&node) {
                    ReindeerMaze::count_tiles_helper(*node, previous_nodes, visited);
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn all() -> [Self; 4] {
        [Self::Up, Self::Down, Self::Left, Self::Right]
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

type Node = (Point, Direction);

pub fn run() {
    let rm = ReindeerMaze::new(&read_to_string("inputs/day16.txt").unwrap());
    let (previous_nodes, distances, lowest_score) = rm.dijkstra();
    println!("Lowest score: {}", lowest_score);
    let tile_count = rm.count_tiles(&previous_nodes, &distances, lowest_score);
    println!("Tile count: {}", tile_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lowest_score() {
        let rm = ReindeerMaze::new(&read_to_string("inputs/day16_small.txt").unwrap());
        let (_, _, lowest_score) = rm.dijkstra();
        assert_eq!(7036, lowest_score)
    }

    #[test]
    fn lowest_score2() {
        let rm = ReindeerMaze::new(&read_to_string("inputs/day16_small2.txt").unwrap());
        let (_, _, lowest_score) = rm.dijkstra();
        assert_eq!(11048, lowest_score)
    }

    #[test]
    fn count_tiles() {
        let rm = ReindeerMaze::new(&read_to_string("inputs/day16_small.txt").unwrap());
        let (previous_nodes, distances, lowest_score) = rm.dijkstra();
        assert_eq!(
            45,
            rm.count_tiles(&previous_nodes, &distances, lowest_score)
        )
    }

    #[test]
    fn count_tiles2() {
        let rm = ReindeerMaze::new(&read_to_string("inputs/day16_small2.txt").unwrap());
        let (previous_nodes, distances, lowest_score) = rm.dijkstra();
        assert_eq!(
            64,
            rm.count_tiles(&previous_nodes, &distances, lowest_score)
        )
    }
}
