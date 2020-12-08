
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

pub struct Console {
    pub address: i32,
    pub accumulator: i32,
    memory: Vec<Instruction>,
}

impl Console {
    pub fn new(instructions: Vec<Instruction>) -> Console {
        Console {
            address: 0,
            accumulator: 0,
            memory: instructions
        }
    }

    pub fn step(&mut self) {
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
        }
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
