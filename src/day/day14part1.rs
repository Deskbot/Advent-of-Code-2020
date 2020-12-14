use crate::day::day14::Instruction;

pub struct DockerProgram {
    instructions: Vec<Instruction>,
    pub memory: Vec<i64>,
    one_mask: i64, // 1 = 1, 0 = X
    zero_mask: i64, // 1 = 0, 0 = X
}

impl DockerProgram {
    pub fn parse(input: &str) -> DockerProgram {
        DockerProgram {
            instructions: input.lines()
                .map(Instruction::parse_instruction)
                .collect(),
            memory: Vec::new(),
            one_mask: 0,
            zero_mask: 0,
        }
    }

    fn split(mask: &str) -> (i64, i64) {
        let mut col_value = 1;
        let mut ones = 0;
        let mut zeros = 0;

        for digit in mask.chars().rev() {
            if digit == '0' {
                zeros += col_value;
            } else if digit == '1' {
                ones += col_value;
            }

            col_value *= 2;
        }

        return (ones, zeros);
    }

    /*
       num|ones
        --|---|--
        0 | 0 | 0
        0 | 1 | 1
        1 | 0 | 1
        1 | 1 | 1
    */
    fn apply_ones(num: i64, ones: i64) -> i64 {
        num | ones
    }

    /*
       num|zeros
        --|---|--
        0 | 0 | 0
        0 | 1 | 0
        1 | 0 | 1
        1 | 1 | 0
    */
    fn apply_zeros(num: i64, zeros: i64) -> i64 {
        num & !zeros
    }

    pub fn run(&mut self) {
        use Instruction::*;

        for instruction in &self.instructions {
            match instruction {
                Mask(mask) => {
                    let (ones, zeros) = Self::split(mask);
                    self.one_mask = ones;
                    self.zero_mask = zeros;
                },
                &Mem(address, value) => {
                    let result = Self::apply_ones(value, self.one_mask);
                    let result = Self::apply_zeros(result, self.zero_mask);

                    // ensure memory is big enough
                    if self.memory.len() <= address {
                        self.memory.resize(address + 1 , 0);
                    }

                    self.memory[address] = result;
                },
            };
        }
    }
}
