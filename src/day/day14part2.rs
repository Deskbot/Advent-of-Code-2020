use crate::day::day14::Instruction;
use crate::util::sublists;

pub struct DockerProgram {
    instructions: Vec<Instruction>,
    pub memory: Vec<i64>,
    one_mask: i64, // 1 = 1
    x_mask: i64, // 1 = X
}

impl DockerProgram {
    pub fn parse(input: &str) -> DockerProgram {
        DockerProgram {
            instructions: input.lines()
                .map(Instruction::parse_instruction)
                .collect(),
            memory: Vec::new(),
            one_mask: 0,
            x_mask: 0,
        }
    }

    fn split(mask: &str) -> (i64, i64) {
        let mut col_value = 1;
        let mut ones = 0;
        let mut zeros = 0;

        for digit in mask.chars().rev() {
            if digit == 'X' {
                zeros += col_value;
            } else if digit == '1' {
                ones += col_value;
            }

            col_value *= 2;
        }

        return (ones, zeros);
    }

    fn all_x_mask_digits(&self) -> Vec::<i64> {
        let mut x_digits = Vec::<i64>::with_capacity(36);

        let mut digit = 1;
        for _ in 0..36 {
            digit *= 2;
            let x_at_digit = digit & self.x_mask != 0;

            if x_at_digit {
                x_digits.push(digit);
            }
        }

        return x_digits;
    }

    /*
       num|ones
        --|---|--
        0 | 0 | 0
        0 | 1 | 1
        1 | 0 | 1
        1 | 1 | 1
    */
    fn apply_ones(num: i64, one_bit_field: i64) -> i64 {
        num | one_bit_field
    }

    /*
       num|zeros
        --|---|--
        0 | 0 | 0
        0 | 1 | 0
        1 | 0 | 1
        1 | 1 | 0
    */
    fn apply_zeros(num: i64, zero_bit_field: i64) -> i64 {
        num & !zero_bit_field
    }

    fn addresses(&self, address: i64, results: &mut Vec<usize>) {
        let mut result = Self::apply_ones(address, self.one_mask);

        // get a list of digits that are xs
        let x_digits = self.all_x_mask_digits();

        // split the x digits into every combination of which digits are one and zero
        for (ones, zeros) in sublists(&mut x_digits.iter()) {
            // turns that combination of digits back into masks
            let ones_mask = ones.iter()
                .fold(0, |acc, &&sum| acc + sum);
            let zeros_mask = zeros.iter()
                .fold(0, |acc, &&sum| acc + sum);

            // only apply digits in the x digit columns
            result = Self::apply_ones(result, self.x_mask & ones_mask);
            result = Self::apply_zeros(result, self.x_mask & zeros_mask);
            results.push(result as usize);
        }
    }

    pub fn run(&mut self) {
        use Instruction::*;

        for instruction in &self.instructions {
            match instruction {
                Mask(mask) => {
                    let (ones, xs) = Self::split(mask);
                    self.one_mask = ones;
                    self.x_mask = xs;
                },
                &Mem(base_addr, value) => {
                    let mut addresses = Vec::new();
                    self.addresses(base_addr as i64, &mut addresses);

                    for address in addresses {
                        // ensure memory is big enough
                        if self.memory.len() <= address {
                            self.memory.resize(address + 1 , 0);
                        }

                        self.memory[address as usize] = value;
                    }
                },
            };
        }
    }
}
