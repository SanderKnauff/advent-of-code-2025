use std::collections::HashSet;
use std::fs::read_to_string;

use crate::stopwatch::time;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-4-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-4-example.txt", err
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-4-input.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-4-input.txt", err
        )
    });

    time("Day 4, Part 1 Example", || {
        run_part_1(example_data.as_str())
    });
    time("Day 4, Part 1 Puzzle", || run_part_1(puzzle_data.as_str()));

    time("Day 4, Part 2 Example", || {
        run_part_2(example_data.as_str())
    });
    time("Day 4, Part 2 Puzzle", || run_part_2(puzzle_data.as_str()));
}

fn run_part_1(input: &str) {
    let roll_positions = parse_roll_positions(input);

    let rolls = find_rolls_with_less_than_4_neighbours(&roll_positions);

    println!(
        "The number of paper rolls with less than 4 neighbours is {}",
        rolls.len()
    );
}

fn run_part_2(input: &str) {
    let mut roll_positions = parse_roll_positions(input);
    let mut removed_rolls = 0;

    loop {
        let removable_rolls = find_rolls_with_less_than_4_neighbours(&roll_positions);

        if removable_rolls.is_empty() {
            println!("No more rolls can be removed.");
            break;
        }

        removed_rolls += removable_rolls.len();

        for roll in &removable_rolls {
            roll_positions.retain(|position| position != roll);
        }

        println!("Removed {} rolls this iteration", removable_rolls.len());
    }

    println!(
        "The of rolls that can be recursively removed is {}",
        removed_rolls
    );
}

fn parse_roll_positions(input: &str) -> HashSet<(u8, u8)> {
    let mut positions: HashSet<(u8, u8)> = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        for (x, column) in row.chars().enumerate() {
            if column == '@' {
                positions.insert((x as u8, y as u8));
            }
        }
    }

    positions
}

fn find_rolls_with_less_than_4_neighbours(positions: &HashSet<(u8, u8)>) -> HashSet<(u8, u8)> {
    let mut removable_roll_positions: HashSet<(u8, u8)> = HashSet::new();

    for position in positions {
        let neighbours = count_neighbours(position, positions);
        if neighbours < 4 {
            removable_roll_positions.insert(*position);
        }
    }

    removable_roll_positions
}

fn count_neighbours(position: &(u8, u8), positions: &HashSet<(u8, u8)>) -> u16 {
    let mut count: u16 = 0;

    count += match position.0.checked_sub(1) {
        Some(x) => {
            let mut count = 0;

            count += position
                .1
                .checked_sub(1)
                .map(|y| positions.contains(&(x, y)))
                .unwrap_or(false) as u16;
            count += positions.contains(&(x, position.1)) as u16;
            count += position
                .1
                .checked_add(1)
                .map(|y| positions.contains(&(x, y)))
                .unwrap_or(false) as u16;

            count
        }
        None => 0,
    };

    count += position
        .1
        .checked_sub(1)
        .map(|y| positions.contains(&(position.0, y)))
        .unwrap_or(false) as u16;
    count += position
        .1
        .checked_add(1)
        .map(|y| positions.contains(&(position.0, y)))
        .unwrap_or(false) as u16;

    count += match position.0.checked_add(1) {
        Some(x) => {
            let mut count = 0;

            count += u16::from(
                position
                    .1
                    .checked_sub(1)
                    .is_some_and(|y| positions.contains(&(x, y))),
            );
            count += u16::from(positions.contains(&(x, position.1)));
            count += u16::from(
                position
                    .1
                    .checked_add(1)
                    .is_some_and(|y| positions.contains(&(x, y))),
            );

            count
        }
        None => 0,
    };

    count
}
