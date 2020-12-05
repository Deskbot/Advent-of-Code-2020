use std::fs;
use substring::Substring;

const THE_NUMBER_2_AS_AN_I32: i32 = 2;

struct Seat {
    col: i32,
    row: i32,
}

impl Seat {
    pub fn from_str(s: &str) -> Seat {
        let row_seq = s.substring(0, 7);
        let col_seq = s.substring(7, 10);

        let row = Seat::seq_to_num(row_seq);
        let col = Seat::seq_to_num(col_seq);

        return Seat {
            col,
            row
        };
    }

    pub fn id(&self) -> i32 {
        return self.col + self.row * 8;
    }

    fn seq_to_num(s: &str) -> i32 {
        let splits = s.len() as u32;
        let mut min = 0;
        let mut max = THE_NUMBER_2_AS_AN_I32.pow(splits) - 1;

        for ch in s.chars() {
            let new_bound = (min as f32 + max as f32) / 2 as f32;

            match ch {
                'B' => min = new_bound.ceil() as i32,
                'R' => min = new_bound.ceil() as i32,

                'F' => max = new_bound.floor() as i32,
                'L' => max = new_bound.floor() as i32,
                _ => {
                    panic!("poo")
                },
            }
        }

        assert_eq!(min, max);

        return min;
    }
}

pub fn day05() {
    let file = fs::read_to_string("input/day05.txt")
        .expect("input not found");

    println!("Part 1: {}", part1(&file));
    println!("Part 2: {}", part2(&file));
}

fn part1(file: &str) -> i32 {
    file.lines()
        .map(Seat::from_str)
        .map(|seat| seat.id())
        .max()
        .unwrap()
}

fn part2(file: &str) -> i32 {
    let mut seat_ids = file.lines()
        .map(Seat::from_str)
        .map(|seat| seat.id())
        .collect::<Vec<i32>>();

    seat_ids.sort();

    for i in 1..(seat_ids.len() - 2) {
        let &prev_id = &seat_ids[i-1];
        let &id = &seat_ids[i];
        let &next_id = &seat_ids[i+1];

        if prev_id != id - 1 {
            return id - 1;
        }

        if next_id != id + 1 {
            return id + 1;
        }
    }

    panic!("poop");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {

        assert_eq!(part1("FBFBBFFRLR"), 357);
        assert_eq!(part1("BFFFBBFRRR"), 567);
        assert_eq!(part1("FFFBBBFRRR"), 119);
        assert_eq!(part1("BBFFBBFRLL"), 820);
    }
}