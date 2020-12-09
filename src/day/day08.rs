use crate::console::{
    Console,
    Exit,
    Instruction,
    parse_code,
};
use std::fs;

pub fn day08() {
    let file = fs::read_to_string("input/day08.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i32 {
    let instructions = parse_code(input);
    let mut machine = Console::new(instructions);

    machine.run();

    return machine.accumulator;
}

fn part2(input: &str) -> i32 {
    let instructions = parse_code(input);
    let machine = Console::new(instructions);

    // make an iterator of every possible machine
    // with the potentially problematic instruction changed
    let machines_to_test =
        (0..machine.memory.len())
            .map(|address| (address, &machine.memory[address]))
            .filter(|(_, instruction)| match instruction {
                Instruction::Nop(_) => true,
                Instruction::Jmp(_) => true,
                Instruction::Acc(_) => false,
            })
            .map(|(address, instruction_to_hack)| {
                let mut new_machine = machine.clone();
                new_machine.hack(address, fix_instruction(&instruction_to_hack));
                return new_machine;
            });

    // let passing_machine =
    //     machines_to_test
    //         .filter(|machine| {
    //             let result = machine.run();

    //             match result {
    //                 Exit::InfiniteLoop => false,
    //                 Exit::Terminated(_) => true,
    //             }
    //         })
    //         .next()
    //         .unwrap();

    // get accumulator of the first hacked machine that terminates
    for mut machine in machines_to_test {
        let exit_code = machine.run();

        if let Exit::Terminated(result) = exit_code {
            return result;
        }
    }

    panic!("no machine wasn't an infinite loop");
}

fn fix_instruction(instruction: &Instruction) -> Instruction {
    match *instruction {
        Instruction::Nop(opcode) => Instruction::Jmp(opcode),
        Instruction::Jmp(opcode) => Instruction::Nop(opcode),
        Instruction::Acc(opcode) => Instruction::Acc(opcode),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = fs::read_to_string("input/day08.txt").expect("input not found");
        assert_eq!(part1(&input), 1766);
    }

    #[test]
    fn part2_example() {
        let input = fs::read_to_string("input/day08.txt").expect("input not found");
        assert_eq!(part2(&input), 1639);
    }
}
