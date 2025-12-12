use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::fs::read_to_string;
use std::ops::{Add, BitAnd, BitXor};
use std::str::FromStr;
use crate::stopwatch::time;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-10-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-10-example.txt", err
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-10-input.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-10-input.txt", err
        )
    });

    time("Day 10, Part 1 Example", || {
        run_part_1(example_data.as_str());
    });
    time("Day 10, Part 1 Puzzle", || {
        run_part_1(puzzle_data.as_str());
    });

    // TODO: Solve part 2 efficiently. According to hints, this problem is related to constraint solving.
    time("Day 10, Part 2 Example", || {
        run_part_2(example_data.as_str());
    });
    time("Day 10, Part 2 Puzzle", || {
        run_part_2(puzzle_data.as_str());
    });
}

fn run_part_1(input: &str) -> u16 {
    let input = input.replace('\r', "");

    let machines: Vec<Machine> = parse_machines(input.as_str());

    let minimal_button_presses = machines.iter()
        .map(|machine| {
            let buttons = machine.find_least_amount_of_buttons_to_enable_machine();

            buttons.len() as u16
        })
        .sum();

    println!("It takes a minimum of {minimal_button_presses} presses to enable all lights on all machines.");

    minimal_button_presses
}

fn run_part_2(input: &str) -> u16 {
    let input = input.replace('\r', "");

    let machines: Vec<Machine> = parse_machines(input.as_str());

    let minimal_button_presses: u16 = machines.iter()
        .map(|machine| {
            let buttons = machine.find_least_amount_of_buttons_to_configure_machine();

            println!("Took {} button presses for machine", buttons.len());

            buttons.len() as u16
        })
        .sum();

    println!("It takes a minimum of {minimal_button_presses} presses to enable all lights on all machines.");

    minimal_button_presses
}

struct Machine {
    wanted_indicators: IndicatorLights,
    buttons: Vec<Button>,
    joltages: Vec<u16>
}

impl Machine {
    fn find_least_amount_of_buttons_to_enable_machine(&self) -> Vec<Button> {
        let mut nodes: HashMap<u16, Vec<Button>> = Default::default();
        nodes.insert(0, vec![]);

        let mut depth: Option<Vec<Button>> = None;

        'search_loop:
        for _ in 0..1000 { // Iterate only 1000 times as failsafe.
            for (state, buttons) in nodes.clone().iter() {
                for button in &self.buttons {
                    let new_state = button.toggle_indicators(*state);

                    let buttons = buttons.clone();

                    if nodes.contains_key(&new_state) {
                        continue;
                    }

                    let node_for_state = nodes.entry(new_state)
                         .or_insert_with(|| buttons);

                    node_for_state.push(button.clone());

                    if new_state == self.wanted_indicators.lights {
                        depth = Some(node_for_state.clone());
                        break 'search_loop;
                    }
                }
            }
        }

        depth.expect("Failed to find shortest way to target after 1000 iterations")
    }

    fn find_least_amount_of_buttons_to_configure_machine(&self) -> Vec<Button> {
        let mut nodes: HashMap<Vec<u16>, Vec<Button>> = Default::default();
        nodes.insert(vec![0; self.joltages.len()], vec![]);

        let mut depth: Option<Vec<Button>> = None;

        'search_loop:
        for _ in 0..1000 { // Iterate only 1000 times as failsafe.
            for (state, buttons) in nodes.clone().iter() {
                for button in &self.buttons {
                    let new_state: Vec<u16> = button.toggle_joltages(state);

                    let buttons = buttons.clone();

                    if nodes.contains_key(&new_state) {
                        continue;
                    }

                    let node_for_state = nodes.entry(new_state.clone())
                         .or_insert_with(|| buttons);

                    node_for_state.push(button.clone());

                    if self.compare_joltages(&new_state) {
                        depth = Some(node_for_state.clone());
                        break 'search_loop;
                    }
                }
            }
        }

        depth.expect("Failed to find shortest way to target after 1000 iterations")
    }

    fn compare_joltages(&self, joltages: &Vec<u16>) -> bool {
        for (index, joltage) in self.joltages.iter().enumerate() {
            if joltage != &joltages[index] {
                return false;
            }
        }

        true
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let joined_buttons = self.buttons.iter()
            .map(|button| button.to_string())
            .fold(String::new(), |acc, s| acc + " " + &s);

        f.write_fmt(format_args!("{} {joined_buttons}", self.wanted_indicators))
    }
}

struct IndicatorLights {
    amount_of_lights: u8,
    lights: u16
}

impl Display for IndicatorLights {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('[')?;

        for i in 0..self.amount_of_lights {
            let light_to_check = 1 << i;
            let expected = self.lights.bitand(light_to_check);
            // println!("Light to check: {light_to_check}, Expected: {expected}. All: {}", self.lights);

            if expected == light_to_check {
                f.write_char('#')?
            } else {
                f.write_char('.')?
            }
        }

        f.write_char(']')
    }
}

#[derive(Debug, Copy, Clone)]
struct Button {
    toggles: u16
}

impl Button {
    fn toggle_indicators(&self, current_indicators: u16) -> u16 {
        current_indicators.bitxor(self.toggles)
    }

    fn toggle_joltages(&self, current_joltages: &[u16]) -> Vec<u16> {
        let mut new_joltages = Vec::from(current_joltages);

        for i in 0..16 {
            let bit_to_check = 1 << i;
            let expected = self.toggles.bitand(bit_to_check);

            if expected == bit_to_check {
                new_joltages[i] += 1;
            }
        }

        new_joltages
    }
}

impl Display for Button {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char('(')?;

        for i in 0..16 {
            let bit_to_check = 1 << i;
            let expected = self.toggles.bitand(bit_to_check);
            // println!("Light to check: {light_to_check}, Expected: {expected}. All: {}", self.lights);

            if expected == bit_to_check {
            f.write_str(format!("{}", i).as_str())?;
            f.write_char(',')?;
            }
        }

        f.write_char(')')
    }
}

fn parse_machines(input: &str) -> Vec<Machine> {
    input.lines()
        .map(parse_machine)
        .collect()
}

fn parse_machine(input: &str) -> Machine {
    let button_index = input.find('(').unwrap_or_else(|| panic!("Could not find button section start '(' in {input}"));
    let joltage_index = input.find('{').unwrap_or_else(|| panic!("Could not find joltage section start '{{' in {input}"));

    let indicator_string = &input[..button_index];
    let button_string = &input[button_index..joltage_index];
    let joltage_string = &input[joltage_index..];

    let indicators = parse_indicators(indicator_string);
    let buttons = parse_buttons(button_string);
    let joltages = parse_joltages(joltage_string);

    Machine {
        wanted_indicators: indicators,
        buttons,
        joltages,
    }
}

fn parse_indicators(input: &str) -> IndicatorLights {
    let input = input.replace('[', "");
    let input = input.replace(']', "");
    let input = input.trim();

    let mut indicators: u16 = 0;
    let mut amount = 0;
    for (index, char) in input.chars().enumerate() {
        match char {
            '.' => {}
            '#' => {
                let bits = 1 << index;
                indicators = indicators.add(bits)
            }
            other => panic!("Unexpected character '{other}' in indicator input")
        }
        amount += 1;
    }

    IndicatorLights {
        amount_of_lights: amount,
        lights: indicators,
    }
}

fn parse_buttons(input: &str) -> Vec<Button> {
    let input = input.trim();

    input.split(' ')
        .map(parse_button)
        .collect()
}

fn parse_button(input: &str) -> Button {
    let input = input.replace('(', "");
    let input = input.replace(')', "");

    let toggles = input.split(',')
        .map(u8::from_str)
        .map(|result| result.unwrap_or_else(|err| panic!("Failed parsing to u8: {err}")))
        .map(|index| 1 << index)
        .sum();

    Button {
        toggles,
    }
}

fn parse_joltages(input: &str) -> Vec<u16> {
    let input = input.replace('{', "");
    let input = input.replace('}', "");
    let input = input.trim();

    input.split(',')
        .map(u16::from_str)
        .map(|result| result.unwrap_or_else(|err| panic!("Failed parsing to u8: {err}")))
        .collect()
}

#[test]
fn test_toggle_machine() {
    let button = parse_button("(3)");
    assert_eq!(button.toggle_indicators(0), 8);

    let button = parse_button("(1,3)");
    assert_eq!(button.toggle_indicators(0), 10);

    let button = parse_button("(2)");
    assert_eq!(button.toggle_indicators(0), 4);

    let button = parse_button("(2,3)");
    assert_eq!(button.toggle_indicators(0), 12);

    let button = parse_button("(0,2)");
    assert_eq!(button.toggle_indicators(0), 5);

    let button = parse_button("(0,1)");
    assert_eq!(button.toggle_indicators(0), 3);
}

#[test]
fn test_find_least_amount_of_buttons_to_enable_machines(){
    let machine = parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    assert_eq!(machine.find_least_amount_of_buttons_to_enable_machine().len(), 2);

    let machine = parse_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
    assert_eq!(machine.find_least_amount_of_buttons_to_enable_machine().len(), 3);

    let machine = parse_machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
    assert_eq!(machine.find_least_amount_of_buttons_to_enable_machine().len(), 2);
}

#[test]
fn test_example_two(){
    let machine = parse_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
    assert_eq!(machine.find_least_amount_of_buttons_to_enable_machine().len(), 3);
}

#[test]
fn test_example_manual(){
    let machine = parse_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
    let mut state = 0;
    state = machine.buttons[2].toggle_indicators(state);
    state = machine.buttons[3].toggle_indicators(state);
    state = machine.buttons[4].toggle_indicators(state);


    assert_eq!(state, machine.wanted_indicators.lights);
}

#[test]
fn test_find_least_amount_of_buttons_to_configure_machines(){
    let machine = parse_machine("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
    assert_eq!(machine.find_least_amount_of_buttons_to_configure_machine().len(), 10);

    let machine = parse_machine("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
    assert_eq!(machine.find_least_amount_of_buttons_to_configure_machine().len(), 12);

    let machine = parse_machine("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
    assert_eq!(machine.find_least_amount_of_buttons_to_configure_machine().len(), 11);
}
