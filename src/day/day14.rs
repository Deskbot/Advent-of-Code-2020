use crate::day::day14part1;
use crate::day::day14part2;
use regex::Regex;
use std::fs;

pub enum Instruction {
    Mask(String),
    Mem(usize, i64),
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
        let mem_addr = mem_addr.parse::<usize>().unwrap();
        let assign = assign.parse::<i64>().unwrap();

        return Instruction::Mem(mem_addr, assign);
    }
}

pub fn day14() {
    let file = fs::read_to_string("input/day14.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    let mut program = day14part1::DockerProgram::parse(input);
    program.run();

    return program.memory.iter()
        .fold(0, |acc,next| acc + next);
}

#[cfg(test)]
mod tests {
    use super::*;

}
