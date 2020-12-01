use std::env;

mod day;

fn main() {
    let mut args = env::args();

    match args.nth(1) {
        Some(day) => run_day(day),
        None => panic!("No day given"),
    }
}

fn run_day(day: String) {
    println!("Day {}", day);

    if day == "1" {
        day::day01::day01();
    }
}
