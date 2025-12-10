use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day9visualizer;
mod stopwatch;

fn main() {
    let mut args = env::args();
    let first_argument = args.nth(1);

    let first_argument = match first_argument {
        Some(arg) => arg,
        None => {
            println!("Please enter a day number");
            return;
        }
    };

    match first_argument.as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        "5" => day5::run(),
        "6" => day6::run(),
        "7" => day7::run(),
        "8" => day8::run(),
        "9" => day9::run(),
        n => println!("Unknown day: {}", n),
    }
}
