#[allow(unused)]
use crate::{output, OutputLevel, Challenge};
use std::{collections::HashSet};

pub struct Day2;

impl Challenge for Day2 {
    fn solve_part1(&self, input: &String) -> Option<i64>  {
        let mut result: u64 = 0;
        let lines = input.split(",");
        for line in lines {
            let mut line = line.split("-").into_iter();
            let min = line.next().unwrap();
            let max = line.next().unwrap();

            let (starting_num, starting_digits) = if min.len() % 2 == 0 {
                (min.parse::<u64>().unwrap(), min.len())
            } else if max.len() % 2 == 0 {
                (power_of_10(min.len() as u8), max.len())
            } else {
                continue;
            };

            let exponent = power_of_10(starting_digits as u8 / 2);
            let mut first_half = starting_num as u64 / exponent;
            let min_value = min.parse::<u64>().unwrap();
            let max_value = max.parse::<u64>().unwrap();
            let max_first_half = max_value / exponent;

            while first_half <= max_first_half {
                let potential_invalid = first_half * exponent + first_half;
                if potential_invalid >= min_value && potential_invalid <= max_value {
                    result += potential_invalid;
                }
                    
                first_half += 1;
                if digits(first_half) != starting_digits as u8 / 2 {
                    break;
                }
            }
        }

        Some(result as i64)
    }
    fn solve_part2(&self, input: &String) -> Option<i64>  {
        let mut result: u64 = 0;

        let lines = input.split(",").map(|line| {
            let mut values = line.split("-");
            (
                values.next().unwrap().parse::<u64>().unwrap(),
                values.next().unwrap().parse::<u64>().unwrap()
            )
        });

        // Simplify by making each range only include the same number of digits
        let mut values: Vec<(u64, u64)> = Vec::new();
        for line in lines {
            if digits(line.0) != digits(line.1) {
                values.push((line.0, power_of_10(digits(line.0)) - 1));
                values.push((power_of_10(digits(line.0)), line.1))
            } else {
                values.push(line);
            }
        }

        for (min, max) in values {
            assert_eq!(digits(min), digits(max));
            let num_digits: u8 = digits(min);

            // Keep track of matches to avoid double-counting
            let mut invalids: HashSet<u64> = HashSet::new();
            for checker_length in 1..num_digits / 2 + 1 {
                let mut segment = first_n_digits(min, checker_length as u64);
                
                output(OutputLevel::Debug, format!("Checking {checker_length} for range {min}..{max}"));
                output(OutputLevel::Debug, format!("\tFirst checking {segment}"));
                
                loop {
                    let potential_invalid = repeat(segment, num_digits as u64 / checker_length as u64);
                    if potential_invalid >= min && potential_invalid <= max {
                        output(OutputLevel::Info, format!("\tFound an invalid: {potential_invalid}"));
                        invalids.insert(potential_invalid);
                    }
                    if potential_invalid >= max {
                        break;
                    }
                    segment += 1;
                }
            }
            result += invalids.iter().sum::<u64>();
        }


        Some(result as i64)
    }
}

fn power_of_10(power: u8) -> u64 {
    let mut result = 1;
    for _ in 0..power {
        result *= 10;
    }
    result
}

fn digits(num: u64) -> u8 {
    if num == 0 { 
        1
    } else {
        f64::log10(num as f64).floor() as u8 + 1
    }
}

fn first_n_digits(num: u64, n: u64) -> u64 {
    num / power_of_10(digits(num) - n as u8)
}

fn repeat(num: u64, n: u64) -> u64 {
    let mut result = 0;
    for i in 0..n {
        result += num * power_of_10(i as u8 * digits(num));
    }
    result
}