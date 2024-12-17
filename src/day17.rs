use core::panic;
use std::fs::read_to_string;

use itertools::Itertools;

struct CPU {
    ra: u64,
    rb: u64,
    rc: u64,
    ip: usize,
    program: Vec<u8>,
    output: Vec<u64>,
}

impl CPU {
    fn new(input: &str) -> Self {
        let mut data = input.split("\n\n");
        let (ra, rb, rc) = data
            .next()
            .unwrap()
            .lines()
            .map(|line| line.split(": ").skip(1).next().unwrap().parse().unwrap())
            .collect_tuple()
            .unwrap();
        let program = data
            .next()
            .unwrap()
            .trim_end()
            .split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|n| n.parse().unwrap())
            .collect_vec();
        Self {
            ra,
            rb,
            rc,
            ip: 0,
            program,
            output: Vec::new(),
        }
    }

    fn run(&mut self) {
        while let Some(combo) = self.program.get(self.ip + 1) {
            let opcode = self.program.get(self.ip).unwrap();
            Instruction::new(*opcode, *combo).execute(self);
        }
    }

    fn output(&self) -> String {
        self.output.iter().join(",")
    }
}

struct Instruction {
    kind: InstructionKind,
    operand: u8,
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Self {
        let kind = match opcode {
            0 => InstructionKind::Adv,
            1 => InstructionKind::Bxl,
            2 => InstructionKind::Bst,
            3 => InstructionKind::Jnz,
            4 => InstructionKind::Bxc,
            5 => InstructionKind::Out,
            6 => InstructionKind::Bdv,
            7 => InstructionKind::Cdv,
            _ => panic!("Invalid opcode"),
        };
        Self { kind, operand }
    }

    fn execute(&self, cpu: &mut CPU) {
        let combo = match self.operand {
            0..=3 => self.operand as u64,
            4 => cpu.ra,
            5 => cpu.rb,
            6 => cpu.rc,
            _ => panic!("Invalid combo"),
        };
        let mut jumped = false;
        match self.kind {
            InstructionKind::Adv => cpu.ra /= 2_u64.pow(combo as u32),
            InstructionKind::Bxl => cpu.rb ^= self.operand as u64,
            InstructionKind::Bst => cpu.rb = combo % 8,
            InstructionKind::Jnz => {
                if cpu.ra != 0 {
                    jumped = true;
                    cpu.ip = self.operand as usize;
                }
            }
            InstructionKind::Bxc => cpu.rb ^= cpu.rc,
            InstructionKind::Out => cpu.output.push(combo % 8),
            InstructionKind::Bdv => cpu.rb = cpu.ra / 2_u64.pow(combo as u32),
            InstructionKind::Cdv => cpu.rc = cpu.ra / 2_u64.pow(combo as u32),
        }
        if !jumped {
            cpu.ip += 2;
        }
    }
}

enum InstructionKind {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

pub fn run() {
    let input = read_to_string("inputs/day17.txt").unwrap();
    let mut cpu = CPU::new(&input);
    cpu.run();
    println!("Output: {}", cpu.output())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        let mut cpu = CPU::new(&read_to_string("inputs/day17_small.txt").unwrap());
        cpu.run();
        assert_eq!("4,6,3,5,6,3,5,2,1,0", cpu.output())
    }
}
