use crate::stopwatch::time;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::Hash;

pub fn run() {
    let example_data = read_to_string("../puzzle-inputs/day-11-example-1.txt").unwrap_or_else(|_| {
        panic!(
            "Failed to read file {}",
            "./puzzle-inputs/day-11-example.txt"
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-11-input.txt")
        .unwrap_or_else(|_| panic!("Failed to read file {}", "./puzzle-inputs/day-11-input.txt"));

    time("Day 11, Part 1 Example", || {
        run_part_1(example_data.as_str());
    });
    time("Day 11, Part 1 Puzzle", || {
        run_part_1(puzzle_data.as_str());
    });

    time("Day 11, Part 2 Example", || {
        run_part_2(example_data.as_str())
    });
    time("Day 11, Part 2 Puzzle", || run_part_2(puzzle_data.as_str()));
}

fn run_part_1(input: &str) -> u64 {
    let graph = parse_graph(input);

    let result = graph.count_paths_from_to(Box::from("you"), Box::from("out"));

    println!("The number of paths from `you` to `out` is {}", result);

    result
}

fn run_part_2(_input: &str) {}

struct Graph {
    nodes: HashMap<Box<str>, Node>,
}

impl Graph {
    fn count_paths_from_to(&self, from: Box<str>, to: Box<str>) -> u64 {
        let mut nodes_to_visit: Vec<Box<str>> = Vec::new();
        let mut paths_from_root: HashMap<Box<str>, u64> = HashMap::new();

        let end_node = self
            .nodes
            .get(&to)
            .unwrap_or_else(|| panic!("Could not find node with identity '{}'", to));

        nodes_to_visit.push(from.clone());

        while nodes_to_visit.len() > 0 {

            let node_to_visit = nodes_to_visit.remove(nodes_to_visit.len() - 1);

            let node_to_visit = self.nodes.get(&node_to_visit).unwrap_or_else(|| {
                panic!("Could not find node with identity '{}'", node_to_visit)
            });

            let current_paths = paths_from_root.get(&node_to_visit.identity).unwrap_or(&0);
            paths_from_root.insert(
                node_to_visit.identity.clone(),
                current_paths + 1,
            );

            for next_node in &node_to_visit.destinations {
                let next_node = self.nodes.get(next_node).unwrap_or_else(|| {
                    panic!("Could not find node with identity '{}'", next_node)
                });
                nodes_to_visit.push(next_node.identity.clone());
            }
        }

        paths_from_root
            .get(&end_node.identity)
            .unwrap_or_else(|| panic!("Could not find any path from {} to {}", from, to))
            .clone()
    }
}

struct Node {
    identity: Box<str>,
    destinations: Vec<Box<str>>,
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.identity.hash(state);
    }
}

fn parse_graph(input: &str) -> Graph {
    let mut nodes: HashMap<Box<str>, Node> = HashMap::new();

    nodes.insert(
        Box::from("out"),
        Node {
            identity: Box::from("out"),
            destinations: vec![],
        },
    );

    for line in input.lines() {
        let node = read_and_insert_node(line);
        nodes.insert(node.identity.clone(), node);
    }

    Graph { nodes }
}

fn read_and_insert_node(line: &str) -> Node {
    let mut split = line.split(':');
    let header = split.next().unwrap_or_else(|| {
        panic!("Invalid line. Could not find ':' and thus failed to parse header for '{line}'")
    });
    let values = split.next().unwrap_or_else(|| {
        panic!("Invalid line. Could not find ':' and thus failed to parse values for '{line}'")
    });

    let destinations: Vec<Box<str>> = values
        .trim()
        .split(' ')
        .map(|destination| destination.to_owned().into_boxed_str())
        .collect();

    Node {
        identity: Box::from(header),
        destinations,
    }
}

#[test]
fn test_example_1() {
    let example_data = read_to_string("../puzzle-inputs/day-11-example-1.txt").unwrap_or_else(|_| {
        panic!(
            "Failed to read file {}",
            "./puzzle-inputs/day-11-example.txt"
        )
    });

    let result = run_part_1(example_data.as_str());

    assert_eq!(result, 5);
}
