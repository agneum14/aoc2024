#![allow(dead_code)]
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

    fn defragmented_checksum(&self) -> usize {
        let max_id = match self
            .disk
            .iter()
            .filter(|x| matches!(x.kind, BlopKind::File(_)))
            .last()
            .unwrap()
            .kind
        {
            BlopKind::File(v) => v,
            _ => unreachable!(),
        };

        let mut disk = self.disk.clone();
        for id in (0..=max_id).rev() {
            let (right, file) = disk
                .iter()
                .cloned()
                .enumerate()
                .rfind(|(_, b)| b.kind == BlopKind::File(id))
                .unwrap();
            if let Some((left, free)) = disk
                .iter()
                .cloned()
                .enumerate()
                .take(right)
                .find(|(_, b)| matches!(b.kind, BlopKind::Free) && b.blocks >= file.blocks)
            {
                disk[right] = Blop {
                    kind: BlopKind::Free,
                    blocks: file.blocks,
                };
                disk[left] = file;
                let diff = free.blocks - file.blocks;
                if diff > 0 {
                    disk.insert(
                        left + 1,
                        Blop {
                            kind: BlopKind::Free,
                            blocks: diff,
                        },
                    );
                }

                // combine adjacent free blocks
                for right in 1..disk.len() {
                    let left = right - 1;
                    if disk[left].kind == BlopKind::Free && disk[right].kind == BlopKind::Free {
                        disk[right].blocks += disk[left].blocks;
                        disk[left].blocks = 0;
                    }
                }
                disk.retain(|x| x.blocks > 0);
            }
        }

        let mut i = 0;
        let mut checksum = 0;
        for b in disk.iter() {
            if let BlopKind::File(id) = b.kind {
                for _ in 0..b.blocks {
                    checksum += id * i;
                    i += 1;
                }
            } else {
                i += b.blocks;
            }
        }
        checksum
    }
}

#[derive(Debug, Clone, Copy)]
struct Blop {
    kind: BlopKind,
    blocks: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BlopKind {
    Free,
    File(usize),
}

pub fn run() {
    let input = read_to_string("inputs/day09.txt").unwrap();
    let c = Computer::new(&input);
    println!("Checksum: {}", c.checksum());
    println!("Defragmented checksum: {}", c.defragmented_checksum());
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

    #[test]
    fn defragmented_checksum() {
        let c = Computer::new(&read_to_string("inputs/day09_small.txt").unwrap());
        assert_eq!(2858, c.defragmented_checksum())
    }
}
