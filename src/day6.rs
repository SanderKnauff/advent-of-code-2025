use crate::stopwatch::time;
use std::fs::read_to_string;
use std::str::FromStr;

pub fn run() {
    let example_data = read_to_string("./puzzle-inputs/day-6-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-6-example.txt", err
        )
    });
    let puzzle_data = read_to_string("./puzzle-inputs/day-6-input.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-6-input.txt", err
        )
    });

    time("Day 6, Part 1 Example", || {
        run_part_1(example_data.as_str());
    });
    time("Day 6, Part 1 Puzzle", || {
        run_part_1(puzzle_data.as_str());
    });

    time("Day 6, Part 2 Example", || {
        run_part_2(example_data.as_str());
    });
    time("Day 6, Part 2 Puzzle", || {
        run_part_2(puzzle_data.as_str());
    });
}

fn run_part_1(input: &str) -> u64 {
    let input = input.replace('\r', "");
    let split_numbers: Vec<&str> = input.split('\n').collect();

    let number_rows = &split_numbers[0..split_numbers.len() - 1];
    let operand_row = &split_numbers[split_numbers.len() - 1..split_numbers.len()][0];

    let worksheet_columns: Vec<WorksheetColumn> = parse_worklist_columns(number_rows, operand_row);

    let result = worksheet_columns
        .iter()
        .map(WorksheetColumn::calculate_horizontal)
        .sum();

    println!("The sum of all worksheet items is {result}");

    result
}

fn run_part_2(input: &str) -> u64 {
    let input = input.replace('\r', "");
    let split_numbers: Vec<&str> = input.split('\n').collect();

    let number_rows = &split_numbers[0..split_numbers.len() - 1];
    let operand_row = &split_numbers[split_numbers.len() - 1..split_numbers.len()][0];

    let worksheet_columns: Vec<WorksheetColumn> = parse_worklist_columns(number_rows, operand_row);

    let result = worksheet_columns
        .iter()
        .map(WorksheetColumn::calculate_vertical)
        .sum();

    println!("The sum of all worksheet items is {result}");

    result
}

fn parse_worklist_columns(number_rows: &[&str], operand_row: &str) -> Vec<WorksheetColumn> {
    let mut worksheets: Vec<WorksheetColumn> = Vec::new();

    let number_rows: Vec<Vec<char>> = number_rows
        .iter()
        .map(|row| row.chars().collect())
        .collect();
    let mut operand_iterator = operand_row.split_whitespace();
    let mut buffer: Vec<String> = vec![String::new(); number_rows.len()];

    let total_row_length = number_rows.iter().map(Vec::len).max().unwrap_or(0);
    for i in 0..total_row_length {
        for (index, row) in number_rows.clone().iter().enumerate() {
            let char_to_push = if i < row.len() { row[i] } else { ' ' };
            buffer[index].push(char_to_push);
        }

        let is_column_divider = buffer.iter().all(|str| str.ends_with(' '));
        if is_column_divider || i >= total_row_length - 1 {
            let operand = match operand_iterator.next() {
                Some("+") => Operand::Addition,
                Some("*") => Operand::Multiply,
                Some(value) => panic!("Failed reading operand '{value}'"),
                None => panic!("Expected an operand, but found none"),
            };

            let mut worksheet = WorksheetColumn {
                number_texts: Vec::new(),
                operand,
            };

            for str in &buffer {
                let range_end = if is_column_divider {
                    str.len() - 1
                } else {
                    str.len()
                };
                worksheet.number_texts.push(str[0..range_end].to_owned());
            }

            buffer = vec![String::new(); number_rows.len()];

            worksheets.push(worksheet);
        }
    }

    worksheets
}

enum Operand {
    Addition,
    Multiply,
}

impl Operand {
    fn apply(&self, numbers: Vec<u64>) -> u64 {
        match self {
            Operand::Addition => numbers.iter().sum(),
            Operand::Multiply => numbers.iter().product(),
        }
    }
}

struct WorksheetColumn {
    number_texts: Vec<String>,
    operand: Operand,
}

impl WorksheetColumn {
    fn calculate_horizontal(&self) -> u64 {
        let numbers = self
            .number_texts
            .iter()
            .map(|number_text| number_text.trim())
            .map(|number_text| {
                u64::from_str(number_text)
                    .unwrap_or_else(|e| panic!("Failed to parse {number_text}: {e}"))
            })
            .collect();

        self.operand.apply(numbers)
    }

    // 123 328  51 64
    //  45 64  387 23
    //   6 98  215 314
    // *   +   *   +
    //
    // 4 + 431 + 621
    // 175 * 581 * 32
    // 8 + 248 + 369
    // 356 * 24 * 1
    fn calculate_vertical(&self) -> u64 {
        let mut pivoted_texts = vec![String::new(); self.number_texts[0].len()];
        for text in &self.number_texts {
            for (index, char) in text.chars().rev().enumerate() {
                if char != ' ' {
                    pivoted_texts[index].push(char);
                }
            }
        }

        let numbers = pivoted_texts
            .iter()
            .map(|number_text| number_text.trim())
            .map(|number_text| {
                u64::from_str(number_text)
                    .unwrap_or_else(|e| panic!("Failed to parse {number_text}: {e}"))
            })
            .collect();

        self.operand.apply(numbers)
    }
}

#[test]
fn test_run_example_part_1() {
    let example_data = read_to_string("./puzzle-inputs/day-6-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-6-example.txt", err
        )
    });
    assert_eq!(run_part_1(example_data.as_str()), 4277556);
}

#[test]
fn test_run_example_part_2() {
    let example_data = read_to_string("./puzzle-inputs/day-6-example.txt").unwrap_or_else(|err| {
        panic!(
            "Failed to read file {}: {}",
            "./puzzle-inputs/day-6-example.txt", err
        )
    });
    assert_eq!(run_part_2(example_data.as_str()), 3263827);
}
