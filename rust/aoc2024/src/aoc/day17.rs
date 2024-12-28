use std::collections::HashMap;

use common::{
    constants::TWO_NEWLINE,
    problem::day::{AoCProblem, Solution},
};

pub struct Day17 {
    program: Vec<u8>,
    registers: HashMap<char, usize>,
}

impl AoCProblem for Day17 {
    fn prepare(input: String) -> Self {
        let (raw_registers, raw_program) = input.split_once(TWO_NEWLINE).unwrap();
        let mut registers = HashMap::new();

        for reg in raw_registers.lines() {
            registers.insert(
                reg[9..10].chars().nth(0).unwrap(),
                reg[12..].parse().unwrap(),
            );
        }

        Self {
            registers,
            program: raw_program[9..]
                .split(',')
                .map(|n| n.parse().unwrap())
                .collect(),
        }
    }

    fn part1(&mut self) -> Solution {
        let mut computer = Computer::new(
            &self.program,
            *self.registers.get(&'A').unwrap(),
            *self.registers.get(&'B').unwrap(),
            *self.registers.get(&'C').unwrap(),
        );
        computer.run_until_completion();
        computer
            .output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
            .into()
    }

    fn part2(&mut self) -> Solution {
        0.into()
    }

    fn day() -> u32 {
        17
    }

    fn year() -> u32 {
        2024
    }
}

pub struct Computer<'a> {
    program: &'a [u8],
    reg_a: usize,
    reg_b: usize,
    reg_c: usize,
    instr_ptr: usize,
    output: Vec<usize>,
}

impl<'a> Computer<'a> {
    pub fn new(program: &'a [u8], reg_a: usize, reg_b: usize, reg_c: usize) -> Self {
        Self {
            program,
            reg_a,
            reg_c,
            reg_b,
            instr_ptr: 0,
            output: vec![],
        }
    }

    pub fn run_single_step(&mut self) -> bool {
        if self.instr_ptr >= self.program.len() {
            return false;
        }

        match self.program[self.instr_ptr] {
            0 => self.adv(),
            1 => self.bxl(),
            2 => self.bst(),
            3 => self.jnz(),
            4 => self.bxc(),
            5 => self.out(),
            6 => self.bdv(),
            7 => self.cdv(),
            p => panic!("Unsupported instruction {p}"),
        }

        true
    }

    pub fn run_until_completion(&mut self) {
        while self.run_single_step() {}
    }

    pub fn next_instruction(&mut self) {
        self.instr_ptr += 2;
    }

    pub fn adv(&mut self) {
        let numerator = self.reg_a;
        let denominator = 2_usize.pow(self.combo_operand() as u32);
        self.reg_a = numerator / denominator;
        self.next_instruction();
    }

    pub fn bxl(&mut self) {
        self.reg_b ^= self.literal_operand();
        self.next_instruction();
    }

    pub fn bst(&mut self) {
        self.reg_b = self.combo_operand() % 8;
        self.next_instruction();
    }

    pub fn jnz(&mut self) {
        if self.reg_a == 0 {
            self.next_instruction();
        } else {
            self.instr_ptr = self.literal_operand();
        }
    }

    pub fn bxc(&mut self) {
        self.reg_b ^= self.reg_c;
        self.next_instruction();
    }

    pub fn out(&mut self) {
        self.output.push(self.combo_operand() % 8);
        self.next_instruction();
    }

    pub fn bdv(&mut self) {
        let numerator = self.reg_a;
        let denominator = 2_usize.pow(self.combo_operand() as u32);
        self.reg_b = numerator / denominator;
        self.next_instruction();
    }

    pub fn cdv(&mut self) {
        let numerator = self.reg_a;
        let denominator = 2_usize.pow(self.combo_operand() as u32);
        self.reg_c = numerator / denominator;
        self.next_instruction();
    }

    pub fn literal_operand(&self) -> usize {
        self.program[self.instr_ptr + 1] as usize
    }

    pub fn combo_operand(&self) -> usize {
        match self.program[self.instr_ptr + 1] {
            v @ 0..=3 => v as usize,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            7 => panic!("Reserved!"),
            _ => unreachable!(),
        }
    }
}
