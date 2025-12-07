use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod stopwatch;

fn main() {
    let mut args = env::args();
    let first_argument = args.nth(1);

    let Some(first_argument) = first_argument else {
        println!("Please enter a day number");
        return;
    };

    match first_argument.as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        "5" => day5::run(),
        "6" => day6::run(),
        unmatched_number => println!("Unknown day: {unmatched_number}"),
    }
}
