use crate::day::day14part1;
use crate::day::day14part2;
use regex::Regex;
use std::fs;

pub enum Instruction {
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

pub fn day14() {
    let file = fs::read_to_string("input/day14.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    let mut program = day14part1::DockerProgram::parse(input);
    program.run();

    return program.memory.iter()
        .fold(0, |acc,next| acc + next);
}

fn part2(input: &str) -> i64 {
    let mut program = day14part2::DockerProgram::parse(input);
    program.run();

    return program.memory.values()
        .fold(0, |acc,next| acc + next);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(part2(input), 208);
    }
}
