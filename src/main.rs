use std::env;

mod day1;
mod day2;
mod day3;
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
        n => println!("Unknown day: {}", n),
    }
}
