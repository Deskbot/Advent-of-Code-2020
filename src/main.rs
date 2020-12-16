use std::env;

mod console;
mod day;
mod grid;
mod point;
mod util;

use day::{
    day01::*,
    day02::*,
    day03::*,
    day04::*,
    day05::*,
    day06::*,
    day07::*,
    day08::*,
    day09::*,
    day10::*,
    day11::*,
    day12::*,
    day13::*,
    day14::*,
    day15::*,
    day16::*,
};

fn main() {
    let mut args = env::args();

    match args.nth(1) {
        Some(day) => {
            let day_num = day.parse::<usize>().expect("Invalid day given.");
            run_day(day_num);
        },
        None => panic!("No day given"),
    }
}

fn run_day(day: usize) {
    let days = [
        day01,
        day02,
        day03,
        day04,
        day05,
        day06,
        day07,
        day08,
        day09,
        day10,
        day11,
        day12,
        day13,
        day14,
        day15,
        day16,
    ];

    let day_index = day - 1;
    let day_func = days.get(day_index).expect("Invalid day given.");

    println!("Day {}", day);
    day_func();
}
