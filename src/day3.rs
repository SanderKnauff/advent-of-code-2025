use crate::stopwatch::time;
use std::fs::read_to_string;
use std::str::FromStr;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-3-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-3-example.txt", err
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-3-input.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-3-input.txt", err
        )
    });

    time("Day 3, Part 1 Example", || {
        run_part_1(example_data.as_str())
    });
    time("Day 3, Part 1 Puzzle", || run_part_1(puzzle_data.as_str()));

    time("Day 3, Part 2 Example", || {
        run_part_2(example_data.as_str())
    });
    time("Day 3, Part 2 Puzzle", || run_part_2(puzzle_data.as_str()));
}

struct Bank {
    // Joltage is not a typo. It's part of the AoC **domain**.
    battery_joltages: Vec<u8>,
}

fn run_part_1(input: &str) {
    let banks = parse_banks(input);

    let mut total_output_joltage = 0_u16;

    for bank in banks {
        total_output_joltage += find_highest_joltage_combined_from_n_batteries(&bank, 2) as u16;
    }

    println!("The total output joltage is {}", total_output_joltage);
}

fn run_part_2(input: &str) {
    let banks = parse_banks(input);

    let mut total_output_joltage: u64 = 0;

    for bank in banks {
        total_output_joltage += find_highest_joltage_combined_from_n_batteries(&bank, 12);
    }

    println!("The total output joltage is {}", total_output_joltage);
}

fn parse_banks(input: &str) -> Vec<Bank> {
    let mut banks: Vec<Bank> = Vec::new();

    for bank_string in input.lines() {
        let mut bank = Bank {
            battery_joltages: Vec::new(),
        };
        for joltage_text in bank_string.chars() {
            let joltage: u8 = u8::from_str(joltage_text.to_string().as_str())
                .unwrap_or_else(|e| panic!("Failed parsing Joltage {}: {}", joltage_text, e));

            bank.battery_joltages.push(joltage)
        }

        banks.push(bank);
    }

    banks
}

fn find_highest_joltage_combined_from_n_batteries(
    bank: &Bank,
    amount_of_batteries_to_combine: u8,
) -> u64 {
    let mut highest_joltages: Vec<u8> = vec![0; amount_of_batteries_to_combine as usize];
    let mut lowest_possible_index = 0;

    for highest_joltage_index in 0..highest_joltages.len() {
        let highest_possible_battery_index =
            bank.battery_joltages.len() - highest_joltages.len() + highest_joltage_index;

        let offset_array = lowest_possible_index;
        for (index_in_bank, joltage_in_bank) in bank.battery_joltages
            [lowest_possible_index..=highest_possible_battery_index]
            .iter()
            .enumerate()
        {
            if joltage_in_bank > &highest_joltages[highest_joltage_index] {
                highest_joltages[highest_joltage_index] = *joltage_in_bank;
                lowest_possible_index = index_in_bank + offset_array + 1;
            }
        }
    }

    let mut total_joltage: u64 = 0;
    for (index, joltage) in highest_joltages.iter().enumerate() {
        let position: u32 = (highest_joltages.len() - index - 1) as u32;

        let multiplier = 10_u64.pow(position);
        total_joltage += *joltage as u64 * multiplier;
    }

    total_joltage
}

#[test]
fn test_parse_banks() {
    assert_eq!(
        parse_banks("987654321111111")[0].battery_joltages,
        vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1]
    );
    assert_eq!(
        parse_banks("811111111111119")[0].battery_joltages,
        vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9]
    );
}

#[test]
fn test_find_highest_joltage_from_n_batteries() {
    assert_eq!(
        find_highest_joltage_combined_from_n_batteries(&parse_banks("81191")[0], 3),
        891
    );
}

#[test]
fn test_find_highest_joltage_from_two_batteries() {
    assert_eq!(
        find_highest_joltage_combined_from_n_batteries(&parse_banks("987654321111111")[0], 2),
        98
    );
    assert_eq!(
        find_highest_joltage_combined_from_n_batteries(&parse_banks("811111111111119")[0], 2),
        89
    );
    assert_eq!(
        find_highest_joltage_combined_from_n_batteries(&parse_banks("234234234234278")[0], 2),
        78
    );
    assert_eq!(
        find_highest_joltage_combined_from_n_batteries(&parse_banks("818181911112111")[0], 2),
        92
    );
}

#[test]
fn test_find_highest_joltage_combined_from_12_batteries() {
    assert_eq!(
        find_highest_joltage_combined_from_n_batteries(&parse_banks("987654321111111")[0], 12),
        987654321111
    );
    assert_eq!(
        find_highest_joltage_combined_from_n_batteries(&parse_banks("811111111111119")[0], 12),
        811111111119
    );
    assert_eq!(
        find_highest_joltage_combined_from_n_batteries(&parse_banks("234234234234278")[0], 12),
        434234234278
    );
    assert_eq!(
        find_highest_joltage_combined_from_n_batteries(&parse_banks("818181911112111")[0], 12),
        888911112111
    );
}
