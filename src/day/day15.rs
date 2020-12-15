use std::collections::HashMap;


pub fn day15() {
    println!("Part 1: {}", part1(&[15,5,1,4,7,0]));
    println!("Part 2: {}", part2(&[15,5,1,4,7,0]));
}

fn part1(starting_numbers: &[i64]) -> i64 {
    get_nth_number(starting_numbers, 2020)
}

fn part2(starting_numbers: &[i64]) -> i64 {
    get_nth_number(starting_numbers, 30000000)
}

fn get_nth_number(starting_numbers: &[i64], n: i64) -> i64 {
    let mut penultimate_turns = HashMap::<i64,i64>::new();

    for (index, &num) in starting_numbers.iter().enumerate() {
        penultimate_turns.insert(num, 1 + index as i64); // 1 indexed
    }

    let mut last_number = *starting_numbers.last().unwrap();
    let mut last_turn = starting_numbers.len() as i64; // 1 indexed

    while last_turn < n {
        let this_number;

        if let Some(&penultimate_turn) = penultimate_turns.get(&last_number) {
            this_number = last_turn - penultimate_turn;
        } else {
            this_number = 0;
        }

        penultimate_turns.insert(last_number, last_turn);

        last_turn += 1;
        last_number = this_number;
    }

    assert_eq!(last_turn, n);

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
