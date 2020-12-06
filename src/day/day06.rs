use std::fs;
use std::collections::{
    HashMap,
    HashSet,
};

pub fn day06() {
    let file = fs::read_to_string("input/day06.txt")
        .expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i32 {
    let groups = input.split("\n\n");

    return groups.map(any_answered_yes)
        .fold(0, |a,b| a+b);
}

fn part2(input: &str) -> i32 {
    let groups = input.split("\n\n");

    return groups.map(all_answered_yes)
        .fold(0, |a,b| a+b);
}

fn any_answered_yes(group_input: &str) -> i32 {
    let mut unique = HashSet::new();

    for yes_answer in group_input.chars() {
        if yes_answer == '\n' {
            continue;
        }

        unique.insert(yes_answer);
    }

    return unique.len() as i32;
}

fn all_answered_yes(group_input: &str) -> i32 {
    let mut yes_count: HashMap<char,i32> = HashMap::new();

    // build map of questions to number of people who answered it yes

    for yes_answer in group_input.chars() {
        if yes_answer == '\n' {
            continue;
        }

        yes_count.entry(yes_answer)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    // find entries in the map that have the correct size
    let people_count = group_input.lines().count() as i32;
    let mut all_yes_count = 0;

    for &count in yes_count.values() {
        if count == people_count {
            all_yes_count += 1;
        }
    }

    return all_yes_count;

}
