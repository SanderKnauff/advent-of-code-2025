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

    run_part_1(example_data.as_str());
    run_part_1(puzzle_data.as_str());

    // run_part_2(example_data.as_str());
    // run_part_2(puzzle_data.as_str());
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

    println!("The sum of all invalid ids is {}", sum_of_invalid_ids);
}

fn has_twice_repeated_number_sequence(id: u64) -> bool {
    // 1. Find the logarithmic size of the id
    // 2. If even, split the number into two parts.
    // 3. Compare the two parts.

    let digits_in_number: u64 = (id.ilog(10) + 1) as u64;

    // Odd numbers cannot be an exact twice repeating value.
    if digits_in_number % 2 == 1 {
        return false;
    }

    let divisor = 10_u64.pow(digits_in_number as u32 / 2);
    let high_part = id / divisor;
    let low_part = id - (high_part * divisor);

    high_part == low_part
}

#[test]
fn test_is_invalid_id() {
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
fn show_logarithmics() {
    println!("{}", 1_u64.ilog(10));
    println!("{}", 9_u64.ilog(10));
    println!("{}", 10_u64.ilog(10));
    println!("{}", 99_u64.ilog(10));
    println!("{}", 100_u64.ilog(10));
}

#[test]
fn show_calculation_validations() {
    println!("Expected result: {}", 123_456 / 1000);

    let number_to_split: u64 = 123_456;
    let digits_in_number: u64 = (number_to_split.ilog(10) + 1) as u64;
    println!("Digits in number: {}", digits_in_number);
    println!("Power of 10: {}", 10_u64.pow(digits_in_number as u32));
    println!("Power of 10: {}", 10_u64.pow(digits_in_number as u32 / 2));
    println!(
        "High part: {}",
        number_to_split / (10_u64.pow((digits_in_number as u32 / 2)))
    );
    println!(
        "Low part: {}",
        number_to_split / (10_u64.pow((digits_in_number as u32 / 2)))
    );
}
