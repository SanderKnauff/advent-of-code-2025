use crate::stopwatch::time;
use std::fs::read_to_string;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-2-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-2-example.txt", err
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-2-input.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-2-input.txt", err
        )
    });

    time("Day 2, Part 1 Example", || {
        run_part_1(example_data.as_str())
    });
    time("Day 2, Part 1 Puzzle", || run_part_1(puzzle_data.as_str()));

    time("Day 2, Part 2 Example", || {
        run_part_2(example_data.as_str())
    });
    time("Day 2, Part 2 Puzzle", || run_part_2(puzzle_data.as_str()));
}

struct IdRange {
    first_id: u64,
    last_id: u64,
}

fn read_id_ranges(input: &str) -> Vec<IdRange> {
    let mut ids: Vec<IdRange> = Vec::new();

    for id_range_text in input.split(',') {
        let split_at = id_range_text.find('-').unwrap_or_else(|| {
            panic!(
                "range of '{}' did not contain a `-` to split on",
                id_range_text
            )
        });
        let (first_id_text, last_id_text) = id_range_text.split_at(split_at);
        let last_id_text = &last_id_text[1..last_id_text.len()]; // Remove the leading `-` from the start of the last_id_text

        let range = IdRange {
            first_id: first_id_text
                .parse()
                .unwrap_or_else(|e| panic!("Failed parsing id `{}`: {}", first_id_text, e)),
            last_id: last_id_text
                .parse()
                .unwrap_or_else(|e| panic!("Failed parsing id `{}`: {}", last_id_text, e)),
        };

        ids.push(range);
    }

    ids
}

fn run_part_1(input: &str) {
    let mut sum_of_invalid_ids: u64 = 0;

    for range in read_id_ranges(input).iter() {
        for id in range.first_id..=range.last_id {
            if has_twice_repeated_number_sequence(id) {
                sum_of_invalid_ids += id;
            }
        }
    }

    println!(
        "The sum of all ids which are composed of a single repeated sequence is {}",
        sum_of_invalid_ids
    );
}

fn run_part_2(input: &str) {
    let mut sum_of_invalid_ids: u64 = 0;

    for range in read_id_ranges(input).iter() {
        for id in range.first_id..=range.last_id {
            if has_any_repeated_number_sequence(id) {
                sum_of_invalid_ids += id;
            }
        }
    }

    println!(
        "The sum of all ids which are composed of multiple repeating sequences is {}",
        sum_of_invalid_ids
    );
}

fn has_twice_repeated_number_sequence(id: u64) -> bool {
    // 1. Find the logarithmic size of the id
    // 2. If even, split the number into two parts.
    // 3. Compare the two parts.
    let digits_in_number: u64 = (id.ilog10() + 1) as u64;

    // Odd numbers cannot be an exact twice repeating value.
    if digits_in_number % 2 == 1 {
        return false;
    }

    let divisor = 10_u64.pow(digits_in_number as u32 / 2);
    let high_part = id / divisor;
    let low_part = id - (high_part * divisor);

    high_part == low_part
}

fn has_any_repeated_number_sequence(id: u64) -> bool {
    let digits_in_id = id.ilog10() + 1;
    let max_group_size = digits_in_id / 2;

    'group_size_iterator: for group_size in (1..=max_group_size).rev() {
        if !digits_in_id.is_multiple_of(group_size) {
            // If the number of digits is not divisible by the group size, skip the group.
            continue 'group_size_iterator;
        }

        let number_of_potential_groups = digits_in_id / group_size;
        let to_match = find_grouped_digits_by_group_size_and_index(id, group_size as u64, 0);

        for group in 1..number_of_potential_groups {
            let found_group =
                find_grouped_digits_by_group_size_and_index(id, group_size as u64, group as u64);
            if found_group != to_match {
                // If there is no match, continue
                continue 'group_size_iterator;
            }
        }

        // All groups match!
        return true;
    }

    false
}

fn find_grouped_digits_by_group_size_and_index(
    number_to_shift: u64,
    group_size: u64,
    group_index: u64,
) -> u64 {
    let total_digits: u64 = number_to_shift.ilog10() as u64 + 1;
    let total_groups: u64 = total_digits / group_size;

    let digits_to_remove_right = (total_groups - (group_index + 1)) * group_size;

    let id_right_removed = number_to_shift / (10_u64.pow(digits_to_remove_right as u32));
    let redundant_left = id_right_removed / 10_u64.pow(group_size as u32);
    let redundant_left = redundant_left * 10_u64.pow(group_size as u32);

    id_right_removed - redundant_left
}

#[test]
fn test_has_twice_repeated_number_sequence() {
    assert!(has_twice_repeated_number_sequence(11));
    assert!(has_twice_repeated_number_sequence(22));
    assert!(has_twice_repeated_number_sequence(99));
    assert!(has_twice_repeated_number_sequence(1010));
    assert!(has_twice_repeated_number_sequence(1188511885));
    assert!(has_twice_repeated_number_sequence(222222));
    assert!(has_twice_repeated_number_sequence(446446));
    assert!(has_twice_repeated_number_sequence(38593859));

    assert!(!has_twice_repeated_number_sequence(1));
    assert!(!has_twice_repeated_number_sequence(111));
    assert!(!has_twice_repeated_number_sequence(1241));
    assert!(!has_twice_repeated_number_sequence(6643));
    assert!(!has_twice_repeated_number_sequence(41246));
    assert!(!has_twice_repeated_number_sequence(13225));
    assert!(!has_twice_repeated_number_sequence(2156));
    assert!(!has_twice_repeated_number_sequence(12357));
    assert!(!has_twice_repeated_number_sequence(84532));
}

#[test]
fn test_has_any_repeated_number_sequence() {
    assert!(has_any_repeated_number_sequence(11));
    assert!(has_any_repeated_number_sequence(1212));
    assert!(has_any_repeated_number_sequence(121212));
    assert!(has_any_repeated_number_sequence(123123));
}

#[test]
fn test_inverted_has_repeated_number_sequence() {
    assert!(!has_any_repeated_number_sequence(12));
    assert!(!has_any_repeated_number_sequence(12312));
}

#[test]
fn show_logarithmics() {
    println!("{}", 1_u64.ilog10());
    println!("{}", 9_u64.ilog10());
    println!("{}", 10_u64.ilog10());
    println!("{}", 99_u64.ilog10());
    println!("{}", 100_u64.ilog10());
}

#[test]
fn show_calculation_validations() {
    println!("Expected result: {}", 123_456 / 1000);

    let number_to_split: u64 = 123_456;
    let digits_in_number: u64 = (number_to_split.ilog10() + 1) as u64;
    println!("Digits in number: {}", digits_in_number);
    println!("Power of 10: {}", 10_u64.pow(digits_in_number as u32));
    println!("Power of 10: {}", 10_u64.pow(digits_in_number as u32 / 2));
    println!(
        "High part: {}",
        number_to_split / (10_u64.pow(digits_in_number as u32 / 2))
    );
    println!(
        "Low part: {}",
        number_to_split / (10_u64.pow(digits_in_number as u32 / 2))
    );
}

#[test]
fn show_calculate_shifting_decimals() {
    let number_to_shift = 123_456_789_246;

    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 3, 0),
        123
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 3, 1),
        456
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 3, 2),
        789
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 3, 3),
        246
    );

    // Same number as before, but displayed in pairs of 2
    let number_to_shift = 12_34_56_78_92_46;
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 2, 0),
        12
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 2, 1),
        34
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 2, 2),
        56
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 2, 3),
        78
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 2, 4),
        92
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 2, 5),
        46
    );

    // Same number as before, but displayed in pairs of 4
    let number_to_shift = 1234_5678_9246;
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 4, 0),
        1234
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 4, 1),
        5678
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 4, 2),
        9246
    );

    let number_to_shift = 12;
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 1, 0),
        1
    );
    assert_eq!(
        find_grouped_digits_by_group_size_and_index(number_to_shift, 1, 1),
        2
    );
}
