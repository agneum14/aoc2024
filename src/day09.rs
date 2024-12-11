use std::fs::read_to_string;

use itertools::Itertools;

struct Computer {
    disk: Vec<Blop>,
}

impl Computer {
    fn new(input: &str) -> Self {
        let mut id = 0;
        let disk = input
            .trim()
            .chars()
            .chunks(2)
            .into_iter()
            .flat_map(|mut x| {
                let file = Blop {
                    kind: BlopKind::File(id),
                    blocks: x.next().unwrap().to_digit(10).unwrap() as usize,
                };
                id += 1;

                if let Some(free) = x.next() {
                    let free = Blop {
                        kind: BlopKind::Free,
                        blocks: free.to_digit(10).unwrap() as usize,
                    };
                    Vec::from([file, free])
                } else {
                    Vec::from([file])
                }
            })
            .filter(|x| x.blocks > 0)
            .collect_vec();
        Self { disk }
    }

    fn checksum(&self) -> usize {
        let mut uncompressed = Vec::<BlopKind>::new();
        for x in self.disk.iter() {
            for _ in 0..x.blocks {
                uncompressed.push(x.kind);
            }
        }

        let mut left = 0;
        let mut right = uncompressed.len() - 1;
        loop {
            while !matches!(uncompressed[left], BlopKind::Free) && left < right {
                left += 1;
            }
            while !matches!(uncompressed[right], BlopKind::File(_)) && right > left {
                right -= 1;
            }
            if left == right {
                break;
            }
            uncompressed.swap(left, right);
        }

        uncompressed
            .iter()
            .enumerate()
            .take_while(|(_, b)| !matches!(b, BlopKind::Free))
            .map(|(i, b)| {
                let id = match b {
                    BlopKind::File(v) => v,
                    _ => unreachable!(),
                };
                i * id
            })
            .sum()
    }
}

struct Blop {
    kind: BlopKind,
    blocks: usize,
}

#[derive(Clone, Copy)]
enum BlopKind {
    Free,
    File(usize),
}

pub fn run() {
    let input = read_to_string("inputs/day09.txt").unwrap();
    let c = Computer::new(&input);
    println!("Checksum: {}", c.checksum());
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::Computer;

    #[test]
    fn checksum() {
        let c = Computer::new(&read_to_string("inputs/day09_small.txt").unwrap());
        assert_eq!(1928, c.checksum())
    }
}
