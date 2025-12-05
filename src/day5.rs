use crate::stopwatch::time;
use std::fs::read_to_string;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-5-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-5-example.txt", err
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-5-input.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-5-input.txt", err
        )
    });

    time("Day 5, Part 1 Example", || {
        run_part_1(example_data.as_str())
    });
    time("Day 5, Part 1 Puzzle", || run_part_1(puzzle_data.as_str()));

    time("Day 5, Part 2 Example", || {
        run_part_2(example_data.as_str())
    });
    time("Day 5, Part 2 Puzzle", || run_part_2(puzzle_data.as_str()));
}

fn run_part_1(input: &str) {
    let mut sections = input.split("\r\n\r\n");
    let ranges_text = sections.next().expect("No ranges section found in input");
    let ids_text = sections.next().expect("No ids section found in input");

    let ranges = read_ingredient_ranges(ranges_text);
    let ingredients = read_ingredients(ids_text);

    let mut total_fresh_ingredients = 0;

    'ingredient_id_iterator: for ingredient_id in ingredients {
        for range in &ranges {
            if range.includes(ingredient_id) {
                total_fresh_ingredients += 1;
                continue 'ingredient_id_iterator;
            }
        }
    }

    println!(
        "The total number of fresh ingredients is {}",
        total_fresh_ingredients
    );
}

fn run_part_2(_input: &str) {
    unimplemented!()
}

struct IngredientRange {
    first_id: u64,
    last_id: u64,
}

impl IngredientRange {
    fn includes(&self, id: u64) -> bool {
        id >= self.first_id && id <= self.last_id
    }
}

fn read_ingredient_ranges(input: &str) -> Vec<IngredientRange> {
    let mut ids: Vec<IngredientRange> = Vec::new();

    for id_range_text in input.lines() {
        if id_range_text.is_empty() {
            break;
        }

        let split_at = id_range_text.find('-').unwrap_or_else(|| {
            panic!(
                "range of '{}' did not contain a `-` to split on",
                id_range_text
            )
        });
        let (first_id_text, last_id_text) = id_range_text.split_at(split_at);
        let last_id_text = &last_id_text[1..last_id_text.len()]; // Remove the leading `-` from the start of the last_id_text

        let range = IngredientRange {
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

fn read_ingredients(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|id_text| {
            id_text
                .parse()
                .unwrap_or_else(|e| panic!("Failed parsing id `{}`: {}", id_text, e))
        })
        .collect()
}
