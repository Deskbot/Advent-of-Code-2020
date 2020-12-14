use regex::Regex;
use std::fs;

enum Instruction {
    Mask(String),
    Mem(i64, i64),
}

impl Instruction {
    pub fn parse_instruction(line: &str) -> Instruction {
        let mut itr = line.split(" = ");
        let lhs = itr.next().unwrap();
        let assign = itr.next().unwrap();

        if lhs == "mask" {
            return Instruction::Mask(assign.to_string());
        }

        let regex = Regex::new(r"^mem\[([0-9]+)\]$").unwrap();

        // get(1) gets the first bracketed section
        let mem_addr = regex.captures(lhs).unwrap().get(1).unwrap().as_str(); // WTF RUST ???????
        let mem_addr = mem_addr.parse::<i64>().unwrap();
        let assign = assign.parse::<i64>().unwrap();

        return Instruction::Mem(mem_addr, assign);
    }
}

struct DockerProgram {
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
                    if self.memory.len() <= address as usize {
                        self.memory.resize(address as usize + 1 , 0);
                    }

                    self.memory[address as usize] = result;
                },
            };
        }
    }
}

pub fn day14() {
    let file = fs::read_to_string("input/day14.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    let mut program = DockerProgram::parse(input);
    program.run();

    return program.memory.iter()
        .fold(0, |acc,next| acc + next);
}

#[cfg(test)]
mod tests {
    use super::*;

}
