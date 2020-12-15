use std::collections::HashMap;


pub fn day15() {
    println!("Part 1: {}", part1(&[15,5,1,4,7,0]));
    // println!("Part 2: {}", part2(&file));
}

fn part1(starting_numbers: &[i64]) -> i64 {
    let mut last_position = HashMap::<i64,i64>::new();

    for (index, &num) in starting_numbers.iter().enumerate() {
        last_position.insert(num, 1 + index as i64); // 1 indexed
    }

    let mut last_number = *starting_numbers.last().unwrap();
    let mut last_index = starting_numbers.len() as i64; // 1 indexed

    while last_index < 2020 {
        let next_number;

        if let Some(&index_last_time) = last_position.get(&last_number) {
            next_number = last_index - index_last_time;
        } else {
            next_number = 0;
        }

        let next_index = last_index + 1;
        last_position.insert(last_number, next_index);

        last_index += 1;
        last_number = next_number;

        println!("{:?}", last_position);
    }

    assert_eq!(last_index, 2020);

    return last_number;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&[0,3,6]), 436);
    }
}
