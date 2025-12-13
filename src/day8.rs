use crate::stopwatch::time;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::Hash;
use std::rc::Rc;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-8-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-8-example.txt", err
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-8-input.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-8-input.txt", err
        )
    });

    time("Day 8, Part 1 Example", || {
        run_part_1(example_data.as_str(), 10);
    });
    time("Day 8, Part 1 Puzzle", || {
        run_part_1(puzzle_data.as_str(), 1000);
    });

    time("Day 8, Part 2 Example", || {
        run_part_2(example_data.as_str());
    });
    time("Day 8, Part 2 Puzzle", || {
        run_part_2(puzzle_data.as_str());
    });
}

fn run_part_1(input: &str, connections_to_make: u32) -> u32 {
    let junction_boxes = parse_junction_boxes(input);

    let distances: Vec<JunctionBoxDistance> = find_distances_between_points(&junction_boxes);

    let mut circuits: HashSet<Rc<Circuit>> = HashSet::new();
    let mut circuit_lookup: HashMap<JunctionBox, Rc<Circuit>> = HashMap::new();
    junction_boxes.iter().for_each(|point| {
        let circuit = Circuit {
            identity: point.clone(),
            junction_boxes: HashSet::from([point.clone()]),
        };
        let circuit = Rc::new(circuit);
        circuit_lookup.insert(point.clone(), circuit.clone());
        circuits.insert(circuit.clone());
    });

    let mut count = 0;
    for distance in &distances {
        if count > connections_to_make {
            break;
        }

        println!("[{count}]: Processing distance {:?}", distance);

        let one = circuit_lookup.get(&distance.first_box.clone().into()).unwrap_or_else(|| panic!("Distance {distance:?} has invalid first_box, circuit lookup does not have an entry for point {:?}", distance.first_box));
        let other = circuit_lookup.get(&distance.second_box.clone().into()).unwrap_or_else(|| panic!("Distance {distance:?} has invalid second_box, circuit lookup does not have an entry for point {:?}", distance.second_box));

        let is_same_circuit = one == other;
        count += 1;
        if is_same_circuit {
            println!(
                "Skipping distance {distance:?} because {one:?} and {other:?} are part of the same circuit"
            );
            continue;
        }

        merge_circuits(
            &mut circuits,
            &mut circuit_lookup,
            distance.first_box.clone(),
            distance.second_box.clone(),
        );
    }

    let circuits: Vec<&Rc<Circuit>> = circuits.iter().collect();

    let mut circuits: Vec<usize> = circuits
        .iter()
        .map(|circuit| circuit.junction_boxes.len().clone())
        .collect();
    circuits.sort_by(|a, b| b.cmp(a));
    let size: usize = circuits[..=2].iter().product();

    println!("The product of the largest 10 circuits is {size}");

    size as u32
}

fn run_part_2(input: &str) -> u64 {
    let junction_boxes = parse_junction_boxes(input);

    let distances: Vec<JunctionBoxDistance> = find_distances_between_points(&junction_boxes);

    let mut circuits: HashSet<Rc<Circuit>> = HashSet::new();
    let mut circuit_lookup: HashMap<JunctionBox, Rc<Circuit>> = HashMap::new();
    junction_boxes.iter().for_each(|point| {
        let circuit = Circuit {
            identity: point.clone(),
            junction_boxes: HashSet::from([point.clone()]),
        };
        let circuit = Rc::new(circuit);
        circuit_lookup.insert(point.clone(), circuit.clone());
        circuits.insert(circuit.clone());
    });

    for distance in &distances {
        let one = circuit_lookup.get(&distance.first_box.clone().into()).unwrap_or_else(|| panic!("Distance {distance:?} has invalid first_box, circuit lookup does not have an entry for point {:?}", distance.first_box));
        let other = circuit_lookup.get(&distance.second_box.clone().into()).unwrap_or_else(|| panic!("Distance {distance:?} has invalid second_box, circuit lookup does not have an entry for point {:?}", distance.second_box));

        let is_same_circuit = one == other;
        if is_same_circuit {
            continue;
        }

        if circuits.len() == 2 {
            let result = distance.first_box.x * distance.second_box.x;
            println!(
                "The product of X coordinates of the two last two circuits to be connected is {result:?}"
            );
            return result;
        }

        merge_circuits(
            &mut circuits,
            &mut circuit_lookup,
            distance.first_box.clone(),
            distance.second_box.clone(),
        );
    }

    panic!("Did not connect last circuits!");
}

fn merge_circuits(
    circuits: &mut HashSet<Rc<Circuit>>,
    circuit_lookup: &mut HashMap<JunctionBox, Rc<Circuit>>,
    first_point: JunctionBox,
    second_point: JunctionBox,
) {
    let first_circuit = circuit_lookup.get(&first_point).unwrap_or_else(|| {
        panic!(
            "Circuit lookup does not have an entry for point {:?}",
            first_point
        )
    });
    let second_circuit = circuit_lookup.get(&second_point).unwrap_or_else(|| {
        panic!(
            "Circuit lookup does not have an entry for point {:?}",
            second_point
        )
    });

    let joined_circuit = Circuit {
        identity: first_circuit.identity.clone(),
        junction_boxes: first_circuit
            .junction_boxes
            .union(&second_circuit.junction_boxes)
            .cloned()
            .collect(),
    };
    let joined_circuit_ref = Rc::new(joined_circuit);

    circuits.remove(first_circuit);
    circuits.remove(second_circuit);

    circuits.insert(joined_circuit_ref.clone());

    for junction_box in joined_circuit_ref.junction_boxes.iter() {
        circuit_lookup.insert(junction_box.to_owned(), joined_circuit_ref.clone());
    }
}

fn find_distances_between_points(junction_boxes: &Vec<JunctionBox>) -> Vec<JunctionBoxDistance> {
    let mut distances: Vec<JunctionBoxDistance> = Vec::new();

    for (index, first_box) in junction_boxes.iter().enumerate() {
        for second_box in junction_boxes[index + 1..].iter() {
            if first_box == second_box {
                continue;
            }

            distances.push(JunctionBoxDistance {
                distance: first_box.distance_sq(&second_box) as u64,
                first_box: first_box.clone(),
                second_box: second_box.clone(),
            });
        }
    }

    distances.sort_by(|a, b| a.distance.cmp(&b.distance));
    distances
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn distance_sq(&self, other: &JunctionBox) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

#[derive(PartialEq, Eq, Debug)]
struct JunctionBoxDistance {
    distance: u64,
    first_box: JunctionBox,
    second_box: JunctionBox,
}

impl PartialOrd for JunctionBoxDistance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for JunctionBoxDistance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Circuit {
    identity: JunctionBox,
    junction_boxes: HashSet<JunctionBox>,
}

impl Hash for Circuit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identity.hash(state);
    }
}

fn parse_junction_boxes(input: &str) -> Vec<JunctionBox> {
    let input = input.replace('\r', "");

    let mut boxes: Vec<JunctionBox> = Vec::new();
    for box_text in input.lines() {
        let coords: Vec<f64> = box_text
            .split(',')
            .map(|coord| coord.parse::<f64>().unwrap())
            .collect();
        let junction_box = JunctionBox {
            x: coords[0] as u64,
            y: coords[1] as u64,
            z: coords[2] as u64,
        };
        boxes.push(junction_box);
    }

    boxes
}

#[test]
fn test_run_example_part_1() {
    let path = "./puzzle-inputs/day-8-example.txt";
    let example_data =
        read_to_string(path).unwrap_or_else(|err| panic!("Failed to read file {path}: {err}"));
    assert_eq!(run_part_1(example_data.as_str(), 9), 40);
}

#[test]
fn test_run_example_part_2() {
    let path = "./puzzle-inputs/day-8-example.txt";
    let example_data =
        read_to_string(path).unwrap_or_else(|err| panic!("Failed to read file {path}: {err}"));
    assert_eq!(run_part_2(example_data.as_str()), 25272);
}

#[test]
fn test_rc_multiple_maps() {
    let mut circuit_junction_box_lut: Vec<Rc<Circuit>> = Vec::new();
    let mut junction_box_circuit_lut: HashMap<JunctionBox, Rc<Circuit>> = HashMap::new();

    let junction_box = JunctionBox { x: 0, y: 0, z: 0 };

    let hashed_point: JunctionBox = junction_box.into();

    let circuit = Circuit {
        identity: hashed_point.clone(),
        junction_boxes: HashSet::from([hashed_point.clone()]),
    };

    let circuit_ref = Rc::new(circuit);
    circuit_junction_box_lut.push(circuit_ref.clone());
    junction_box_circuit_lut.insert(hashed_point.clone(), circuit_ref.clone());

    let c_from_lookup = junction_box_circuit_lut
        .get(&hashed_point)
        .unwrap()
        .as_ref()
        .to_owned();
    println!("{:?}", c_from_lookup.junction_boxes);
}
