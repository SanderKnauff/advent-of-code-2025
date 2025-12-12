use crate::stopwatch::time;
use std::collections::HashMap;
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

fn run_part_2(input: &str) -> u64 {
    let diagram = parse_manifold_diagram(input);

    let paths = simulate_tachyon_particles(diagram);

    println!("There are {paths} paths that a tachyon particle can take.");

    paths
}

fn shoot_beam(diagram: TachyonManifoldDiagram) -> u32 {
    let mut beam_origins: Vec<Coordinate> = Vec::new();
    beam_origins.push(diagram.beam_origin);

    let mut vertical_beams: Vec<VerticalBeam> = Vec::new(); // Performance potential: Swap Vec with HashMap that buckets beams by X coordinate.

    while !beam_origins.is_empty() {
        let mut origins_for_iteration: Vec<Coordinate> = Vec::new();
        beam_origins.iter().for_each(|origin| {
            origins_for_iteration.push(*origin);
        });
        beam_origins.clear();

        for origin in origins_for_iteration {
            if vertical_beams.iter().any(|beam| beam.intersects(origin)) {
                continue; // We already have seen this beam, so ignore it.
            }

            let splitter = diagram
                .splitters
                .iter()
                .filter(|splitter| splitter.x == origin.x)
                .find(|splitter| splitter.y > origin.y);

            match splitter {
                None => {}
                Some(splitter) => {
                    vertical_beams.push(VerticalBeam {
                        x: origin.x,
                        y_min: origin.y,
                        y_max: splitter.y,
                    });

                    let split_left = Coordinate {
                        x: splitter.x - 1,
                        y: splitter.y,
                    };
                    let split_right = Coordinate {
                        x: splitter.x + 1,
                        y: splitter.y,
                    };

                    beam_origins.push(split_left);
                    beam_origins.push(split_right);
                }
            }
        }
    }

    vertical_beams
        .iter()
        .map(|b| Coordinate { x: b.x, y: b.y_max })
        .collect::<Vec<_>>()
        .len() as u32
}

fn simulate_tachyon_particles(diagram: TachyonManifoldDiagram) -> u64 {
    let mut splitter_data: HashMap<Coordinate, Splitter> = diagram
        .splitters
        .iter()
        .map(|coordinate| Splitter {
            coordinate: *coordinate,
            amount_of_paths_to: 0,
            amount_of_paths_from: 0,
        })
        .fold(HashMap::new(), |mut map, splitter| {
            map.insert(splitter.coordinate, splitter);
            map
        });

    let mut splitters_at_x: HashMap<i32, Vec<Coordinate>> =
        diagram
            .splitters
            .iter()
            .fold(HashMap::new(), |mut map, coordinate| {
                let coordinates_at_x = map.entry(coordinate.x).or_insert(Vec::new());
                coordinates_at_x.push(*coordinate);
                coordinates_at_x.sort_by(|c1, c2| c1.x.cmp(&c2.x));
                map
            });

    let splitters_at_y: HashMap<i32, Vec<Coordinate>> =
        diagram
            .splitters
            .iter()
            .fold(HashMap::new(), |mut map, coordinate| {
                let coordinates_at_y = map.entry(coordinate.y).or_insert(Vec::new());
                coordinates_at_y.push(*coordinate);
                coordinates_at_y.sort_by(|c1, c2| c1.y.cmp(&c2.y));
                map
            });

    // Init first splitter
    let first_splitter = splitters_at_x
        .get(&diagram.beam_origin.x)
        .unwrap_or_else(|| panic!("Could not find splitters for X{}", diagram.beam_origin.x))
        .first()
        .unwrap_or_else(|| {
            panic!(
                "Could not find first splitter for X{}",
                diagram.beam_origin.x
            )
        });
    let mut first_splitter = splitter_data
        .get_mut(first_splitter)
        .unwrap_or_else(|| panic!("Could not find splitter data for {:?}", first_splitter))
        .clone();
    first_splitter.amount_of_paths_to += 1;
    splitter_data.insert(first_splitter.coordinate, first_splitter);

    let max_y = diagram
        .splitters
        .iter()
        .map(|c| c.y)
        .max()
        .expect("Could not find any splitter to find the max Y");
    for y in 0..max_y {
        let Some(splitters) = splitters_at_y.get(&y) else {
            continue;
        };

        for splitter in splitters {
            let mut splitter = splitter_data.get_mut(splitter).unwrap().clone();
            println!("Hit splitter @{:?}", splitter);

            let target_left = splitter.coordinate.x - 1;
            let next_left = splitters_at_x
                .entry(target_left)
                .or_default()
                .iter()
                .filter(|splitter| splitter.x == target_left)
                .find(|coordinate| coordinate.y > splitter.coordinate.y);
            if let Some(coordinate) = next_left {
                println!("Next left: {:?}", coordinate);

                splitter.amount_of_paths_from += 1;
                let mut splitter_at_left = splitter_data.get(coordinate).unwrap().clone();
                splitter_at_left.amount_of_paths_to += splitter.amount_of_paths_to;
                splitter_data.insert(splitter_at_left.coordinate, splitter_at_left);
            }

            let target_right = splitter.coordinate.x + 1;
            let next_right = splitters_at_x
                .entry(target_right)
                .or_default()
                .iter()
                .filter(|splitter| splitter.x == target_right)
                .find(|coordinate| coordinate.y > splitter.coordinate.y);
            if let Some(coordinate) = next_right {
                println!("Next right: {:?}", coordinate);

                splitter.amount_of_paths_from += 1;
                let mut splitter_at_right = splitter_data.get(coordinate).unwrap().clone();
                splitter_at_right.amount_of_paths_to += splitter.amount_of_paths_to;
                splitter_data.insert(splitter_at_right.coordinate, splitter_at_right);
            }

            splitter_data.insert(splitter.coordinate, splitter);
        }
    }

    let direct_endpoints: u64 = splitter_data
        .values()
        .filter(|splitter| splitter.amount_of_paths_from == 0)
        .map(|splitter| splitter.amount_of_paths_to * 2)
        .sum();

    let indirect_endpoints: u64 = splitter_data
        .values()
        .filter(|splitter| splitter.amount_of_paths_from == 1)
        .map(|splitter| splitter.amount_of_paths_to)
        .sum();

    direct_endpoints + indirect_endpoints
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

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Splitter {
    coordinate: Coordinate,
    amount_of_paths_to: u64,
    amount_of_paths_from: u64,
}

impl VerticalBeam {
    fn intersects(&self, coordinate: Coordinate) -> bool {
        if self.x != coordinate.x {
            return false;
        }

        coordinate.y >= self.y_min && coordinate.y <= self.y_max
    }
}

struct TachyonManifoldDiagram {
    beam_origin: Coordinate,
    splitters: Vec<Coordinate>,
}

fn parse_manifold_diagram(input: &str) -> TachyonManifoldDiagram {
    let input = input.replace('\r', "");
    let lines = input.split('\n');

    let mut beam_origin: Option<Coordinate> = None;
    let mut splitters: Vec<Coordinate> = Vec::new();
    for (line_index, line_text) in lines.enumerate() {
        for (character_index, character) in line_text.chars().enumerate() {
            match character {
                'S' => {
                    beam_origin = Some(Coordinate {
                        x: character_index as i32,
                        y: line_index as i32,
                    })
                }
                '^' => {
                    let position = Coordinate {
                        x: character_index as i32,
                        y: line_index as i32,
                    };
                    splitters.push(position);
                }
                '.' => {}
                other => panic!(
                    "Parsed unexpected character '{other}' at line {line_index}, char {character_index}"
                ),
            }
        }
    }

    let beam_origin = beam_origin.expect("Did not find any beam origin in the input");

    TachyonManifoldDiagram {
        beam_origin,
        splitters,
    }
}

#[test]
fn test_run_example_part_1() {
    let path = "./puzzle-inputs/day-7-example.txt";
    let example_data =
        read_to_string(path).unwrap_or_else(|err| panic!("Failed to read file {path}: {err}"));
    assert_eq!(run_part_1(example_data.as_str()), 21);
}

#[test]
fn test_run_example_part_2() {
    let path = "./puzzle-inputs/day-7-example.txt";
    let example_data =
        read_to_string(path).unwrap_or_else(|err| panic!("Failed to read file {path}: {err}"));
    assert_eq!(run_part_2(example_data.as_str()), 40);
}

#[test]
fn test_shoot_beam_with_one_splitter() {
    let mut splitters = Vec::new();
    splitters.push(Coordinate { x: 0, y: 1 });

    let diagram = TachyonManifoldDiagram {
        beam_origin: Coordinate { x: 0, y: 0 },
        splitters,
    };

    let splits = shoot_beam(diagram);

    assert_eq!(splits, 1);
}

#[test]
fn test_shoot_beam_with_two_splitters() {
    let mut splitters = Vec::new();
    splitters.push(Coordinate { x: 0, y: 1 });
    splitters.push(Coordinate { x: 1, y: 3 });

    let diagram = TachyonManifoldDiagram {
        beam_origin: Coordinate { x: 0, y: 0 },
        splitters,
    };

    let splits = shoot_beam(diagram);

    assert_eq!(splits, 2);
}

#[test]
fn test_shoot_beam_with_two_splitters_that_share_a_target() {
    let mut splitters = Vec::new();
    splitters.push(Coordinate { x: 1, y: 1 }); // First hit
    splitters.push(Coordinate { x: 0, y: 3 });
    splitters.push(Coordinate { x: 2, y: 3 });
    splitters.push(Coordinate { x: 10, y: 3 }); // Should not hit

    let diagram = TachyonManifoldDiagram {
        beam_origin: Coordinate { x: 1, y: 0 },
        splitters,
    };

    let splits = shoot_beam(diagram);

    assert_eq!(splits, 3);
}

#[test]
fn test_simulate_particle_with_two_splitters() {
    let mut splitters = Vec::new();
    //..S..
    //..^..
    //...^.
    splitters.push(Coordinate { x: 0, y: 1 });
    splitters.push(Coordinate { x: 1, y: 3 });

    let diagram = TachyonManifoldDiagram {
        beam_origin: Coordinate { x: 0, y: 0 },
        splitters,
    };

    let splits = simulate_tachyon_particles(diagram);

    assert_eq!(splits, 3);
}

#[test]
fn test_simulate_particle_that_hits_the_same_splitter_twice() {
    let mut splitters = Vec::new();
    //..S..
    //..^..
    //.^.^.
    //..^..
    //.....
    splitters.push(Coordinate { x: 1, y: 1 });
    splitters.push(Coordinate { x: 0, y: 2 });
    splitters.push(Coordinate { x: 2, y: 2 });

    splitters.push(Coordinate { x: 1, y: 3 }); // Target Splitter

    let diagram = TachyonManifoldDiagram {
        beam_origin: Coordinate { x: 1, y: 0 },
        splitters,
    };

    let hits = simulate_tachyon_particles(diagram);

    assert_eq!(hits, 6);
}

#[test]
fn test_simulate_tachyon_particle() {
    let mut splitters = Vec::new();
    //..S..
    //..^..
    //.^.^.
    //..^..
    //...^.
    splitters.push(Coordinate { x: 1, y: 1 });
    splitters.push(Coordinate { x: 0, y: 2 });
    splitters.push(Coordinate { x: 2, y: 2 });
    splitters.push(Coordinate { x: 1, y: 3 });

    splitters.push(Coordinate { x: 2, y: 4 }); // Target Splitter

    let diagram = TachyonManifoldDiagram {
        beam_origin: Coordinate { x: 1, y: 0 },
        splitters,
    };

    let hits = simulate_tachyon_particles(diagram);

    assert_eq!(hits, 8);
}
