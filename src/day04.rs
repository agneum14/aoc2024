use std::fs::read_to_string;

use itertools::Itertools;

struct WordSearch {
    letters: Vec<Vec<char>>,
}

impl WordSearch {
    fn new(input: &str) -> Self {
        let letters = input.lines().map(|x| x.chars().collect_vec()).collect_vec();
        Self { letters }
    }

    fn get(&self, y: isize, x: isize) -> Option<char> {
        if y < 0 || x < 0 {
            return None;
        }
        let (y, x) = (y as usize, x as usize);
        match (self.letters.get(y), self.letters[0].get(x)) {
            (Some(_), Some(_)) => Some(self.letters[y][x]),
            _ => None,
        }
    }

    fn xmas_cords(&self, cords: &[(isize, isize)]) -> bool {
        cords
            .iter()
            .map(|x| self.get(x.0, x.1))
            .zip(['M', 'A', 'S'].into_iter())
            .find(|(a, b)| *a != Some(*b))
            .is_none()
    }

    fn xmas_count(&self) -> usize {
        let mut count = 0;
        for (y, line) in self.letters.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if *c != 'X' {
                    continue;
                }
                let (y, x) = (y as isize, x as isize);
                count += [
                    [(y, x - 1), (y, x - 2), (y, x - 3)],
                    [(y - 1, x), (y - 2, x), (y - 3, x)],
                    [(y, x + 1), (y, x + 2), (y, x + 3)],
                    [(y + 1, x), (y + 2, x), (y + 3, x)],
                    [(y - 1, x - 1), (y - 2, x - 2), (y - 3, x - 3)],
                    [(y - 1, x + 1), (y - 2, x + 2), (y - 3, x + 3)],
                    [(y + 1, x + 1), (y + 2, x + 2), (y + 3, x + 3)],
                    [(y + 1, x - 1), (y + 2, x - 2), (y + 3, x - 3)],
                ]
                .into_iter()
                .filter(|x| self.xmas_cords(x))
                .count();
            }
        }
        count
    }
}

pub fn run() {
    let input = read_to_string("inputs/day04.txt").unwrap();
    let ws = WordSearch::new(&input);
    println!("XMAS appearances: {}", ws.xmas_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xmas_count() {
        let ws = WordSearch::new(&read_to_string("inputs/day04_small.txt").unwrap());
        assert_eq!(18, ws.xmas_count())
    }
}
