use std::str::Chars;
use std::fs;

pub fn day18() {
    let file = fs::read_to_string("input/day18.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(input: &str) -> i64 {
    // split into problems
    input.lines()
        .map(str::chars)
        .map(|mut c| solve(&mut c))
        .sum()
}

fn part2(input: &str) -> i64 {
    // split into problems
    input.lines()
        .map(str::chars)
        .map(|mut c| solve_advanced(&mut c))
        .sum()
}

fn solve(problem: &mut Chars) -> i64 {
     // calculate answer str -> i64
        // if encounter a bracket
            // find the paired bracket
            // calculate the answer to that substring
            // continue

    let mut accumulator = 0;
    let mut op: Option<char> = None;
    let chars = problem;

    loop {
        // parse
        let c = match chars.next() {
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

        else if c == ')' {
            break;
        }

        else {
            if c == '(' {
                operand = solve(chars);
            } else {
                // assume it's a 1 digit number
                operand = c.to_digit(10).unwrap() as i64;
            }

            // do a calculation

            accumulator = match op {
                Some('*') => accumulator * operand,
                Some('+') => accumulator + operand,
                None =>      operand,
                _ => panic!("invalid operation"),
            }
        }

    }

    return accumulator;
}


fn solve_advanced(problem: &mut Chars) -> i64 {
     // calculate answer str -> i64
        // if encounter a bracket
            // find the paired bracket
            // calculate the answer to that substring
            // continue

    let mut accumulator = 0;
    let mut op: Option<char> = None;
    let chars = problem;

    loop {
        // parse
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };

        let operand: i64;

        if c == ' ' {
            continue;
        }

        if c == '+' {
            accumulator += solve_advanced(chars);
            continue;
        }

        if c == '*' {
            op = Some(c);
        }

        else if c == ')' {
            break;
        }

        else {
            if c == '(' {
                operand = solve_advanced(chars);
            } else {
                // assume it's a 1 digit number
                operand = c.to_digit(10).unwrap() as i64;
            }

            // do a calculation

            accumulator = match op {
                Some('*') => accumulator * operand,
                None =>      operand,
                _ => panic!("invalid operation"),
            }
        }

    }

    return accumulator;
}
