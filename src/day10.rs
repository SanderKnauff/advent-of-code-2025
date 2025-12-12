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

    time("Day 10, Part 2 Example", || {
        run_part_2(example_data.as_str());
    });
    time("Day 10, Part 2 Puzzle", || {
        run_part_2(puzzle_data.as_str());
    });
}

fn run_part_1(input: &str) {
    let input = input.replace('\r', "");

    let machines: Vec<Machine> = parse_machines(input.as_str());

    machines.iter().for_each(|m| println!("{m}"));
}

fn run_part_2(_input: &str) {
    todo!()
}

struct Machine {
    wanted_indicators: IndicatorLights,
    buttons: Vec<Button>,
    joltages: Vec<u16>
}

impl Machine {
    fn is_ready(&self, current_indicators: u16) -> bool {
        current_indicators == self.wanted_indicators.lights
    }

    fn find_shortest_way_to_target(&self) -> u8 {
        let length: u8 = 0;

        length
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} {:?}", self.wanted_indicators, self.buttons))
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
    fn toggle(&self, current_indicators: u16) -> u16 {
        current_indicators.bitxor(self.toggles)
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
    // let joltage_string = &input[joltage_index..];

    let indicators = parse_indicators(indicator_string);
    let buttons = parse_buttons(button_string);

    Machine {
        wanted_indicators: indicators,
        buttons,
        joltages: Default::default()
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

#[test]
fn test_toggle_machine() {
    let button = parse_button("(3)");
    assert_eq!(button.toggle(0), 8);

    let button = parse_button("(1,3)");
    assert_eq!(button.toggle(0), 10);

    let button = parse_button("(2)");
    assert_eq!(button.toggle(0), 4);

    let button = parse_button("(2,3)");
    assert_eq!(button.toggle(0), 12);

    let button = parse_button("(0,2)");
    assert_eq!(button.toggle(0), 5);

    let button = parse_button("(0,1)");
    assert_eq!(button.toggle(0), 3);
}
