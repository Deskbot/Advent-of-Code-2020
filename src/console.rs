
use std::collections::HashSet;

#[derive(Clone)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Instruction {
    fn new(opcode: &str, operand: i32) -> Instruction {
        match opcode {
            "acc" => Instruction::Acc(operand),
            "jmp" => Instruction::Jmp(operand),
            "nop" => Instruction::Nop(operand),
            _ => panic!(),
        }
    }
}

pub enum Exit {
    InfiniteLoop,
    Terminated(i32),
}

#[derive(Clone)]
pub struct Console {
    pub accumulator: i32,
    address: i32,
    pub memory: Vec<Instruction>,
}

impl Console {
    pub fn new(instructions: Vec<Instruction>) -> Console {
        Console {
            accumulator: 0,
            address: 0,
            memory: instructions
        }
    }

    pub fn hack(&mut self, address: usize, value: Instruction) {
        self.memory[address] = value;
    }

    pub fn run(&mut self) -> Exit {
        let mut prev_addresses = HashSet::new();

        loop {
            if prev_addresses.contains(&self.address) {
                return Exit::InfiniteLoop;
            }

            prev_addresses.insert(self.address);

            let terminated = self.step();

            if terminated {
                return Exit::Terminated(self.accumulator);
            }
        }
    }

    pub fn step(&mut self) -> bool {
        use Instruction::*;

        let instruction = &self.memory[self.address as usize];

        match *instruction {
            Acc(operand) => {
                self.accumulator += operand;
                self.address += 1;
            },

            Jmp(operand) => {
                self.address += operand;
            },

            Nop(_) => {
                self.address += 1;
            }
        };

        return self.address >= self.memory.len() as i32;
    }
}

pub fn parse_code(code: &str) -> Vec<Instruction> {
    code.lines()
        .map(parse_instruction)
        .collect()
}

fn parse_instruction(code: &str) -> Instruction {
    let mut itr = code.split(' ');
    let opcode = itr.next().unwrap();
    let operand = itr.next().unwrap().parse::<i32>().unwrap();

    return Instruction::new(opcode, operand);
}
