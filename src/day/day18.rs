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
        .map(|mut c| solve_advanced(&mut c, 0))
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

        else if c == '+' || c == '*' {
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


fn solve_advanced(problem: &mut Chars, depth: usize) -> i64 {
    let mut accumulator = 0;
    let chars = problem;

    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => break,
        };

        println!("{} acc b4 {}", "  ".repeat(depth), accumulator);
        println!("{} char {}",   "  ".repeat(depth), c);

        if c == ' ' {
            continue;
        }

        else if c == '+' {
            accumulator += evaluate_eager(chars, 0);
        }

        else if c == '*' {
            accumulator *= solve_advanced(chars, 0);
        }

        else if c == ')' {
            break;
        }

        else if c == '(' {
            return solve_advanced(chars, 0);
        }

        else {
            // assume it's a 1 digit number
            accumulator = c.to_digit(10).unwrap() as i64;
        }

        println!("acc af {}", accumulator);
    }


    return accumulator;
}

fn evaluate_eager(chars: &mut Chars, depth: usize) -> i64 {
    loop {
        let c = match chars.next() {
            Some(c) => c,
            None => panic!("expected an expression to evaluate"),
        };

        if c == ' ' {
            continue;
        }

        if c == '(' {
            return solve_advanced(chars, depth + 1);
        }

        // assume it's a number

        return c.to_digit(10).unwrap() as i64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_example_1() {
        assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
    }

    #[test]
    fn part2_example_3() {
        assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
    }

    #[test]
    fn part2_example_4() {
        assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669060);
    }

    #[test]
    fn part2_example_5() {
        assert_eq!(part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"), 23340);
    }

    #[test]
    fn bug() {
        assert_eq!(part2("(2 * (5 * 5) + 6) + 9"), 71);
    }

    #[test]
    fn bug_2() {
        assert_eq!(part2("(2 + 2) * 5 + 6"), 44);
    }
}
