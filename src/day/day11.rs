use crate::grid::Grid;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Seat {
    Empty,
    Floor,
    Occupied,
}

pub fn day11() {
    let file = fs::read_to_string("input/day11.txt").expect("input not found");

    println!("Part 1: {}", part1(&file));
    // println!("Part 2: {}", part2(&joltages));
}

fn part1(input: &str) -> i64 {
    let char_2d_vec = input.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let grid = Grid::new(char_2d_vec)
        .fmap(|ch| match ch {
            'L' => Seat::Empty,
            '#' => Seat::Occupied,
            '.' => Seat::Floor,
            _ => panic!("invalid input: char doesn't represent a seat"),
        });

    let mut grid = grid;

    loop {
        let new_grid = grid.game_of_life(|was, neighbours| {

            if was == &Seat::Empty {
                if has_qty(neighbours, &&Seat::Occupied, 0) {
                    return Seat:: Occupied
                }
            }

            else if was == &Seat::Occupied {
                if has_at_least_qty(neighbours, &&Seat::Occupied, 4) {
                    return Seat::Empty;
                }
            }

            return was.clone();
        });


        if new_grid.eq(&grid) {
            break;
        }

        grid = new_grid;
    }

    return grid.cell_iter().iter()
        .filter(|cell| cell == &&Seat::Occupied)
        .count() as i64;
}

fn has_qty<T: Eq>(vec: &Vec<T>, val_to_restrict: &T, qty: i64) -> bool {
    let instances_found =
        vec.iter()
            .filter(|&val| val == val_to_restrict)
            .count() as i64;

    return instances_found == qty;
}

fn has_at_least_qty<T: Eq>(vec: &Vec<T>, val_to_restrict: &T, qty: i64) -> bool {
    let mut qty_found = 0;

    for val in vec {
        if val == val_to_restrict {
            qty_found += 1;

            if qty_found >= qty {
                return true;
            }
        }
    }

    return qty_found >= qty; // account for 0
}
