use std::env;

mod day;

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
        day::day01::day01,
        day::day02::day02,
    ];

    let day_func = days.get(day as usize).expect("Invalid day given.");

    println!("Day {}", day);
    day_func();
}
