use std::env;

mod console;
mod day;
mod grid;
mod point;
mod util;

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
        day::day01::day01,
        day::day02::day02,
        day::day03::day03,
        day::day04::day04,
        day::day05::day05,
        day::day06::day06,
        day::day07::day07,
        day::day08::day08,
        day::day09::day09,
        day::day10::day10,
        day::day11::day11,
        day::day12::day12,
        day::day13::day13,
        day::day14::day14,
        day::day15::day15,
        day::day16::day16,
        day::day17::day17,
        day::day18::day18,
        day::day19::day19,
        day::day20::day20,
        day::day21::day21,
        day::day22::day22,
        day::day23::day23,
    ];

    let day_index = day - 1;
    let day_func = days.get(day_index).expect("Invalid day given.");

    println!("Day {}", day);
    day_func();
}
