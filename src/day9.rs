use crate::stopwatch::time;
use std::fs::read_to_string;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-9-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-9-example.txt", err
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-9-input.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-9-input.txt", err
        )
    });

    time("Day 9, Part 1 Example", || {
        run_part_1(example_data.as_str());
    });
    time("Day 9, Part 1 Puzzle", || {
        run_part_1(puzzle_data.as_str());
    });

    time("Day 9, Part 2 Example", || {
        run_part_2(example_data.as_str());
    });
    time("Day 9, Part 2 Puzzle", || {
        run_part_2(puzzle_data.as_str());
    });
}

fn run_part_1(input: &str) -> u64 {
    let coordinates = parse_coordinates(input);

    let largest_rectangle = find_largest_rectangle(&coordinates);

    let size = largest_rectangle.0.calculate_area(&largest_rectangle.1);

    println!("Largest rectangle has an area of {size}");

    size
}

fn run_part_2(_input: &str) -> u64 {
    // Create the perimeter of the grid

    // loop through all rectangles

    // Create function to test if a coordinate is within a rectangle

    // Filter by any rectangle that is entirely within the perimeter


    0
}

#[derive(Debug, Clone, Copy)]
struct Coordinate {
    x: u64,
    y: u64,
}

impl Coordinate {
    fn calculate_area(&self, other: &Coordinate) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}

fn parse_coordinates(input: &str) -> Vec<Coordinate> {
    input.lines()
        .map(|line| parse_coordinate(line))
        .collect()
}

fn parse_coordinate(input: &str) -> Coordinate {
    let mut split = input.split(',');
    Coordinate {
        x: split.next().unwrap_or_else(|| panic!("Coordinate {input} was missing X component")).parse::<u64>().unwrap(),
        y: split.next().unwrap_or_else(|| panic!("Coordinate {input} was missing Y component")).parse::<u64>().unwrap(),
    }
}

fn find_largest_rectangle(coordinates: &[Coordinate]) -> (Coordinate, Coordinate) {
    let mut largest_area: Option<(Coordinate, Coordinate)> = None;
    for (index, first) in coordinates.iter().enumerate() {
        for second in coordinates[index..].iter() {
            let area = first.calculate_area(second);
            if largest_area.is_none() {
                largest_area = Some((first.clone(), second.clone()));
            } else if area > largest_area.unwrap().0.calculate_area(&largest_area.unwrap().1) {
                largest_area = Some((first.clone(), second.clone()));
            }
        }
    }

    largest_area.unwrap_or_else(|| panic!("No squares found"))
}

#[test]
fn test_run_example_part_1() {
    let path = "./puzzle-inputs/day-9-example.txt";
    let example_data =
        read_to_string(path).unwrap_or_else(|err| panic!("Failed to read file {path}: {err}"));
    assert_eq!(run_part_1(example_data.as_str()), 50);
}

#[test]
fn test_run_example_part_2() {
    let path = "./puzzle-inputs/day-9-example.txt";
    let example_data =
        read_to_string(path).unwrap_or_else(|err| panic!("Failed to read file {path}: {err}"));
    assert_eq!(run_part_2(example_data.as_str()), 25272);
}