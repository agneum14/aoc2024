use std::{collections::HashMap, fs::read_to_string};

struct Pluto {
    stones: HashMap<Stone, usize>,
}

impl Pluto {
    fn new(input: &str) -> Self {
        let mut stones = HashMap::new();

        for stone in input
            .split_whitespace()
            .map(|x| Stone::new(x.parse().unwrap()))
        {
            *stones.entry(stone).or_default() += 1;
        }

        Self { stones }
    }

    fn blink(&self, count: usize) -> usize {
        let mut stones = self.stones.clone();
        for _ in 0..count {
            let mut new_stones = HashMap::new();
            for (stone, count) in stones.iter() {
                for s in stone.blink() {
                    *new_stones.entry(s).or_default() += count;
                }
            }

            stones = new_stones;
        }
        stones.values().sum()
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Stone {
    engraving: u64,
}

impl Stone {
    fn new(engraving: u64) -> Self {
        Self { engraving }
    }

    fn blink(&self) -> Vec<Self> {
        if self.engraving == 0 {
            return Vec::from([Self::new(1)]);
        }

        let s = format!("{}", self.engraving);
        if s.len() % 2 == 0 {
            let mid = s.len() / 2;
            let l = s[..mid].parse().unwrap();
            let r = s[mid..].parse().unwrap();
            return Vec::from([Self::new(l), Self::new(r)]);
        }

        Vec::from([Self::new(self.engraving * 2024)])
    }
}

pub fn run() {
    let input = read_to_string("inputs/day11.txt").unwrap();
    let pluto = Pluto::new(&input);
    println!("Stones after 25 blinks: {}", pluto.blink(25));
    println!("Stones after 75 blinks: {}", pluto.blink(75));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blink25() {
        let pluto = Pluto::new("125 17");
        assert_eq!(55312, pluto.blink(25))
    }
}
