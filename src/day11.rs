use crate::stopwatch::time;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::hash::Hash;

pub fn run() {
    let example_data_1 = read_to_string("./puzzle-inputs/day-11-example-1.txt").unwrap_or_else(|_| {
        panic!(
            "Failed to read file {}",
            "./puzzle-inputs/day-11-example-1.txt"
        )
    });
    let example_data_2 = read_to_string("./puzzle-inputs/day-11-example-2.txt").unwrap_or_else(|_| {
        panic!(
            "Failed to read file {}",
            "./puzzle-inputs/day-11-example-1.txt"
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-11-input.txt")
        .unwrap_or_else(|_| panic!("Failed to read file {}", "./puzzle-inputs/day-11-input.txt"));

    time("Day 11, Part 1 Example", || {
        run_part_1(example_data_1.as_str());
    });
    time("Day 11, Part 1 Puzzle", || {
        run_part_1(puzzle_data.as_str());
    });

    time("Day 11, Part 2 Example", || {
        run_part_2(example_data_2.as_str());
    });
    time("Day 11, Part 2 Puzzle", || {
        run_part_2(puzzle_data.as_str());
    });
}

fn run_part_1(input: &str) -> u64 {
    let graph = parse_graph(input);

    let result = graph.count_paths_from_to(Box::from("you"), Box::from("out"));

    println!("The number of paths from `you` to `out` is {}", result);

    result
}

fn run_part_2(input: &str) -> u64 {
    let graph = parse_graph(input);

    let svr_to_fft = graph.find_paths_through(Box::from("svr"), Box::from("fft"));
    let fft_to_dac = graph.find_paths_through(Box::from("fft"), Box::from("dac"));
    let dac_to_out = graph.find_paths_through(Box::from("dac"), Box::from("out"));

    let svr_to_dac = graph.find_paths_through(Box::from("svr"), Box::from("dac"));
    let dac_to_fft = graph.find_paths_through(Box::from("dac"), Box::from("fft"));
    let fft_to_out = graph.find_paths_through(Box::from("fft"), Box::from("out"));

    let result = (svr_to_fft * fft_to_dac * dac_to_out) + (svr_to_dac * dac_to_fft * fft_to_out);

    println!("The number of paths from `svr` to `out` through `fft` and `dac` is {}", result);

    result
}

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

    fn find_paths_through(&self, from: Box<str>, to: Box<str>) -> u64 {
        self.dfs(&from, &to, &mut HashSet::new(), &mut HashMap::new())
    }

    fn dfs(&self, from: &Box<str>, to: &Box<str>, visited: &mut HashSet<Box<str>>, memo: &mut HashMap<Box<str>, u64>) -> u64 {
        if memo.contains_key(from) {
            return memo.get(from).unwrap_or(&0).clone();
        }


        if from == to {
            return 1
        }

        let from = self.nodes.get(from).unwrap_or_else(|| panic!("Could not find node with identity '{}'", from));

        visited.insert(from.identity.clone());

        let mut visited_sorted = visited.iter().cloned()
            .collect::<Vec<_>>();
        visited_sorted.sort();

        let mut paths = 0;
        for child in &from.destinations {
            if visited.contains(child) {
                continue;
            }
            paths += self.dfs(child, to, visited, memo);
        }

        visited.remove(&from.identity.clone());
        memo.insert(from.identity.clone(), paths);

        paths
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
    let example_data = read_to_string("./puzzle-inputs/day-11-example-1.txt").unwrap_or_else(|_| {
        panic!(
            "Failed to read file {}",
            "./puzzle-inputs/day-11-example-1.txt"
        )
    });

    let result = run_part_1(example_data.as_str());

    assert_eq!(result, 5);
}

#[test]
fn test_example_2() {
    let example_data = read_to_string("./puzzle-inputs/day-11-example-2.txt").unwrap_or_else(|_| {
        panic!(
            "Failed to read file {}",
            "./puzzle-inputs/day-11-example-2.txt"
        )
    });

    let result = run_part_2(example_data.as_str());

    assert_eq!(result, 2);
}
