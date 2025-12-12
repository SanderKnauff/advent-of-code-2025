use crate::stopwatch::time;
use std::error::Error;
use std::fs::read_to_string;
use crate::day9::geometry_primitives::{Coordinate, Edge};
use crate::day9::perimeter::Perimeter;
use crate::day9::rectangle::Rectangle;

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

    let size = largest_rectangle.calculate_area();

    println!("Largest rectangle has an area of {size}");

    size
}

fn run_part_2(input: &str) -> u64 {
    let coordinates = parse_coordinates(input);
    let rectangles = create_rectangles(&coordinates);

    let perimeter = create_perimeter(&coordinates);

    let largest_fitting_rectangle = rectangles
        .iter()
        .filter(|rectangle| perimeter.contains(rectangle))
        .max_by(|first, second| first.calculate_area().cmp(&second.calculate_area()))
        .expect("Could not find any fitting rectangle");

    println!(
        "The largest fitting rectangle in the perimeter is {:?} with an area of {}",
        largest_fitting_rectangle,
        largest_fitting_rectangle.calculate_area()
    );

    // Create the perimeter of the grid

    // loop through all rectangles

    // Create function to test if a coordinate is within a rectangle

    // Filter by any rectangle that is entirely within the perimeter

    largest_fitting_rectangle.calculate_area()
}

pub fn parse_coordinates(input: &str) -> Vec<Coordinate> {
    input.lines().map(parse_coordinate).collect()
}

fn parse_coordinate(input: &str) -> Coordinate {
    let mut split = input.split(',');
    Coordinate {
        x: split
            .next()
            .unwrap_or_else(|| panic!("Coordinate {input} was missing X component"))
            .parse::<u64>()
            .unwrap(),
        y: split
            .next()
            .unwrap_or_else(|| panic!("Coordinate {input} was missing Y component"))
            .parse::<u64>()
            .unwrap(),
    }
}

pub fn create_perimeter(coordinates: &[Coordinate]) -> Perimeter {
    let mut edges: Vec<Edge> = Vec::new();

    let mut coordinate_iterator = coordinates.iter();
    let first_coordinate = coordinate_iterator.next().unwrap_or_else(|| {
        panic!("Failed creating perimeter: Coordinate slice did not return a first value.")
    });

    let mut current_coordinate = first_coordinate;
    for next_coordinate in coordinate_iterator {
        let edge = Edge::new(current_coordinate, next_coordinate).unwrap_or_else(|err| panic!("{err}"));
        edges.push(edge);
        current_coordinate = next_coordinate;
    }

    let edge = Edge::new(current_coordinate, first_coordinate).unwrap_or_else(|err| panic!("{err}"));
    edges.push(edge);

    Perimeter { edges }
}

fn create_rectangles(coordinates: &[Coordinate]) -> Vec<Rectangle> {
    let mut rectangles: Vec<Rectangle> = Vec::new();
    for (index, first) in coordinates.iter().enumerate() {
        for second in coordinates[index..].iter() {
            rectangles.push(Rectangle {
                first_corner: *first,
                second_corner: *second,
            })
        }
    }

    rectangles
}

fn find_largest_rectangle(coordinates: &[Coordinate]) -> Rectangle {
    *create_rectangles(coordinates)
        .iter()
        .max_by(|first, second| first.calculate_area().cmp(&second.calculate_area()))
        .expect("Could not find rectangle")
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
