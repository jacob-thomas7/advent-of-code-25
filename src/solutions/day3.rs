#[allow(unused)]
use crate::{output, OutputLevel, Challenge};

pub struct Day3;

impl Challenge for Day3 {
    fn solve_part1(&self, input: &String) -> Option<i64> {
        Some(max_joltage(input, 2))
    }
    fn solve_part2(&self, input: &String) -> Option<i64> {
        Some(max_joltage(input, 12))
    }
}

fn max_joltage(input: &String, batteries_per_bank: u8) -> i64 {
    let mut result = 0;
    for line in input.lines() {
        output(OutputLevel::Info, format!("Parsing line: {line}"));
        let mut battery_indexes: Vec<usize> = Vec::new();

        for battery_num in 0..batteries_per_bank {
            let mut largest_value = '0';
            let mut largest_index = 0;

            let start_index = if battery_indexes.len() > 0 {
                battery_indexes[battery_indexes.len() - 1]
            } else {
                0
            };

            for (i, c) in line.chars().enumerate().skip(start_index) {
                if c > largest_value && !battery_indexes.contains(&i) && i <= line.len() - batteries_per_bank as usize + battery_num as usize {
                    largest_value = c;
                    largest_index = i;
                }
            }
            battery_indexes.push(largest_index);
        }
        let mut digits = String::new();
        for battery in battery_indexes {
            digits.push(line.chars().nth(battery).unwrap());
        }
        output(OutputLevel::Info, format!("\tResult: {digits}"));
        result += digits.parse::<i64>().unwrap();
    }
    result
}