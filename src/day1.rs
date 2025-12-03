use std::fmt::{Display, Formatter};
use std::fs::read_to_string;

const DIAL_MAX: u8 = 100;

use crate::stopwatch::time;
use Turn::Left;
use Turn::Right;
use std::str::FromStr;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-1-example.txt").unwrap_or_else(|_| {
        panic!(
            "Failed to read file {}",
            "./puzzle-inputs/day-1-example.txt"
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-1-input.txt")
        .unwrap_or_else(|_| panic!("Failed to read file {}", "./puzzle-inputs/day-1-input.txt"));

    time("Day 1, Part 1 Example", || {
        run_part_1(example_data.as_str())
    }); // 3
    time("Day 1, Part 1 Puzzle", || run_part_1(puzzle_data.as_str()));

    time("Day 1, Part 2 Example", || {
        run_part_2(example_data.as_str())
    }); // 6
    time("Day 1, Part 2 Puzzle", || run_part_2(puzzle_data.as_str()));
}

struct Dial {
    position: u8,
}

enum Turn {
    Left(u16),
    Right(u16),
}

impl Display for Turn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Left(n) => f.write_fmt(format_args!("L{}", n)),
            Right(n) => f.write_fmt(format_args!("R{}", n)),
        }
    }
}

impl Dial {
    /// Turns the dial according to direction and amount of turn.
    /// Returns the amount of times that 0 was passed.
    fn turn(&mut self, turn: Turn) -> u16 {
        let mut next_value: i16 = self.position.into();

        next_value += match turn {
            Right(n) => n as i16,
            Left(n) => -(n as i16),
        };

        let from = self.position as i16;
        let to = next_value;

        // Handle the overflow
        next_value %= DIAL_MAX as i16;
        if next_value < 0 {
            next_value = DIAL_MAX as i16 - next_value.abs();
        }

        self.position = next_value.try_into().unwrap_or_else(|_| {
            panic!(
                "next_value {} was out of bounds for dial position",
                next_value
            )
        });

        print!(
            "The dial is rotated {} to point at {} ({})",
            turn, self.position, to
        );

        let mut amount_of_0s_hit = 0;
        if to <= 0 {
            amount_of_0s_hit = to / -(DIAL_MAX as i16) + 1;
            println!(
                "; during this rotation, it points to 0 {} times.",
                amount_of_0s_hit
            );

            if from == 0 {
                amount_of_0s_hit -= 1;
            }
        } else if to > 99 {
            amount_of_0s_hit = to / (DIAL_MAX as i16);
            println!(
                "; during this rotation, it points to 0 {} times.",
                amount_of_0s_hit
            );
        } else {
            println!(".");
        }

        amount_of_0s_hit as u16
    }
}

fn run_part_1(input: &str) {
    let mut dial = Dial { position: 50 };
    let mut amount_of_0_positions = 0;

    input.lines().for_each(|line| {
        let direction = parse_turn(&line);

        dial.turn(direction);

        if dial.position == 0 {
            amount_of_0_positions += 1;
        }
    });

    println!(
        "The dial position is {}, the amount of 0 positions found was {}.",
        dial.position, amount_of_0_positions
    );
}

fn run_part_2(input: &str) {
    let mut dial = Dial { position: 50 };

    let mut amount_of_0_positions = 0;

    input.lines().for_each(|line| {
        let direction = parse_turn(&line);

        amount_of_0_positions += dial.turn(direction);
    });

    println!(
        "The dial position is {}, the amount of times the dial hit 0 was {}.",
        dial.position, amount_of_0_positions
    );
}

fn parse_turn(line: &&str) -> Turn {
    let chars = &line[..1];
    let count = &line[1..line.len()];
    let count: u16 =
        u16::from_str(count).unwrap_or_else(|_| panic!("Failed parsing count {}", count));

    let direction = chars.chars().next();

    match direction {
        Some('R') => Right(count),
        Some('L') => Left(count),
        _ => {
            panic!("Unexpected direction")
        }
    }
}

#[test]
fn dial_should_work() {
    assert_eq!(
        Dial { position: 50 }.turn(Left(1)),
        0,
        "Turning left without hitting 0 should not count the times 0 was hit."
    );
    assert_eq!(
        Dial { position: 0 }.turn(Left(1)),
        0,
        "Turning left once from position 0 should not count 0 as being hit."
    );
    assert_eq!(
        Dial { position: 50 }.turn(Left(50)),
        1,
        "Turning left 50 times from from position 50 should count 0 as being hit."
    );
    assert_eq!(
        Dial { position: 50 }.turn(Left(49)),
        0,
        "Turning left 49 times from from position 50 should not count 0 as being hit."
    );
    assert_eq!(
        Dial { position: 1 }.turn(Left(1)),
        1,
        "Turning left once from position 1 should count as 0 being hit."
    );
    assert_eq!(
        Dial { position: 99 }.turn(Left(99)),
        1,
        "Turning left 99 times from position 99 should count as 0 being hit."
    );
    assert_eq!(
        Dial { position: 99 }.turn(Left(100)),
        1,
        "Turning left 100 times from position 99 should count as 0 being hit."
    );
    assert_eq!(
        Dial { position: 99 }.turn(Left(199)),
        2,
        "Turning left 199 times from position 99 should count as 0 being hit twice."
    );

    let mut dial = Dial { position: 99 };
    let zeroes = dial.turn(Left(100));
    assert_eq!(
        dial.position, 99,
        "Turning left 100 times from position 99 should set the position as 99."
    );
    assert_eq!(
        zeroes, 1,
        "Turning left 100 times from position 0 should count as 0 being hit once."
    );

    let mut dial = Dial { position: 0 };
    let zeroes = dial.turn(Left(101));

    assert_eq!(
        dial.position, 99,
        "Turning left 101 times from position 0 should set the position as 99."
    );
    assert_eq!(
        zeroes, 1,
        "Turning left 101 times from position 0 should count as 0 being hit once."
    );
}
