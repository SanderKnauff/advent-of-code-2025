use std::collections::{HashMap, HashSet};
use crate::stopwatch::time;
use std::fs::read_to_string;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-7-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-7-example.txt", err
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-7-input.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-7-input.txt", err
        )
    });

    time("Day 7, Part 1 Example", || {
        run_part_1(example_data.as_str());
    });
    time("Day 7, Part 1 Puzzle", || {
        run_part_1(puzzle_data.as_str());
    });

    time("Day 7, Part 2 Example", || {
        run_part_2(example_data.as_str());
    });
    time("Day 7, Part 2 Puzzle", || {
        run_part_2(puzzle_data.as_str());
    });
}

fn run_part_1(input: &str) -> u32 {
    let diagram = parse_manifold_diagram(input);

    let split_count = shoot_beam(diagram);

    println!("The tachyon beam splits {split_count} times.");

    split_count
}

fn run_part_2(_input: &str) -> u32 {
 0
}

fn shoot_beam(diagram: TachyonManifoldDiagram) -> u32 {
    let mut beam_origins: HashSet<Coordinate> = HashSet::new();
    beam_origins.insert(diagram.beam_origin);

    let mut vertical_beams: Vec<VerticalBeam> = Vec::new(); // Performance potential: Swap Vec with HashMap that buckets beams by X coordinate.

    while !beam_origins.is_empty() {
        let mut origins_for_iteration: HashSet<Coordinate> = HashSet::new();
        beam_origins.iter().for_each(|origin| {
            origins_for_iteration.insert(origin.clone());
        });
        beam_origins.clear();

        for origin in origins_for_iteration {
            if vertical_beams.iter().any(|beam| beam.intersects(origin)) {
                continue; // We already have seen this beam, so ignore it.
            }

            let vec = Vec::new();
            let splitters = diagram.splitters_per_x_coordinate.get(&(origin.x as usize))
                .unwrap_or(&vec);

            let splitter = splitters.iter().find(|splitter| splitter.y > origin.y);
            match splitter {
                None => {},
                Some(splitter) => {
                    vertical_beams.push(VerticalBeam { x: origin.x, y_min: origin.y, y_max: splitter.y});

                    let split_left = Coordinate { x: splitter.x - 1, y: splitter.y };
                    let split_right = Coordinate { x: splitter.x + 1, y: splitter.y };

                    beam_origins.insert(split_left);
                    beam_origins.insert(split_right);
                },
            }
        }
    }

    vertical_beams.iter().map(|b| Coordinate { x: b.x, y: b.y_max}).collect::<HashSet<_>>().len() as u32
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

struct VerticalBeam {
    x: i32,

    y_min: i32,
    y_max: i32,
}

impl VerticalBeam {
    fn intersects(&self, coordinate: Coordinate) -> bool {
        if self.x != coordinate.x {
            return false
        }

        coordinate.y >= self.y_min && coordinate.y <= self.y_max
    }
}

struct TachyonManifoldDiagram {
    beam_origin: Coordinate,
    splitters_per_x_coordinate: HashMap<usize, Vec<Coordinate>>,
}

fn parse_manifold_diagram(input: &str) -> TachyonManifoldDiagram {
    let input = input.replace('\r', "");
    let lines = input.split('\n');

    let mut beam_origin: Option<Coordinate> = None;
    let mut splitters_per_x_coordinate: HashMap<usize, Vec<Coordinate>> = HashMap::new();
    for (line_index, line_text) in lines.enumerate() {
        for (character_index, character) in line_text.chars().enumerate() {
            match character {
                'S' => beam_origin = Some(Coordinate { x: character_index as i32, y: line_index as i32}),
                '^' => {
                    let position = Coordinate { x: character_index as i32, y: line_index as i32 };
                    let splitters = splitters_per_x_coordinate.entry(position.x as usize).or_insert(Vec::new());
                    splitters.push(position);
                }
                '.' => {}
                other => panic!("Parsed unexpected character '{other}' at line {line_index}, char {character_index}")
            }
        }
    }

    let beam_origin = beam_origin.expect("Did not find any beam origin in the input");

    TachyonManifoldDiagram {
        beam_origin,
        splitters_per_x_coordinate
    }
}

#[test]
fn test_run_example_part_1() {
    let path = "./puzzle-inputs/day-7-example.txt";
    let example_data = read_to_string(path).unwrap_or_else(|err| {
        panic!("Failed to read file {path}: {err}")
    });
    assert_eq!(run_part_1(example_data.as_str()), 21);
}

#[test]
fn test_run_example_part_2() {
    let path = "./puzzle-inputs/day-7-example.txt";
    let example_data = read_to_string(path).unwrap_or_else(|err| {
        panic!("Failed to read file {path}: {err}")
    });
    assert_eq!(run_part_2(example_data.as_str()), 40);
}

#[test]
fn test_shoot_beam_with_one_splitter() {
    let mut splitters_per_x_coordinate = HashMap::new();
    splitters_per_x_coordinate.insert(0, vec![Coordinate { x: 0, y: 1 }]);

    let diagram = TachyonManifoldDiagram {
        beam_origin: Coordinate { x: 0, y: 0 },
        splitters_per_x_coordinate,
    };

    let splits = shoot_beam(diagram);

    assert_eq!(splits, 1);
}

#[test]
fn test_shoot_beam_with_two_splitters() {
    let mut splitters_per_x_coordinate = HashMap::new();
    splitters_per_x_coordinate.insert(0, vec![Coordinate { x: 0, y: 1 }]);
    splitters_per_x_coordinate.insert(1, vec![Coordinate { x: 1, y: 3 }]);

    let diagram = TachyonManifoldDiagram {
        beam_origin: Coordinate { x: 0, y: 0 },
        splitters_per_x_coordinate,
    };

    let splits = shoot_beam(diagram);

    assert_eq!(splits, 2);
}

#[test]
fn test_shoot_beam_with_two_splitters_that_share_a_target() {
    let mut splitters_per_x_coordinate = HashMap::new();
    splitters_per_x_coordinate.insert(1, vec![Coordinate { x: 1, y: 1 }]); // First hit

    splitters_per_x_coordinate.insert(0, vec![Coordinate { x: 0, y: 3 }]);
    splitters_per_x_coordinate.insert(2, vec![Coordinate { x: 2, y: 3 }]);

    splitters_per_x_coordinate.insert(10, vec![Coordinate { x:10, y: 3 }]); // Should not hit

    let diagram = TachyonManifoldDiagram {
        beam_origin: Coordinate { x: 1, y: 0 },
        splitters_per_x_coordinate,
    };

    let splits = shoot_beam(diagram);

    assert_eq!(splits, 3);
}
