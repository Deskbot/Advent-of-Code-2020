use std::fs;

pub fn day18() {
    let file = fs::read_to_string("input/day18.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // split into problems
    input.lines()
        .map(solve)
        .sum()
}

fn solve(problem: &str) -> i64 {
     // calculate answer str -> i64
        // if encounter a bracket
            // find the paired bracket
            // calculate the answer to that substring
            // continue

    let mut accumulator = 0;
    let mut op: Option<char> = None;

    let mut itr = problem.chars();

    loop {
        // parse
        let c = match itr.next() {
            Some(c) => c,
            None => break,
        };

        let operand: i64;

        if c == ' ' {
            continue;
        }

        if c == '+' || c == '*' {
            op = Some(c);
        }

        else if c == '(' {
            // find closing bracket
            // by incrementing the iterator
            // operand = call solve on that expression

            let mut depth = 0;
            let substr = "";

            loop {
                let next_ch = itr.next();
                if next_ch ==
            }
        }

        else {
            // assume it's a 1 digit number
            operand = c.to_digit(10).unwrap() as i64;

            // do a calculation

            accumulator = match op {
                Some('*') => accumulator * operand,
                Some('+') => accumulator + operand,
                None =>      operand,
                _ => panic!("invalid operation"),
            }
        }

    }

    0
}
