use clap::{Command, arg};

pub mod utils;

pub mod solutions {
    pub mod day1;
    pub mod day2;
}

use solutions::*;

pub const CHALLENGES: [&dyn Challenge; 2] = [
    &day1::Day1 {},
    &day2::Day2 {},
];

pub trait Challenge {
    #[allow(unused)]
    fn solve_part1(&self, input: &String) -> Option<i64> { None }
    #[allow(unused)]
    fn solve_part2(&self, input: &String) -> Option<i64> { None }
}

pub struct Test {
    pub name: &'static str,
    pub file: &'static str,
    pub callback: Box<dyn Fn(&String) -> Option<i64>>,
}

impl Test {
    pub fn new(name: &'static str, file: &'static str, callback: Box<dyn Fn(&String) -> Option<i64>>) -> Self {
        Self {
            name,
            file,
            callback
        }
    }
}

#[derive(Clone, Copy)]
pub enum OutputLevel {
    None = 0,
    Error = 1,
    Warning = 2,
    Info = 3,
    Debug = 4,
}

pub static mut OUTPUT_LEVEL: OutputLevel = OutputLevel::Warning;

pub fn output(level: OutputLevel, output: String) {
    unsafe {
        // println!("{}, {}", level as u8, OUTPUT_LEVEL as u8);
        if level as u8 <= OUTPUT_LEVEL as u8 {
            println!("\t{output}");
        }
    }
}

pub fn command_parser() -> Command {
    Command::new("Advent of Code 2025")
        .arg(arg!(<DAY>))
        .arg(arg!(-d --disable [OPTIONS]...).num_args(1..))
}