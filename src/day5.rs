use crate::stopwatch::time;
use std::cmp::{max, min};
use std::fmt::{Display, Formatter};
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
    let sections = input.replace("\r", "");
    let mut sections = sections.split("\n\n");

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

fn run_part_2(input: &str) {
    let mut sections = input.split("\r\n\r\n");
    let ranges_text = sections.next().expect("No ranges section found in input");

    let ranges = read_ingredient_ranges(ranges_text);
    let merged_ranges = merge_ranges(ranges);

    let mut total = 0;
    for range in merged_ranges {
        total += range.size()
    }

    println!("The total number of fresh ingredients is {}", total);
}

#[derive(Debug, Clone, PartialEq)]
struct IngredientRange {
    first_id: u64,
    last_id: u64,
}

impl IngredientRange {
    fn includes(&self, id: u64) -> bool {
        id >= self.first_id && id <= self.last_id
    }

    fn size(self) -> u64 {
        (self.last_id - self.first_id) + 1
    }

    fn overlaps(&self, other: &IngredientRange) -> bool {
        self.includes(other.first_id)
            || self.includes(other.last_id)
            || other.includes(self.first_id)
            || other.includes(self.last_id)
    }
}

impl Display for IngredientRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.first_id, self.last_id))
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

fn merge_ranges(mut ranges: Vec<IngredientRange>) -> Vec<IngredientRange> {
    let mut merged_ranges = Vec::new();

    ranges.sort_unstable_by(|a, b| a.first_id.cmp(&b.first_id).then(a.last_id.cmp(&b.last_id)));

    let mut range_iterator = ranges.iter();
    let mut current_range = range_iterator
        .next()
        .expect("Range did not have a first element")
        .clone();
    let mut next_range = range_iterator.next();

    while next_range.is_some() {
        let range_to_inspect =
            next_range.expect("Next range was None even though we just checked for it");

        if range_to_inspect.overlaps(&current_range) {
            current_range.first_id = min(current_range.first_id, range_to_inspect.first_id);
            current_range.last_id = max(current_range.last_id, range_to_inspect.last_id);
        } else {
            merged_ranges.push(current_range);
            current_range = range_to_inspect.clone();
        }

        next_range = range_iterator.next();
    }
    merged_ranges.push(current_range);

    merged_ranges
}

#[test]
fn test_range_merging() {
    let ranges = vec![
        IngredientRange {
            first_id: 1,
            last_id: 3,
        },
        IngredientRange {
            first_id: 3,
            last_id: 5,
        },
        IngredientRange {
            first_id: 5,
            last_id: 10,
        },
    ];
    let merged_ranges = merge_ranges(ranges);
    assert_eq!(merged_ranges.len(), 1);
    assert_eq!(
        merged_ranges[0],
        IngredientRange {
            first_id: 1,
            last_id: 10
        },
        "Simple incremental ranges should work"
    );

    let ranges = vec![
        IngredientRange {
            first_id: 1,
            last_id: 3,
        },
        IngredientRange {
            first_id: 2,
            last_id: 6,
        },
        IngredientRange {
            first_id: 5,
            last_id: 10,
        },
    ];
    let merged_ranges = merge_ranges(ranges);
    assert_eq!(merged_ranges.len(), 1);
    assert_eq!(
        merged_ranges[0],
        IngredientRange {
            first_id: 1,
            last_id: 10
        },
        "Overlapping ranges are merged"
    );

    let ranges = vec![
        IngredientRange {
            first_id: 5,
            last_id: 10,
        },
        IngredientRange {
            first_id: 2,
            last_id: 6,
        },
        IngredientRange {
            first_id: 1,
            last_id: 3,
        },
    ];
    let merged_ranges = merge_ranges(ranges);
    assert_eq!(merged_ranges.len(), 1);
    assert_eq!(
        merged_ranges[0],
        IngredientRange {
            first_id: 1,
            last_id: 10
        },
        "Out-of-order ranges are resolved"
    );

    let ranges = vec![
        IngredientRange {
            first_id: 1,
            last_id: 4,
        },
        IngredientRange {
            first_id: 1,
            last_id: 3,
        },
        IngredientRange {
            first_id: 1,
            last_id: 3,
        },
        IngredientRange {
            first_id: 5,
            last_id: 10,
        },
    ];
    let merged_ranges = merge_ranges(ranges);
    assert_eq!(merged_ranges.len(), 2);
    assert_eq!(
        merged_ranges[0],
        IngredientRange {
            first_id: 1,
            last_id: 4
        },
        "Repeated ranges are merged"
    );
    assert_eq!(
        merged_ranges[1],
        IngredientRange {
            first_id: 5,
            last_id: 10
        },
        "Repeated ranges are merged"
    );

    let ranges = vec![
        IngredientRange {
            first_id: 5,
            last_id: 10,
        },
        IngredientRange {
            first_id: 1,
            last_id: 1,
        },
        IngredientRange {
            first_id: 2,
            last_id: 2,
        },
        IngredientRange {
            first_id: 3,
            last_id: 3,
        },
        IngredientRange {
            first_id: 4,
            last_id: 4,
        },
    ];
    let merged_ranges = merge_ranges(ranges);
    assert_eq!(merged_ranges.len(), 5);
    assert_eq!(
        merged_ranges[0],
        IngredientRange {
            first_id: 1,
            last_id: 1
        },
        "Repeated ranges are merged"
    );
    assert_eq!(
        merged_ranges[1],
        IngredientRange {
            first_id: 2,
            last_id: 2
        },
        "Repeated ranges are merged"
    );
    assert_eq!(
        merged_ranges[2],
        IngredientRange {
            first_id: 3,
            last_id: 3
        },
        "Repeated ranges are merged"
    );
    assert_eq!(
        merged_ranges[3],
        IngredientRange {
            first_id: 4,
            last_id: 4
        },
        "Repeated ranges are merged"
    );
    assert_eq!(
        merged_ranges[4],
        IngredientRange {
            first_id: 5,
            last_id: 10
        },
        "Repeated ranges are merged"
    );
}

#[test]
fn test_out_of_order_merge() {
    let ranges = vec![
        IngredientRange {
            first_id: 10,
            last_id: 20,
        },
        IngredientRange {
            first_id: 20,
            last_id: 30,
        },
        IngredientRange {
            first_id: 10,
            last_id: 50,
        },
    ];
    let merged_ranges = merge_ranges(ranges);
    assert_eq!(merged_ranges.len(), 1);
    assert_eq!(
        merged_ranges[0],
        IngredientRange {
            first_id: 10,
            last_id: 50
        },
        "Repeated ranges are merged"
    );
}

#[test]
fn test_range_size() {
    assert_eq!(
        IngredientRange {
            first_id: 1,
            last_id: 1
        }
        .size(),
        1,
        "Has correct size"
    );
    assert_eq!(
        IngredientRange {
            first_id: 4,
            last_id: 4
        }
        .size(),
        1,
        "Has correct size"
    );
    assert_eq!(
        IngredientRange {
            first_id: 1,
            last_id: 4
        }
        .size(),
        4,
        "Has correct size"
    );
    assert_eq!(
        IngredientRange {
            first_id: 5,
            last_id: 10
        }
        .size(),
        6,
        "Has correct size"
    );
}
