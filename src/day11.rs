use crate::stopwatch::time;
use std::fs::read_to_string;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-11-example.txt").unwrap_or_else(|_| {
        panic!(
            "Failed to read file {}",
            "./puzzle-inputs/day-11-example.txt"
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-11-input.txt")
        .unwrap_or_else(|_| panic!("Failed to read file {}", "./puzzle-inputs/day-11-input.txt"));

    time("Day 11, Part 1 Example", || {
        run_part_1(example_data.as_str())
    });
    time("Day 11, Part 1 Puzzle", || run_part_1(puzzle_data.as_str()));

    time("Day 11, Part 2 Example", || {
        run_part_2(example_data.as_str())
    });
    time("Day 11, Part 2 Puzzle", || run_part_2(puzzle_data.as_str()));
}

fn run_part_1(input: &str) {}

fn run_part_2(_input: &str) {}
