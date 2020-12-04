use std::env;

mod day;
mod util;

use day::{
    day01::*,
    day02::*,
    day03::*,
    day04::*,
};

fn main() {
    let mut args = env::args();

    match args.nth(1) {
        Some(day) => {
            let day_num = day.parse::<u8>().expect("Invalid day given.");
            run_day(day_num);
        },
        None => panic!("No day given"),
    }
}

fn run_day(day: u8) {
    let days = [
        day01,
        day02,
        day03,
        day04,
    ];

    let day_index = (day - 1) as usize;
    let day_func = days.get(day_index).expect("Invalid day given.");

    println!("Day {}", day);
    day_func();
}
