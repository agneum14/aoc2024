#![allow(dead_code)]
use core::panic;
use std::fs::read_to_string;

use itertools::Itertools;

struct Cpu {
    ra: u64,
    rb: u64,
    rc: u64,
    ip: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Cpu {
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

    fn dump_instructions(&mut self) {
        while let Some(combo) = self.program.get(self.ip + 1) {
            let opcode = self.program.get(self.ip).unwrap();
            let inst = Instruction::new(*opcode, *combo);
            println!("{}", inst.decompile());
            inst.execute(self);
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

    fn execute(&self, cpu: &mut Cpu) {
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
            InstructionKind::Out => cpu.output.push((combo % 8) as u8),
            InstructionKind::Bdv => cpu.rb = cpu.ra / 2_u64.pow(combo as u32),
            InstructionKind::Cdv => cpu.rc = cpu.ra / 2_u64.pow(combo as u32),
        }
        if !jumped {
            cpu.ip += 2;
        }
    }

    fn decompile(&self) -> String {
        let combo = match self.operand {
            0..=3 => format!("{}", self.operand),
            4 => "rA".to_string(),
            5 => "rB".to_string(),
            6 => "rC".to_string(),
            _ => panic!("Invalid combo"),
        };
        match self.kind {
            InstructionKind::Adv => format!("rA /= 2**{}", combo),
            InstructionKind::Bxl => format!("rB ^= {}", self.operand),
            InstructionKind::Bst => format!("rB = {} % 8", combo),
            InstructionKind::Jnz => format!("ip = {} if rA != 0", self.operand),
            InstructionKind::Bxc => format!("rB ^= rC"),
            InstructionKind::Out => format!("push {} % 8", combo),
            InstructionKind::Bdv => format!("rB = rA / 2**{}", combo),
            InstructionKind::Cdv => format!("rC = rA / 2**{}", combo),
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

fn run_with_ra(ra: u64, input: &str) -> Vec<u8> {
    let mut cpu = Cpu::new(input);
    cpu.ra = ra;
    cpu.run();
    cpu.output
}

fn find_recursive_ra(input: &str) -> u64 {
    let mut ras = Vec::from([0_u64]);
    let mut cpu = Cpu::new(input);
    cpu.run();
    let program = cpu.program;

    for i in 0..program.len() {
        ras = ras
            .into_iter()
            .flat_map(|ra| (0..8).map(move |end| (ra << 3) + end))
            .filter(|ra| run_with_ra(*ra, input) == program[program.len() - 1 - i..])
            .collect_vec();
    }

    *ras.iter().min().unwrap()
}

pub fn run() {
    let input = read_to_string("inputs/day17.txt").unwrap();
    let mut cpu = Cpu::new(&input);
    cpu.run();
    println!("Output: {}", cpu.output());
    // let mut cpu = Cpu::new(&input);
    // cpu.dump_instructions();
    println!("Register A: {}", find_recursive_ra(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output() {
        let mut cpu = Cpu::new(&read_to_string("inputs/day17_small.txt").unwrap());
        cpu.run();
        assert_eq!("4,6,3,5,6,3,5,2,1,0", cpu.output())
    }
}
