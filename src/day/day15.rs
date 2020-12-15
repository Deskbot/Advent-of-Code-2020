use std::collections::HashMap;


pub fn day15() {
    println!("Part 1: {}", part1(&[15,5,1,4,7,0]));
    // println!("Part 2: {}", part2(&file));
}

fn part1(starting_numbers: &[i64]) -> i64 {
    let mut last_position = HashMap::<usize,i64>::new();

    for (index, &num) in starting_numbers.iter().enumerate() {
        last_position.insert(index, num);
    }

    0
}
